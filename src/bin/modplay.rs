use clap::Parser;
use pfr::{
    assets::table::sound::{Jingle, Sfx},
    sound::controller::TableSequencer,
};
use std::{path::PathBuf, sync::Arc};

#[derive(Parser)]
struct Args {
    modfile: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let moddata = std::fs::read(args.modfile)?;
    let module = pfr::sound::loader::load(&moddata);
    let sequencer = Arc::new(TableSequencer::new(0, 0, 0, false));
    let player = pfr::sound::player::play(module, Some(sequencer.clone()));
    // println!("NAME: {}", module.name);
    // for (i, pat) in module.patterns.iter().enumerate() {
    //     println!("--- PAT {i:02x} ---");
    //     for (j, l) in pat.iter().enumerate() {
    //         print!("{j:02x}:");
    //         for n in l {
    //             print!("   {n}");
    //         }
    //         println!();
    //     }
    // }
    let stdin = std::io::stdin();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        let c = buf.trim();
        if let Some(r) = c.strip_prefix('j') {
            let Ok(r) = u32::from_str_radix(r, 16) else {
                continue;
            };
            sequencer.play_jingle(
                Jingle {
                    position: (r & 0xff) as u8,
                    repeat: (r >> 8 & 0xf) as u8,
                    priority: (r >> 12 & 0xff) as u8,
                },
                false,
                None,
            );
        }
        if let Some(r) = c.strip_prefix('m') {
            let Ok(r) = u32::from_str_radix(r, 16) else {
                continue;
            };
            sequencer.set_music((r & 0xff) as u8);
        }
        let Some(r) = c.strip_prefix('s') else {
            continue;
        };
        let Ok(r) = u32::from_str_radix(r, 16) else {
            continue;
        };
        player.play_sfx(
            Sfx {
                sample: (r & 0xff) as u8,
                period: (r >> 8 & 0xff) as u8,
                channel: (r >> 24 & 0xff) as u8,
            },
            (r >> 16 & 0xff) as u8,
        );
    }
}
