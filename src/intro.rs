#![allow(clippy::collapsible_else_if)]

use std::collections::HashSet;

use arrayvec::ArrayVec;
use unnamed_entity::EntityId;
use winit::event::{ElementState, TouchPhase};
use winit::keyboard::KeyCode;

use crate::icons::IconKind;
use crate::{
    assets::{
        iff::Image,
        intro::{Assets, SlideId, TableSet, TextPageId, CGA_FONT},
    },
    config::{Config, Resolution, ScrollSpeed, TableId},
    sound::player::Player,
    view::{Action, Route, View},
};

pub struct Intro {
    player: Player,
    assets: Assets,
    config: Config,
    state: State,
    text_page: TextPageId,
    key: KeyPress,
    left_state: LeftState,
    left_is_options: bool,
    touch_static: HashSet<u64>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum IntroAction {
    SkipToTables,
    SkipToText,
    Options,
    Table(TableId),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum KeyPress {
    None,
    Table(TableId),
    Options,
    Enter,
    Space,
    Escape,
    Up,
    Down,
    Option(u8),
}

#[derive(Copy, Clone, Debug)]
enum State {
    Slide(SlideId, SlideState),
    InitDelay(u8),
    Left(u16),
    TablesGap(u16),
    TablesWarpIn(u8),
    Tables(u16),
    TablesWarpOut(u8, IntroAction),
    TablesFadeOut(u8, Action),
    TextGap(u16),
    TextFadeIn(u8),
    Text(u16),
    TextFadeOut(u8, IntroAction),
    OptionsGap(u16),
    OptionsFadeIn(u8),
    Options(u8),
    OptionsFadeOut(u8),
    FadeOut(u8, Action),
}

#[derive(Copy, Clone, Debug)]
enum SlideState {
    Gap(u8),
    FadeIn(u8),
    Show,
    FadeOut(u8),
}

#[derive(Copy, Clone, Debug)]
enum LeftState {
    None,
    Image(u16),
    ImageOut(u16),
    TextIn(u16, bool),
    Text(u16, bool),
    TextOut(u16, bool),
}

impl Intro {
    pub fn new(prg: &[u8], module: &[u8], config: Config, table: Option<TableId>) -> Intro {
        let module = crate::sound::loader::load(module);
        let player = crate::sound::player::play(module, None);
        let (state, text_page) = match table {
            Some(TableId::Table1 | TableId::Table2) => {
                (State::InitDelay(0), TextPageId::from_idx(0))
            }
            Some(TableId::Table3 | TableId::Table4) => {
                (State::InitDelay(0), TextPageId::from_idx(1))
            }
            None => (
                State::Slide(SlideId::from_idx(0), SlideState::Gap(0)),
                TextPageId::from_idx(0),
            ),
        };
        Intro {
            player,
            assets: Assets::load(prg),
            config,
            state,
            text_page,
            key: KeyPress::None,
            left_state: LeftState::None,
            left_is_options: false,
            touch_static: HashSet::new(),
        }
    }

    fn is_vertical(&self) -> bool {
        self.config.options.resolution == Resolution::Full
    }

    fn clear_left(&self, data: &mut [u8], num: usize) {
        for y in 0..num {
            let y = 95 + y;
            for x in 8..120 {
                data[y * 2 * 640 + x] = 0x2;
                data[(y * 2 + 1) * 640 + x] = 0x2;
            }
        }
    }

    fn render_left_text(&self, data: &mut [u8], base_y: usize, num: usize, is_options: bool) {
        let text = if is_options {
            &self.assets.left_text_options
        } else {
            &self.assets.left_text_menu
        };
        for (ty, line) in text.iter().enumerate() {
            for (tx, &chr) in line.iter().enumerate() {
                if ty * 12 + tx < num {
                    let y = base_y + 9 * ty;
                    let x = 16 + 8 * tx;
                    for cy in 0..8 {
                        let byte = CGA_FONT[chr as usize][cy];
                        for dx in 0..8 {
                            if (byte & 0x80 >> dx) != 0 {
                                data[(y + cy) * 2 * 640 + x + dx] = 0;
                                data[((y + cy) * 2 + 1) * 640 + x + dx] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    fn unclear_left(&self, data: &mut [u8], num: usize) {
        for y in (90 - num)..90 {
            let y = 95 + y;
            for x in 8..120 {
                let pix = self.assets.left.data[(x, y)];
                data[y * 2 * 640 + x] = pix;
                data[(y * 2 + 1) * 640 + x] = pix;
            }
        }
    }

    fn render_left(&self, data: &mut [u8], pal: &mut [(u8, u8, u8)], offset: usize) {
        pal[..16].copy_from_slice(&self.assets.left.cmap);
        if self.is_vertical() {
            for y in 0..960 {
                let sy = match y {
                    0..=373 => y / 2,
                    374..=854 => 186,
                    _ => (y - 480) / 2,
                };
                for x in offset..self.assets.left.data.dim().0 {
                    let pixel = &mut data[y * 640 + x - offset];
                    *pixel = self.assets.left.data[(x, sy)];
                }
            }
            if offset == 0 {
                self.render_left_text(
                    data,
                    187 + 70,
                    120,
                    matches!(
                        self.state,
                        State::Options(_) | State::OptionsFadeIn(_) | State::OptionsFadeOut(_)
                    ),
                );
            }
        } else {
            for y in 0..480 {
                for x in offset..self.assets.left.data.dim().0 {
                    let pixel = &mut data[y * 640 + x - offset];
                    *pixel = self.assets.left.data[(x, y / 2)];
                }
            }
            match self.left_state {
                LeftState::None | LeftState::Image(_) => {}
                LeftState::ImageOut(n) => {
                    self.clear_left(data, n as usize);
                }
                LeftState::TextIn(n, is_options) => {
                    self.clear_left(data, 90);
                    self.render_left_text(data, 97, n as usize, is_options);
                }
                LeftState::Text(_, is_options) => {
                    self.clear_left(data, 90);
                    self.render_left_text(data, 97, 120, is_options);
                }
                LeftState::TextOut(n, is_options) => {
                    self.clear_left(data, 90);
                    self.render_left_text(data, 97, 120, is_options);
                    self.unclear_left(data, n as usize);
                }
            }
        }
    }

    fn render_table(
        &self,
        data: &mut [u8],
        pal: &mut [(u8, u8, u8)],
        f: impl Fn(usize) -> bool,
        table: TableId,
        base: usize,
        flip: bool,
    ) {
        let (pal_base, img) = match table {
            TableId::Table1 => (0x10, &self.assets.table1),
            TableId::Table2 => (0x20, &self.assets.table2),
            TableId::Table3 => (0x30, &self.assets.table3),
            TableId::Table4 => (0x40, &self.assets.table4),
        };
        pal[pal_base..pal_base + 0x10].copy_from_slice(&img.cmap);
        for y in 0..95 {
            if f(if flip { 94 - y } else { y }) {
                for x in 0..440 {
                    let pidx = (base + y) * 2 * 640 + 160 + x;
                    let pix = img.data[(x, y)] | (pal_base as u8);
                    data[pidx] = pix;
                    data[pidx + 640] = pix;
                }
            }
        }
    }

    fn render_tables(&self, data: &mut [u8], pal: &mut [(u8, u8, u8)], f: impl Fn(usize) -> bool) {
        if self.is_vertical() {
            self.render_table(data, pal, &f, TableId::Table1, 12, false);
            self.render_table(data, pal, &f, TableId::Table2, 132, false);
            self.render_table(data, pal, &f, TableId::Table3, 252, true);
            self.render_table(data, pal, &f, TableId::Table4, 372, true);
        } else if self.text_page.to_idx() % 2 == 0 {
            self.render_table(data, pal, &f, TableId::Table1, 10, false);
            self.render_table(data, pal, &f, TableId::Table2, 135, true);
        } else {
            self.render_table(data, pal, &f, TableId::Table3, 10, false);
            self.render_table(data, pal, &f, TableId::Table4, 135, true);
        }
    }

    fn render_char(&self, data: &mut [u8], font: &Image, chr: u8, x: usize, y: usize) {
        let fidx = match chr {
            b'0'..=b'9' => chr - b'0',
            b'A'..=b'Z' => chr - b'A' + 10,
            b'.' => 36,
            b':' => 37,
            b'-' => 38,
            b'>' => 39,
            _ => return,
        } as usize;
        let fx = fidx % 20 * 32;
        let fy = fidx / 20 * 14;
        for cy in 0..14 {
            for cx in 0..18 {
                let pidx = (y + cy) * 2 * 640 + x + cx;
                let pix = font.data[(fx + cx, fy + cy)];
                data[pidx] = pix | 0x10;
                data[pidx + 640] = pix | 0x10;
            }
        }
    }

    fn render_line(&self, data: &mut [u8], font: &Image, line: &[u8], y: usize) {
        let sx = 164 + (24 - line.len()) * 9;
        for (tx, &chr) in line.iter().enumerate() {
            self.render_char(data, font, chr, sx + tx * 18, y);
        }
    }

    fn render_hiscores(&self, data: &mut [u8], font: &Image, table: TableId, y: usize) {
        let name = match table {
            TableId::Table1 => b"     PARTY LAND         ",
            TableId::Table2 => b"     SPEED DEVILS       ",
            TableId::Table3 => b"     BILLION DOLLAR     ",
            TableId::Table4 => b"     STONES N BONES     ",
        };
        self.render_line(data, font, name, y);
        for (i, score) in self.config.high_scores[table].iter().enumerate() {
            let mut line = [b' '; 24];
            line[2] = b'1' + (i as u8);
            line[3] = b'.';
            line[5..8].copy_from_slice(&score.name);
            line[9] = b'-';
            line[11..23].copy_from_slice(&score.score.to_ascii());
            self.render_line(data, font, &line, y + (i + 1) * 18);
        }
    }

    fn render_text(&self, data: &mut [u8], pal: &mut [(u8, u8, u8)], lq: bool) {
        let (font, hiscores) = if lq {
            (&self.assets.font_lq, &self.assets.hiscores_lq)
        } else {
            (&self.assets.font_hq, &self.assets.hiscores_hq)
        };
        pal[0x10..0x20].copy_from_slice(&font.cmap);
        let page = &self.assets.text_pages[self.text_page];
        match page {
            crate::assets::intro::TextPage::HiScores(tset) => {
                for y in 0..hiscores.data.dim().1 {
                    for x in 0..hiscores.data.dim().0 {
                        let pidx = y * 640 * 2 + x + 184;
                        let pix = hiscores.data[(x, y)];
                        data[pidx] = pix | 0x10;
                        data[pidx + 640] = pix | 0x10;
                    }
                }
                if self.is_vertical() {
                    self.render_hiscores(data, font, TableId::Table1, 42);
                    self.render_hiscores(data, font, TableId::Table2, 150);
                    self.render_hiscores(data, font, TableId::Table3, 258);
                    self.render_hiscores(data, font, TableId::Table4, 366);
                } else {
                    match tset {
                        TableSet::Table12 => {
                            self.render_hiscores(data, font, TableId::Table1, 42);
                            self.render_hiscores(data, font, TableId::Table2, 150);
                        }
                        TableSet::Table34 => {
                            self.render_hiscores(data, font, TableId::Table3, 42);
                            self.render_hiscores(data, font, TableId::Table4, 150);
                        }
                    }
                }
            }
            crate::assets::intro::TextPage::Text(text) => {
                let base = if self.is_vertical() { 120 } else { 0 };
                for (ty, line) in text.iter().enumerate() {
                    self.render_line(data, font, line, 14 + ty * 18 + base);
                }
            }
        }
    }

    fn render_options(
        &self,
        data: &mut [u8],
        pal: &mut [(u8, u8, u8)],
        lq: bool,
        cursor: Option<u8>,
    ) {
        let pitch = if self.is_vertical() { 36 } else { 18 };
        let font = if lq {
            &self.assets.font_lq
        } else {
            &self.assets.font_hq
        };
        pal[0x10..0x20].copy_from_slice(&font.cmap);
        let mut lines = [
            b"OPTIONS MENU".to_vec(),
            vec![],
            b"  BALLS:                ".to_vec(),
            b"  ANGLE:                ".to_vec(),
            b"  SCROLLING:            ".to_vec(),
            b"  INGAME MUSIC:         ".to_vec(),
            b"  RESOLUTION:           ".to_vec(),
            b"  COLOR MODE:           ".to_vec(),
            vec![],
            b"  SAVE AND EXIT         ".to_vec(),
        ];

        lines[2][16] = b'0' + self.config.options.balls;

        if self.config.options.angle_high {
            lines[3][16..20].copy_from_slice(b"HIGH");
        } else {
            lines[3][16..19].copy_from_slice(b"LOW");
        }

        match self.config.options.scroll_speed {
            ScrollSpeed::Hard => lines[4][16..20].copy_from_slice(b"HARD"),
            ScrollSpeed::Medium => lines[4][16..22].copy_from_slice(b"MEDIUM"),
            ScrollSpeed::Soft => lines[4][16..20].copy_from_slice(b"SOFT"),
        }

        if self.config.options.no_music {
            lines[5][16..19].copy_from_slice(b"OFF");
        } else {
            lines[5][16..18].copy_from_slice(b"ON");
        }

        match self.config.options.resolution {
            Resolution::Normal => lines[6][16..22].copy_from_slice(b"NORMAL"),
            Resolution::High => lines[6][16..20].copy_from_slice(b"HIGH"),
            Resolution::Full => lines[6][16..20].copy_from_slice(b"FULL"),
        }

        if self.config.options.mono {
            lines[7][16..20].copy_from_slice(b"MONO");
        } else {
            lines[7][16..21].copy_from_slice(b"COLOR");
        }

        for (ty, line) in lines.into_iter().enumerate() {
            self.render_line(data, font, &line, 14 + ty * pitch);
        }

        if let Some(cursor) = cursor {
            let pos = if cursor == 6 { 9 } else { cursor as usize + 2 };
            self.render_char(data, font, b'>', 175, 14 + pos * pitch);
        }
    }

    fn next_page(&mut self) {
        if self.is_vertical() {
            self.text_page = TextPageId::from_idx(match self.text_page.to_idx() {
                0 | 1 => 2,
                2 => 3,
                3 => 4,
                4 | 5 => 6,
                6 => 7,
                _ => 0,
            });
        } else {
            self.text_page += 1;
        }
        if self.text_page == self.assets.text_pages.next_id() {
            self.text_page = TextPageId::from_idx(0);
        }
    }

    fn handle_tap(&mut self, pos: (i32, i32)) {
        if matches!(self.state, State::Slide(_, _)) {
            self.key = KeyPress::Space;
            return;
        }
        match self.state {
            State::Slide(_, _) => {
                self.key = KeyPress::Space;
            }
            State::InitDelay(_) => (),
            State::Left(_) => (),
            State::TablesGap(_) => (),
            State::TablesWarpIn(_) | State::Tables(_) | State::TablesWarpOut(_, _) => {
                if pos.0 < 128 {
                    // left panel
                    self.key = KeyPress::Space;
                } else {
                    let table = if self.is_vertical() {
                        match pos.1 / 240 {
                            0 => TableId::Table1,
                            1 => TableId::Table2,
                            2 => TableId::Table3,
                            _ => TableId::Table4,
                        }
                    } else {
                        if self.text_page.to_idx() % 2 == 0 {
                            if pos.1 < 240 {
                                TableId::Table1
                            } else {
                                TableId::Table2
                            }
                        } else {
                            if pos.1 < 240 {
                                TableId::Table3
                            } else {
                                TableId::Table4
                            }
                        }
                    };
                    self.key = KeyPress::Table(table);
                }
            }
            State::TablesFadeOut(_, _) => (),
            State::TextGap(_)
            | State::TextFadeIn(_)
            | State::Text(_)
            | State::TextFadeOut(_, _) => {
                self.key = KeyPress::Space;
            }
            State::OptionsGap(_) => (),
            State::OptionsFadeIn(_) => (),
            State::Options(_) => {
                if pos.0 >= 128 {
                    let idx = if self.is_vertical() {
                        if pos.1 < 10 {
                            0
                        } else {
                            (pos.1 - 10) / 72
                        }
                    } else {
                        if pos.1 < 28 {
                            0
                        } else {
                            (pos.1 - 28) / 36
                        }
                    };
                    match idx {
                        2..=7 => self.key = KeyPress::Option(idx as u8 - 2),
                        9 => self.key = KeyPress::Escape,
                        _ => (),
                    }
                }
            }
            State::OptionsFadeOut(_) => (),
            State::FadeOut(_, _) => (),
        }
    }

    fn handle_option(&mut self, which: u8) {
        match which {
            0 => {
                if self.config.options.balls == 3 {
                    self.config.options.balls = 5;
                } else {
                    self.config.options.balls = 3;
                }
            }
            1 => self.config.options.angle_high = !self.config.options.angle_high,
            2 => {
                self.config.options.scroll_speed = match self.config.options.scroll_speed {
                    ScrollSpeed::Hard => ScrollSpeed::Medium,
                    ScrollSpeed::Medium => ScrollSpeed::Soft,
                    ScrollSpeed::Soft => ScrollSpeed::Hard,
                }
            }
            3 => self.config.options.no_music = !self.config.options.no_music,
            4 => {
                self.config.options.resolution = match self.config.options.resolution {
                    Resolution::Normal => Resolution::High,
                    Resolution::High => Resolution::Full,
                    Resolution::Full => Resolution::Normal,
                };
            }
            5 => self.config.options.mono = !self.config.options.mono,
            _ => self.state = State::OptionsFadeOut(0),
        }
    }
}

fn fade_pal(
    dst: &mut [(u8, u8, u8)],
    src: &[(u8, u8, u8)],
    color: (u8, u8, u8),
    num: usize,
    den: usize,
) {
    for (i, pcol) in src.iter().copied().enumerate() {
        dst[i].0 = ((pcol.0 as usize * num + color.0 as usize * (den - num)) / den) as u8;
        dst[i].1 = ((pcol.1 as usize * num + color.1 as usize * (den - num)) / den) as u8;
        dst[i].2 = ((pcol.2 as usize * num + color.2 as usize * (den - num)) / den) as u8;
    }
}

impl View for Intro {
    fn get_resolution(&self) -> (u32, u32) {
        if self.is_vertical() {
            (640, 960)
        } else {
            (640, 480)
        }
    }

    fn get_fps(&self) -> u32 {
        60
    }

    fn get_touch_icons(&self) -> ArrayVec<(usize, IconKind), 8> {
        [(6, IconKind::Fullscreen), (7, IconKind::Options)]
            .into_iter()
            .collect()
    }

    fn run_frame(&mut self) -> Action {
        match self.left_state {
            LeftState::None => {}
            LeftState::Image(ref mut n) => {
                *n += 1;
                if *n >= 480 {
                    self.left_state = LeftState::ImageOut(0);
                }
            }
            LeftState::ImageOut(ref mut n) => {
                *n += 3;
                if *n >= 90 {
                    self.left_state = LeftState::TextIn(0, self.left_is_options);
                }
            }
            LeftState::TextIn(ref mut n, is_options) => {
                *n += 1;
                if *n >= 120 {
                    self.left_state = LeftState::Text(0, is_options);
                }
            }
            LeftState::Text(ref mut n, is_options) => {
                *n += 1;
                if *n >= 480 {
                    self.left_state = LeftState::TextOut(0, is_options);
                }
            }
            LeftState::TextOut(ref mut n, _) => {
                *n += 1;
                if *n >= 90 {
                    self.left_state = LeftState::Image(0);
                }
            }
        }
        match self.state {
            State::Slide(ref mut slide_idx, ref mut sstate) => {
                let slide = &self.assets.slides[*slide_idx];
                match sstate {
                    SlideState::Gap(ref mut n) => {
                        *n += 1;
                        if *n >= slide.gap_frames {
                            *sstate = SlideState::FadeIn(0);
                        }
                    }
                    SlideState::FadeIn(ref mut n) => {
                        *n += 1;
                        if *n >= slide.fade_in_frames {
                            *sstate = SlideState::Show;
                        }
                    }
                    SlideState::Show => {
                        if self.player.ticks() >= slide.fade_out_tick || self.key == KeyPress::Space
                        {
                            *sstate = SlideState::FadeOut(0);
                        }
                    }
                    SlideState::FadeOut(ref mut n) => {
                        *n += 1;
                        if *n >= slide.fade_out_frames {
                            *slide_idx += 1;
                            if *slide_idx == self.assets.slides.next_id()
                                || self.key == KeyPress::Space
                            {
                                self.state = State::InitDelay(0);
                                if self.key == KeyPress::Space {
                                    self.key = KeyPress::None;
                                }
                            } else {
                                let slide = &self.assets.slides[*slide_idx];
                                if slide.gap_frames != 0 {
                                    *sstate = SlideState::Gap(0);
                                } else {
                                    *sstate = SlideState::FadeIn(0);
                                }
                            }
                        }
                    }
                };
            }
            State::InitDelay(ref mut n) => {
                *n += 1;
                if *n >= 11 {
                    self.state = State::Left(128);
                }
            }
            State::Left(ref mut n) => {
                if *n != 0 {
                    *n -= 8;
                } else {
                    self.state = State::TablesGap(0);
                    self.left_state = LeftState::Image(0);
                }
            }
            State::TablesGap(ref mut n) => {
                *n += 1;
                if *n >= 20 {
                    self.state = State::TablesWarpIn(0);
                }
            }
            State::TablesWarpIn(ref mut n) => {
                *n += 1;
                if *n >= self.assets.warp_frames {
                    self.state = State::Tables(0);
                }
            }
            State::TablesFadeOut(ref mut n, action) => {
                self.player.set_master_volume(0x100 * (80 - *n) as u32 / 80);
                if *n >= 80 {
                    return action;
                }
                *n += 1;
            }
            State::Tables(ref mut n) => {
                *n += 1;
                match self.key {
                    KeyPress::Table(table) => {
                        self.state = State::TablesFadeOut(0, Action::Navigate(Route::Table(table)));
                    }
                    KeyPress::Options => {
                        self.state = State::TablesWarpOut(0, IntroAction::Options);
                    }
                    KeyPress::Space => {
                        self.state = State::TablesWarpOut(0, IntroAction::SkipToText);
                    }
                    KeyPress::Enter => {
                        self.state = State::TablesWarpOut(0, IntroAction::SkipToTables);
                    }
                    KeyPress::Escape => {
                        self.state = State::TablesFadeOut(0, Action::Exit);
                    }
                    _ => {
                        if *n >= 540 {
                            self.state = State::TablesWarpOut(0, IntroAction::SkipToText);
                        }
                    }
                }
                self.key = KeyPress::None;
            }
            State::TablesWarpOut(ref mut n, action) => {
                *n += 1;
                if *n >= self.assets.warp_frames {
                    match action {
                        IntroAction::SkipToTables => {
                            self.next_page();
                            self.state = State::TablesGap(0);
                        }
                        IntroAction::SkipToText => {
                            self.state = State::TextGap(0);
                        }
                        IntroAction::Options => {
                            self.state = State::OptionsGap(0);
                            self.left_is_options = true;
                        }
                        IntroAction::Table(_) => unreachable!(),
                    }
                }
            }
            State::TextGap(ref mut n) => {
                *n += 1;
                if *n >= 5 {
                    self.state = State::TextFadeIn(0);
                }
            }
            State::TextFadeIn(ref mut n) => {
                *n += 1;
                if *n >= 20 {
                    self.state = State::Text(0);
                }
            }
            State::Text(ref mut n) => {
                *n += 1;
                match self.key {
                    KeyPress::Table(table) => {
                        self.state = State::TextFadeOut(0, IntroAction::Table(table));
                    }
                    KeyPress::Options => {
                        self.state = State::TextFadeOut(0, IntroAction::Options);
                    }
                    KeyPress::Enter | KeyPress::Space | KeyPress::Escape => {
                        self.state = State::TextFadeOut(0, IntroAction::SkipToTables);
                    }
                    _ => {
                        if *n >= 420 {
                            self.state = State::TextFadeOut(0, IntroAction::SkipToTables);
                        }
                    }
                }
                self.key = KeyPress::None;
            }
            State::TextFadeOut(ref mut n, action) => {
                *n += 1;
                if *n >= 20 {
                    match action {
                        IntroAction::SkipToTables => {
                            self.next_page();
                            self.state = State::TablesGap(0);
                        }
                        IntroAction::Options => {
                            self.next_page();
                            self.state = State::OptionsGap(0);
                            self.left_is_options = true;
                        }
                        IntroAction::Table(table) => {
                            self.state = State::FadeOut(0, Action::Navigate(Route::Table(table)));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            State::OptionsGap(ref mut n) => {
                *n += 1;
                if *n >= 5 {
                    self.state = State::OptionsFadeIn(0);
                }
            }
            State::OptionsFadeIn(ref mut n) => {
                *n += 1;
                if *n >= 40 {
                    self.state = State::Options(0);
                }
            }
            State::Options(ref mut cursor) => {
                match self.key {
                    KeyPress::Enter | KeyPress::Space => {
                        let option = *cursor;
                        self.handle_option(option);
                    }
                    KeyPress::Escape => {
                        self.state = State::OptionsFadeOut(0);
                    }
                    KeyPress::Up => {
                        if *cursor == 0 {
                            *cursor = 6;
                        } else {
                            *cursor -= 1;
                        }
                    }
                    KeyPress::Down => {
                        if *cursor == 6 {
                            *cursor = 0;
                        } else {
                            *cursor += 1;
                        }
                    }
                    KeyPress::Option(option) => self.handle_option(option),
                    _ => {}
                }
                self.key = KeyPress::None;
            }
            State::OptionsFadeOut(ref mut n) => {
                *n += 1;
                if *n >= 40 {
                    self.state = State::TablesGap(0);
                    self.left_is_options = false;
                    return Action::SaveOptions(self.config.options);
                }
            }
            State::FadeOut(ref mut n, action) => {
                self.player.set_master_volume(0x100 * (80 - *n) as u32 / 80);
                if *n >= 80 {
                    return action;
                }
                *n += 1;
            }
        }
        Action::None
    }

    fn handle_key(&mut self, key: KeyCode, state: ElementState) {
        if state != ElementState::Pressed {
            return;
        }
        match key {
            KeyCode::F1 | KeyCode::Digit1 => self.key = KeyPress::Table(TableId::Table1),
            KeyCode::F2 | KeyCode::Digit2 => self.key = KeyPress::Table(TableId::Table2),
            KeyCode::F3 | KeyCode::Digit3 => self.key = KeyPress::Table(TableId::Table3),
            KeyCode::F4 | KeyCode::Digit4 => self.key = KeyPress::Table(TableId::Table4),
            KeyCode::F5 | KeyCode::Digit5 => self.key = KeyPress::Options,
            KeyCode::Escape => self.key = KeyPress::Escape,
            KeyCode::Enter => self.key = KeyPress::Enter,
            KeyCode::Space => self.key = KeyPress::Space,
            KeyCode::ArrowDown => self.key = KeyPress::Down,
            KeyCode::ArrowUp => self.key = KeyPress::Up,
            _ => (),
        }
    }

    fn render(&self, data: &mut [u8], pal: &mut [(u8, u8, u8)]) {
        match self.state {
            State::Slide(sidx, sstate) => {
                let base = if self.is_vertical() { 240 } else { 0 };
                let slide = &self.assets.slides[sidx];
                let img = &slide.image;
                match img.data.dim().0 {
                    320 => {
                        assert_eq!(img.data.dim().1, 240);
                        for y in 0..480 {
                            for x in 0..640 {
                                data[x + (base + y) * 640] = img.data[(x / 2, y / 2)];
                            }
                        }
                    }
                    640 => {
                        assert_eq!(img.data.dim().1, 480);
                        for y in 0..480 {
                            for x in 0..640 {
                                data[x + (base + y) * 640] = img.data[(x, y)];
                            }
                        }
                    }
                    _ => panic!("weird width"),
                }
                match sstate {
                    SlideState::Gap(_) => {
                        pal.fill((0, 0, 0));
                    }
                    SlideState::FadeIn(num) => {
                        let color = if slide.fade_from_white {
                            (0xff, 0xff, 0xff)
                        } else {
                            (0, 0, 0)
                        };
                        fade_pal(
                            pal,
                            &img.cmap,
                            color,
                            num as usize,
                            slide.fade_in_frames as usize,
                        );
                    }
                    SlideState::Show => {
                        pal[..img.cmap.len()].copy_from_slice(&img.cmap);
                    }
                    SlideState::FadeOut(num) => {
                        let den = slide.fade_out_frames;
                        fade_pal(
                            pal,
                            &img.cmap,
                            (0, 0, 0),
                            (den - num) as usize,
                            slide.fade_out_frames as usize,
                        );
                    }
                }
            }
            State::InitDelay(_) => {
                data.fill(0);
                pal.fill((0, 0, 0));
            }
            State::Left(delta) => {
                self.render_left(data, pal, delta as usize);
            }
            State::TablesGap(_) | State::TextGap(_) | State::OptionsGap(_) => {
                self.render_left(data, pal, 0);
            }
            State::TablesWarpIn(n) => {
                self.render_left(data, pal, 0);
                self.render_tables(data, pal, |i| self.assets.warp_table[i] < n);
            }
            State::Tables(_) => {
                self.render_left(data, pal, 0);
                self.render_tables(data, pal, |_| true);
            }
            State::TablesWarpOut(n, _) => {
                self.render_left(data, pal, 0);
                self.render_tables(data, pal, |i| self.assets.warp_table[94 - i] >= n);
            }
            State::TablesFadeOut(n, _) => {
                self.render_left(data, pal, 0);
                self.render_tables(data, pal, |_| true);
                let opal = pal.to_vec();
                fade_pal(pal, &opal, (0, 0, 0), (80 - n) as usize, 80);
            }
            State::TextFadeIn(n) => {
                self.render_left(data, pal, 0);
                self.render_text(data, pal, true);
                for pe in &mut pal[0x10..0x20] {
                    pe.0 = (pe.0 as u32 * (n as u32) / 20) as u8;
                    pe.1 = (pe.1 as u32 * (n as u32) / 20) as u8;
                    pe.2 = (pe.2 as u32 * (n as u32) / 20) as u8;
                }
            }
            State::Text(_) => {
                self.render_left(data, pal, 0);
                self.render_text(data, pal, false);
            }
            State::TextFadeOut(n, _) => {
                self.render_left(data, pal, 0);
                self.render_text(data, pal, true);
                for pe in &mut pal[0x10..0x20] {
                    pe.0 = (pe.0 as u32 * (19 - n as u32) / 20) as u8;
                    pe.1 = (pe.1 as u32 * (19 - n as u32) / 20) as u8;
                    pe.2 = (pe.2 as u32 * (19 - n as u32) / 20) as u8;
                }
            }
            State::OptionsFadeIn(n) => {
                self.render_left(data, pal, 0);
                self.render_options(data, pal, true, None);
                for pe in &mut pal[0x10..0x20] {
                    pe.0 = (pe.0 as u32 * (n as u32) / 40) as u8;
                    pe.1 = (pe.1 as u32 * (n as u32) / 40) as u8;
                    pe.2 = (pe.2 as u32 * (n as u32) / 40) as u8;
                }
            }
            State::Options(cursor) => {
                self.render_left(data, pal, 0);
                self.render_options(data, pal, false, Some(cursor));
            }
            State::OptionsFadeOut(n) => {
                self.render_left(data, pal, 0);
                self.render_options(data, pal, true, None);
                for pe in &mut pal[0x10..0x20] {
                    pe.0 = (pe.0 as u32 * (39 - n as u32) / 40) as u8;
                    pe.1 = (pe.1 as u32 * (39 - n as u32) / 40) as u8;
                    pe.2 = (pe.2 as u32 * (39 - n as u32) / 40) as u8;
                }
            }
            State::FadeOut(n, _) => {
                self.render_left(data, pal, 0);
                let opal = pal.to_vec();
                fade_pal(pal, &opal, (0, 0, 0), (80 - n) as usize, 80);
            }
        }
    }

    fn handle_touch(&mut self, id: u64, phase: TouchPhase, pos: (i32, i32)) {
        match phase {
            TouchPhase::Started => {
                self.touch_static.remove(&id);
                if pos.0 >= 0
                    && pos.1 >= 0
                    && pos.0 < 640
                    && pos.1 < (if self.is_vertical() { 960 } else { 480 })
                {
                    self.touch_static.insert(id);
                }
            }
            TouchPhase::Ended => {
                if self.touch_static.contains(&id) {
                    self.handle_tap(pos);
                }
            }
            TouchPhase::Moved | TouchPhase::Cancelled => {
                self.touch_static.remove(&id);
            }
        }
    }

    fn handle_touch_icon(&mut self, icon: IconKind) {
        match icon {
            IconKind::Options => {
                self.key = KeyPress::Options;
            }
            IconKind::Back => {
                self.key = KeyPress::Escape;
            }
            _ => (),
        }
    }
}
