use game_loop::game_loop;
use std::path::PathBuf;

use clap::Parser;
use pfr::{
    config::{save_high_scores, Config, FileConfigStore, Resolution, TableId},
    icons::IconKind,
    intro::Intro,
    table::Table,
    view::{Action, Route, View},
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyEvent, MouseButton, TouchPhase, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Fullscreen, WindowBuilder},
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
    #[clap(long)]
    touch: bool,
}

fn main() {
    let args = Args::parse();
    let cstore = FileConfigStore::new(&args.data);
    let config = Config::load(&cstore);
    let event_loop = EventLoop::new().unwrap();
    let mut dims = if config.options.resolution == Resolution::Full {
        (640, (576 + 33) * 2)
    } else {
        (640, 480)
    };
    if args.touch {
        dims.1 += 80;
    }
    let window = WindowBuilder::new()
        .with_title("Pinball Fantasies")
        .with_min_inner_size(PhysicalSize::new(dims.0, dims.1))
        .with_inner_size(PhysicalSize::new(dims.0, dims.1))
        .build(&event_loop)
        .unwrap();
    window.set_cursor_visible(false);
    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(dims.0, dims.1, surface_texture).unwrap()
    };
    let game = Game {
        pixels,
        args,
        config,
        view: None,
        dims,
    };
    game_loop(
        event_loop,
        window.into(),
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
                let mut dims = view.get_resolution();
                if g.game.args.touch {
                    dims.1 += if dims.0 == 320 { 40 } else { 80 };
                }
                if dims != g.game.dims {
                    g.window.set_resizable(true);
                    if g.window.fullscreen().is_none() {
                        let size = if matches!(dims, (320, 240) | (320, 350)) {
                            PhysicalSize::new(dims.0 * 4, dims.1 * 4)
                        } else if dims == (640, 480) || dims.0 == 320 {
                            PhysicalSize::new(dims.0 * 2, dims.1 * 2)
                        } else {
                            PhysicalSize::new(dims.0, dims.1)
                        };
                        if let Some(size) = g.window.request_inner_size(size) {
                            g.game
                                .pixels
                                .resize_surface(size.width, size.height)
                                .unwrap();
                        }
                    }
                    g.game.pixels.resize_buffer(dims.0, dims.1).unwrap();
                    g.game.dims = dims;
                }
            }
            let double = g.game.dims.0 == 320;
            let frame = g.game.pixels.frame_mut();
            let width = g.game.dims.0 as usize;
            let mut height = g.game.dims.1 as usize;
            if g.game.args.touch {
                height -= if double { 40 } else { 80 };
            }
            let mut data = vec![0u8; width * height];
            let mut pal = [(0u8, 0u8, 0u8); 256];
            if let Some(ref view) = g.game.view {
                view.render(&mut data, &mut pal);
            }
            let offset = if !g.game.args.touch {
                0
            } else if double {
                40 * 320
            } else {
                80 * 640
            };
            for y in 0..height {
                for x in 0..width {
                    let pidx = y * width + x;
                    let pixel = usize::from(data[pidx]);
                    frame[(offset + pidx) * 4] = pal[pixel].0;
                    frame[(offset + pidx) * 4 + 1] = pal[pixel].1;
                    frame[(offset + pidx) * 4 + 2] = pal[pixel].2;
                    frame[(offset + pidx) * 4 + 3] = 0xff;
                }
            }
            if g.game.args.touch {
                for (i, x) in frame[..offset * 4].iter_mut().enumerate() {
                    if i % 4 == 3 {
                        *x = 0xff;
                    } else {
                        *x = 0;
                    }
                }
                if let Some(ref view) = g.game.view {
                    for (pos, icon) in view.get_touch_icons() {
                        // TODO: icons
                        let _ = icon;
                        if double {
                            for dy in 0..32 {
                                for dx in 0..32 {
                                    let pidx = (dy + 4) * width + (dx + 4 + 40 * pos);
                                    frame[pidx * 4] = 0xff;
                                    frame[pidx * 4 + 1] = 0xff;
                                    frame[pidx * 4 + 2] = 0xff;
                                    frame[pidx * 4 + 3] = 0xff;
                                }
                            }
                        } else {
                            for dy in 0..64 {
                                for dx in 0..64 {
                                    let pidx = (dy + 8) * width + (dx + 8 + 80 * pos);
                                    frame[pidx * 4] = 0xff;
                                    frame[pidx * 4 + 1] = 0xff;
                                    frame[pidx * 4 + 2] = 0xff;
                                    frame[pidx * 4 + 3] = 0xff;
                                }
                            }
                        }
                    }
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
                            event:
                                KeyEvent {
                                    physical_key: key,
                                    state,
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    if let Some(ref mut view) = g.game.view {
                        if let PhysicalKey::Code(key) = *key {
                            if key == KeyCode::F11 && *state == ElementState::Pressed {
                                if g.window.fullscreen().is_some() {
                                    g.window.set_fullscreen(None);
                                } else {
                                    g.window.set_fullscreen(Some(Fullscreen::Borderless(None)))
                                }
                            }
                            view.handle_key(key, *state);
                        }
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { button, state, .. },
                    ..
                } => {
                    if let Some(ref mut view) = g.game.view {
                        if &MouseButton::Left == button {
                            view.handle_key(KeyCode::ShiftLeft, *state);
                        }
                        if &MouseButton::Right == button {
                            view.handle_key(KeyCode::ShiftRight, *state);
                        }
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::Touch(touch),
                    ..
                } => {
                    if let Some(ref mut view) = g.game.view {
                        let pos = g.game.pixels.window_pos_to_pixel((
                            touch.location.x as f32,
                            touch.location.y as f32,
                        ));
                        let mut pos = match pos {
                            Ok((x, y)) => (x as i32, y as i32),
                            Err((x, y)) => (x as i32, y as i32),
                        };
                        if g.game.args.touch {
                            let double = view.get_resolution().0 == 320;
                            let unit = if double { 40 } else { 80 };
                            if touch.phase == TouchPhase::Started
                                && pos.1 >= 0
                                && pos.1 < unit
                                && pos.0 >= 0
                                && pos.0 < view.get_resolution().0 as i32
                            {
                                let idx = (pos.0 / unit) as usize;
                                for (iidx, icon) in view.get_touch_icons() {
                                    if idx == iidx {
                                        if icon == IconKind::Fullscreen {
                                            if g.window.fullscreen().is_some() {
                                                g.window.set_fullscreen(None);
                                            } else {
                                                g.window.set_fullscreen(Some(
                                                    Fullscreen::Borderless(None),
                                                ))
                                            }
                                        }
                                        view.handle_touch_icon(icon);
                                    }
                                }
                            }
                            pos.1 -= unit;
                        }
                        view.handle_touch(touch.id, touch.phase, pos);
                    }
                }

                _ => {}
            }
        },
    )
    .unwrap();
}
