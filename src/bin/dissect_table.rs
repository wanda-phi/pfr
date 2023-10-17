use clap::Parser;
use ndarray::{s, Array2};
use pfr::assets::iff::Image;
use pfr::assets::table::{physics::Layer, Assets};
use pfr::config::TableId;
use std::io::BufWriter;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Args {
    input_dir: PathBuf,
    table: u32,
    output_dir: PathBuf,
}

fn save_png(image: &Image, output_dir: impl AsRef<Path>, name: &str) -> std::io::Result<()> {
    let width = image.data.dim().0;
    let height = image.data.dim().1;
    let file = File::create(output_dir.as_ref().join(name))?;
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Eight);
    let mut cmap = vec![0; image.cmap.len() * 3];
    for i in 0..image.cmap.len() {
        cmap[i * 3] = image.cmap[i].0;
        cmap[i * 3 + 1] = image.cmap[i].1;
        cmap[i * 3 + 2] = image.cmap[i].2;
    }
    encoder.set_palette(&cmap);
    let mut writer = encoder.write_header()?;
    let mut data = vec![0; width * height];
    for y in 0..height {
        for x in 0..width {
            data[y * width + x] = image.data[(x, y)];
        }
    }
    writer.write_image_data(&data)?;
    writer.finish()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let (table, file) = match args.table {
        1 => (TableId::Table1, "TABLE1.PRG"),
        2 => (TableId::Table2, "TABLE2.PRG"),
        3 => (TableId::Table3, "TABLE3.PRG"),
        4 => (TableId::Table4, "TABLE4.PRG"),
        _ => panic!("oops weird table"),
    };
    let assets = Assets::load(args.input_dir.join(file), table)?;
    println!("DS: {ds:04x}", ds = assets.exe.ds);
    let mut main_board = assets.main_board.clone();

    for patch in assets.lights.values() {
        for (i, &color) in patch.colors.iter().enumerate() {
            main_board.cmap[patch.base_index as usize + i] = color
        }
    }

    save_png(&main_board, &args.output_dir, "main.png")?;

    save_png(
        &Image {
            data: assets.occmaps[Layer::Ground].clone(),
            cmap: vec![(0, 0, 0), (255, 255, 255)],
        },
        &args.output_dir,
        "occmap0.png",
    )?;
    save_png(
        &Image {
            data: assets.occmaps[Layer::Overhead].clone(),
            cmap: vec![(0, 0, 0), (255, 255, 255)],
        },
        &args.output_dir,
        "occmap1.png",
    )?;
    save_png(&assets.spring, &args.output_dir, "spring.png")?;
    save_png(&assets.ball, &args.output_dir, "ball.png")?;
    if let Some(ref tower) = assets.dm_tower {
        save_png(
            &Image {
                data: Array2::from_shape_fn((160, 167), |(x, y)| u8::from(tower[y][x])),
                cmap: vec![(0, 0, 0), (255, 255, 255)],
            },
            &args.output_dir,
            "tower.png",
        )?;
    }
    let physmap_pal = vec![
        (0, 0, 0),
        (0, 0, 64),
        (0, 64, 0),
        (64, 0, 0),
        (64, 64, 0),
        (64, 0, 64),
        (0, 64, 64),
        (64, 64, 64),
        (64, 0, 32),
        (64, 32, 0),
        (0, 64, 32),
        (32, 64, 0),
        (0, 32, 64),
        (32, 0, 64),
        (32, 32, 0),
        (0, 32, 32),
        (128, 128, 128),
        (128, 128, 255),
        (128, 255, 128),
        (128, 255, 255),
        (255, 128, 128),
        (255, 128, 255),
        (255, 255, 128),
        (255, 255, 255),
    ];
    fn physmap_map(x: &u8) -> u8 {
        if x & 3 == 0 {
            x >> 4
        } else {
            x & 0xf | 0x10
        }
    }
    for (img, name) in [
        (&assets.physmaps[Layer::Ground], "physmap0.png"),
        (&assets.physmaps[Layer::Overhead], "physmap1.png"),
    ] {
        save_png(
            &Image {
                data: img.map(physmap_map),
                cmap: physmap_pal.clone(),
            },
            &args.output_dir,
            name,
        )?;
    }
    for (idx, patch) in assets.physmap_patches.values().flatten().enumerate() {
        let mut img = assets.physmaps[patch.layer].clone();
        img.slice_mut(s![
            (patch.pos.0 as usize)..(patch.pos.0 as usize + patch.raised.dim().0),
            (patch.pos.1 as usize)..(patch.pos.1 as usize + patch.raised.dim().1),
        ])
        .assign(&patch.raised);
        let img = Image {
            data: img.map(physmap_map),
            cmap: physmap_pal.clone(),
        };
        save_png(&img, &args.output_dir, &format!("physmap_patch{idx}_r.png"))?;
        let mut img = assets.physmaps[patch.layer].clone();
        img.slice_mut(s![
            (patch.pos.0 as usize)..(patch.pos.0 as usize + patch.dropped.dim().0),
            (patch.pos.1 as usize)..(patch.pos.1 as usize + patch.dropped.dim().1),
        ])
        .assign(&patch.dropped);
        let img = Image {
            data: img.map(physmap_map),
            cmap: physmap_pal.clone(),
        };
        save_png(&img, &args.output_dir, &format!("physmap_patch{idx}_d.png"))?;
    }

    for (pos, uop) in &assets.scripts {
        println!("UOP {pos}: {uop:?}");
    }
    for (mid, msg) in &assets.msgs {
        let mut xmsg = String::new();
        for &c in msg.iter() {
            if c < 0x80 {
                xmsg.push(c as char);
            } else {
                xmsg.push_str(&format!("<{c:02x}>"));
            }
        }
        println!("MSG {mid}: {xmsg}");
    }

    for (i, mut flipper) in assets.flippers.values().cloned().enumerate() {
        for (j, frame) in flipper.gfx.iter().enumerate() {
            let image = Image {
                data: frame.clone(),
                cmap: assets.main_board.cmap.clone(),
            };
            save_png(&image, &args.output_dir, &format!("flipper{i}_{j}.png"))?;
        }
        for (j, frame) in flipper.physmap.iter().enumerate() {
            let mut img = assets.physmaps[Layer::Ground].clone();
            img.slice_mut(s![
                (flipper.rect_pos.0 as usize)..(flipper.rect_pos.0 as usize + frame.dim().0),
                (flipper.rect_pos.1 as usize)..(flipper.rect_pos.1 as usize + frame.dim().1),
            ])
            .assign(frame);
            let img = Image {
                data: img.map(physmap_map),
                cmap: physmap_pal.clone(),
            };
            save_png(
                &img,
                &args.output_dir,
                &format!("physmap_flipper{i}_{j}.png"),
            )?;
        }

        flipper.gfx.clear();
        flipper.physmap.clear();
        println!("FLIPPER: {flipper:#?}");
    }
    for pix in assets.ball_outline {
        println!("{pix:?}");
    }
    for (x, y) in assets.ball_outline_by_angle {
        println!("{x} {y}");
    }

    Ok(())
}
