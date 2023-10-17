use clap::Parser;
use pfr::assets::iff::Image;
use pfr::assets::intro::{Assets, SlideId, TextPage};
use std::io::BufWriter;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use unnamed_entity::EntityId;

#[derive(Parser)]
struct Args {
    input_dir: PathBuf,
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
    let assets = Assets::load(args.input_dir.join("INTRO.PRG"))?;
    save_png(
        &assets.slides[SlideId::from_idx(0)].image,
        &args.output_dir,
        "logo0.png",
    )?;
    save_png(
        &assets.slides[SlideId::from_idx(1)].image,
        &args.output_dir,
        "logo1.png",
    )?;
    save_png(
        &assets.slides[SlideId::from_idx(2)].image,
        &args.output_dir,
        "logo2.png",
    )?;
    save_png(
        &assets.slides[SlideId::from_idx(3)].image,
        &args.output_dir,
        "presents.png",
    )?;
    save_png(
        &assets.slides[SlideId::from_idx(4)].image,
        &args.output_dir,
        "pflogo.png",
    )?;
    save_png(&assets.left, &args.output_dir, "left.png")?;
    save_png(&assets.table1, &args.output_dir, "table1.png")?;
    save_png(&assets.table2, &args.output_dir, "table2.png")?;
    save_png(&assets.table3, &args.output_dir, "table3.png")?;
    save_png(&assets.table4, &args.output_dir, "table4.png")?;
    save_png(&assets.font_lq, &args.output_dir, "font_lq.png")?;
    save_png(&assets.font_hq, &args.output_dir, "font_hq.png")?;
    save_png(&assets.hiscores_lq, &args.output_dir, "hiscores_lq.png")?;
    save_png(&assets.hiscores_hq, &args.output_dir, "hiscores_hq.png")?;
    for tp in assets.text_pages.values() {
        match tp {
            TextPage::HiScores(ts) => {
                println!("TEXT PAGE: highscores of {ts:?}");
            }
            TextPage::Text(t) => {
                println!("TEXT PAGE:");
                for l in t {
                    println!("> {}", std::str::from_utf8(l).unwrap());
                }
            }
        }
    }
    println!("LEFT TEXT MENU:");
    for l in &assets.left_text_menu {
        println!("> {}", std::str::from_utf8(l).unwrap());
    }
    println!("LEFT TEXT OPTIONS:");
    for l in &assets.left_text_options {
        println!("> {}", std::str::from_utf8(l).unwrap());
    }
    println!(
        "WARP: {} {} {:?}",
        assets.warp_frames,
        assets.warp_table.len(),
        assets.warp_table
    );
    Ok(())
}
