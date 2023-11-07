use game_loop::game_loop;
use std::path::PathBuf;

use clap::Parser;
use pfr::{
    config::{save_high_scores, Config, FileConfigStore, TableId},
    intro::Intro,
    table::Table,
    view::{Action, Route, View},
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

struct Game {
    pixels: Pixels,
    view: Option<Box<dyn View>>,
    config: Config,
    args: Args,
    dims: (u32, u32),
}

#[derive(Parser)]
struct Args {
    data: PathBuf,
    table: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let cstore = FileConfigStore::new(&args.data);
    let config = Config::load(&cstore);
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pinball Fantasies")
        .with_min_inner_size(PhysicalSize::new(640, 480))
        .with_inner_size(PhysicalSize::new(640 * 2, 480 * 2))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    window.set_cursor_visible(false);
    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(640, 480, surface_texture).unwrap()
    };
    let game = Game {
        pixels,
        args,
        config,
        view: None,
        dims: (640, 480),
    };
    game_loop(
        event_loop,
        window,
        game,
        60,
        0.2,
        move |g| {
            // update
            let mut action = match g.game.view {
                Some(ref mut view) => view.run_frame(),
                None => Action::Navigate(match g.game.args.table {
                    Some(t) => Route::Table(match t {
                        1 => TableId::Table1,
                        2 => TableId::Table2,
                        3 => TableId::Table3,
                        4 => TableId::Table4,
                        _ => panic!("weird table"),
                    }),
                    None => Route::Intro(None),
                }),
            };
            if g.game.args.table.is_some() && matches!(action, Action::Navigate(Route::Intro(_))) {
                action = Action::Exit;
            }
            match action {
                Action::None => {}
                Action::Navigate(route) => {
                    let (prg, module) = match route {
                        Route::Intro(None) => ("INTRO.PRG", "INTRO.MOD"),
                        Route::Intro(Some(_)) => ("INTRO.PRG", "MOD2.MOD"),
                        Route::Table(TableId::Table1) => ("TABLE1.PRG", "TABLE1.MOD"),
                        Route::Table(TableId::Table2) => ("TABLE2.PRG", "TABLE2.MOD"),
                        Route::Table(TableId::Table3) => ("TABLE3.PRG", "TABLE3.MOD"),
                        Route::Table(TableId::Table4) => ("TABLE4.PRG", "TABLE4.MOD"),
                    };
                    let prgdata = std::fs::read(g.game.args.data.join(prg)).unwrap();
                    let moddata = std::fs::read(g.game.args.data.join(module)).unwrap();
                    let view: Box<dyn View> = match route {
                        Route::Intro(table) => {
                            Box::new(Intro::new(&prgdata, &moddata, g.game.config, table))
                        }
                        Route::Table(table) => {
                            Box::new(Table::new(&prgdata, &moddata, g.game.config, table))
                        }
                    };
                    g.set_updates_per_second(view.get_fps());
                    g.game.view = Some(view);
                }
                Action::Exit => g.exit(),
                Action::SaveOptions(options) => {
                    options.save(&cstore);
                    g.game.config.options = options;
                }
                Action::SaveHighScores(table, high_scores) => {
                    save_high_scores(table, high_scores, &cstore);
                    g.game.config.high_scores[table] = high_scores;
                }
            }
        },
        |g| {
            // render
            if let Some(ref view) = g.game.view {
                let dims = view.get_resolution();
                if dims != g.game.dims {
                    g.window.set_resizable(true);
                    // g.window.set_inner_size(PhysicalSize::new(dims.0, dims.1));
                    g.game.pixels.resize_buffer(dims.0, dims.1).unwrap();
                    g.game.dims = dims;
                }
            }
            let frame = g.game.pixels.frame_mut();
            let width = g.game.dims.0 as usize;
            let height = g.game.dims.1 as usize;
            let mut data = vec![0u8; width * height];
            let mut pal = [(0u8, 0u8, 0u8); 256];
            if let Some(ref view) = g.game.view {
                view.render(&mut data, &mut pal);
            }
            for y in 0..height {
                for x in 0..width {
                    let pidx = y * width + x;
                    let pixel = usize::from(data[pidx]);
                    frame[pidx * 4] = pal[pixel].0;
                    frame[pidx * 4 + 1] = pal[pixel].1;
                    frame[pidx * 4 + 2] = pal[pixel].2;
                    frame[pidx * 4 + 3] = 0xff;
                }
            }
            g.game.pixels.render().unwrap();
        },
        |g, event| {
            // event
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    g.exit();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    g.game
                        .pixels
                        .resize_surface(size.width, size.height)
                        .unwrap();
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(key),
                                    state,
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    if let Some(ref mut view) = g.game.view {
                        view.handle_key(*key, *state);
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { button, state, .. },
                    ..
                } => {
                    if let Some(ref mut view) = g.game.view {
                        if &MouseButton::Left == button {
                            view.handle_key(VirtualKeyCode::LShift, *state);
                        }
                        if &MouseButton::Right == button {
                            view.handle_key(VirtualKeyCode::RShift, *state);
                        }
                    }
                }

                _ => {}
            }
        },
    );
}
