use crate::{
    assets::table::{
        dm::DmFont,
        script::{special_chars, DmAnimFrameId, DmAnimId, DmCoord, MsgId},
    },
    bcd::Bcd,
};

use super::Table;

pub struct DotMatrix {
    pub pixels: [[bool; 160]; 16],
    saved: [[bool; 160]; 16],
    state: bool,
    blink: Option<Blink>,
}

struct Blink {
    timer: u16,
    period: u16,
}

impl DotMatrix {
    pub fn new() -> Self {
        DotMatrix {
            pixels: [[false; 160]; 16],
            saved: [[false; 160]; 16],
            state: true,
            blink: None,
        }
    }

    pub fn save(&mut self) {
        self.saved = self.pixels;
    }

    pub fn restore(&mut self) {
        self.pixels = self.saved;
    }

    pub fn stop_blink(&mut self) {
        self.state = true;
        self.blink = None;
    }

    pub fn start_blink(&mut self, period: u16) {
        self.state = true;
        self.blink = Some(Blink {
            timer: period,
            period,
        });
    }

    pub fn blink_frame(&mut self) {
        if let Some(ref mut blink) = self.blink {
            blink.timer -= 1;
            if blink.timer == 0 {
                blink.timer = blink.period;
                self.state = !self.state;
            }
        }
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }

    pub fn state(&self) -> bool {
        self.state
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; 160]; 16];
    }
}

impl Table {
    fn dm_sub_char(&self, chr: u8) -> u8 {
        if chr < 0x80 {
            chr
        } else if (special_chars::HIGH_SCORES..(special_chars::HIGH_SCORES + 12)).contains(&chr) {
            let idx = (chr - special_chars::HIGH_SCORES) / 3;
            let cidx = (chr - special_chars::HIGH_SCORES) % 3;
            self.high_scores[idx as usize].name[cidx as usize]
        } else if chr == special_chars::CUR_BALL {
            b'0' + self.cur_ball
        } else if chr == special_chars::CUR_PLAYER {
            b'0' + self.cur_player
        } else if chr == special_chars::TOTAL_PLAYERS {
            b'0' + self.total_players
        } else if chr == special_chars::BONUS_MULT_L {
            if self.bonus_mult_late == 10 {
                b'1'
            } else {
                b'0' + self.bonus_mult_late
            }
        } else if chr == special_chars::BONUS_MULT_L + 1 {
            if self.bonus_mult_late == 10 {
                b'0'
            } else {
                b' '
            }
        } else if chr == special_chars::BONUS_MULT_R {
            if self.bonus_mult_late == 10 {
                b'1'
            } else {
                b' '
            }
        } else if chr == special_chars::BONUS_MULT_R + 1 {
            if self.bonus_mult_late == 10 {
                b'0'
            } else {
                b'0' + self.bonus_mult_late
            }
        } else if chr == special_chars::NUM_CYCLONES {
            if self.num_cyclone < 100 {
                b'_'
            } else {
                b'0' + (self.num_cyclone / 100 % 10) as u8
            }
        } else if chr == special_chars::NUM_CYCLONES + 1 {
            if self.num_cyclone < 10 {
                b'_'
            } else {
                b'0' + (self.num_cyclone / 10 % 10) as u8
            }
        } else if chr == special_chars::NUM_CYCLONES + 2 {
            b'0' + (self.num_cyclone % 10) as u8
        } else if chr == special_chars::NUM_CYCLONES_TARGET {
            if self.num_cyclone_target < 100 {
                b'_'
            } else {
                b'0' + (self.num_cyclone_target / 100 % 10) as u8
            }
        } else if chr == special_chars::NUM_CYCLONES_TARGET + 1 {
            if self.num_cyclone_target < 10 {
                b'_'
            } else {
                b'0' + (self.num_cyclone_target / 10 % 10) as u8
            }
        } else if chr == special_chars::NUM_CYCLONES_TARGET + 2 {
            b'0' + (self.num_cyclone_target % 10) as u8
        } else if chr == special_chars::NUM_CYCLONES_TARGET_L {
            if self.num_cyclone_target < 10 {
                b'0' + (self.num_cyclone_target % 10) as u8
            } else if self.num_cyclone_target < 100 {
                b'0' + (self.num_cyclone_target / 10 % 10) as u8
            } else {
                b'0' + (self.num_cyclone_target / 100 % 10) as u8
            }
        } else if chr == special_chars::NUM_CYCLONES_TARGET_L + 1 {
            if self.num_cyclone_target < 10 {
                b'_'
            } else if self.num_cyclone_target < 100 {
                b'0' + (self.num_cyclone_target % 10) as u8
            } else {
                b'0' + (self.num_cyclone_target / 10 % 10) as u8
            }
        } else if chr == special_chars::NUM_CYCLONES_TARGET_L + 2 {
            if self.num_cyclone_target < 100 {
                b'_'
            } else {
                b'0' + (self.num_cyclone_target % 10) as u8
            }
        } else {
            panic!("unknown subst char {chr:02x}");
        }
    }

    fn dm_put_char(&mut self, font: DmFont, pos: DmCoord, mut chr: u8) {
        chr = self.dm_sub_char(chr);
        if chr == b' ' {
            return;
        }
        let fdata = &self.assets.dm_fonts[font][&chr];
        // Special hack for LongMsg: need to clear the whole height of the cell.
        if font == DmFont::H13 && chr == b'_' {
            for y in 0..16 {
                for x in 0..8 {
                    let dx = pos.x + x;
                    if !(0..160).contains(&dx) {
                        continue;
                    }
                    self.dm.pixels[y][dx as usize] = false;
                }
            }
            return;
        }
        for y in 0..font.height() {
            let dy = pos.y + (y as i16);
            let fline = fdata[y];
            if !(0..16).contains(&dy) {
                continue;
            }
            for x in 0..8 {
                let dx = pos.x + x;
                if !(0..160).contains(&dx) {
                    continue;
                }
                self.dm.pixels[dy as usize][dx as usize] = (fline << x & 0x80) != 0;
            }
        }
    }

    pub fn dm_put_bcd(&mut self, font: DmFont, mut pos: DmCoord, num: Bcd, center: bool) {
        if center {
            pos.x -= num.leading_zeros() as i16 * 4;
        }
        for (i, chr) in num.to_ascii().into_iter().enumerate() {
            self.dm_put_char(font, pos, chr);
            if matches!(i, 2 | 5 | 8) && chr != b' ' {
                // comma
                self.dm.pixels[pos.y as usize + font.height()][pos.x as usize + 7] = true;
                self.dm.pixels[pos.y as usize + font.height()][pos.x as usize + 8] = true;
                self.dm.pixels[pos.y as usize + font.height() + 1][pos.x as usize + 6] = true;
                self.dm.pixels[pos.y as usize + font.height() + 1][pos.x as usize + 7] = true;
            }
            pos.x += 8;
        }
    }

    pub fn dm_puts(&mut self, font: DmFont, mut pos: DmCoord, msg: &[u8]) {
        for &chr in msg {
            self.dm_put_char(font, pos, chr);
            pos.x += 8;
            if pos.x >= 160 {
                break;
            }
        }
    }

    pub fn dm_anim_frame(&mut self, frame: DmAnimFrameId) {
        let frame = &self.assets.anim_frames[frame];
        for &(pos, state) in frame.iter() {
            self.dm.pixels[pos.y as usize][pos.x as usize] = state;
        }
    }
}

#[derive(Debug)]
pub struct ScriptTaskDmAnim {
    anim: DmAnimId,
    frame_idx: usize,
    delay: u16,
    repeats: u16,
}

impl ScriptTaskDmAnim {
    pub fn new(table: &Table, anim: DmAnimId) -> Self {
        Self {
            anim,
            frame_idx: 0,
            delay: 1,
            repeats: table.assets.anims[anim].repeats,
        }
    }

    pub fn run(&mut self, table: &mut Table) -> bool {
        assert_ne!(self.delay, 0);
        self.delay -= 1;
        if self.delay != 0 {
            return true;
        }
        let anim = &table.assets.anims[self.anim];
        let frame_idx = self.frame_idx;
        if self.frame_idx == anim.num_frames {
            assert_ne!(self.repeats, 0);
            self.repeats -= 1;
            if self.repeats == 0 {
                return false;
            }
            self.frame_idx = anim.restart;
        }
        let (frame, delay) = anim.frames[frame_idx];
        self.frame_idx += 1;
        table.dm_anim_frame(frame);
        self.delay = delay;
        true
    }
}

#[derive(Debug)]
pub struct ScriptTaskDmWipeDown {
    pos: usize,
}

impl ScriptTaskDmWipeDown {
    pub fn new() -> Self {
        Self { pos: 0 }
    }
    pub fn run(&mut self, table: &mut Table) -> bool {
        if self.pos == 16 {
            return false;
        }
        table.dm.pixels[self.pos] = [false; 160];
        self.pos += 1;
        true
    }
}

#[derive(Debug)]
pub struct ScriptTaskDmWipeRight {
    pos: usize,
}

impl ScriptTaskDmWipeRight {
    pub fn new() -> Self {
        Self { pos: 0 }
    }
    pub fn run(&mut self, table: &mut Table) -> bool {
        if self.pos == 160 {
            return false;
        }
        for dx in 0..2 {
            for y in 0..16 {
                table.dm.pixels[y][self.pos + dx] = false;
            }
        }
        self.pos += 2;
        true
    }
}

#[derive(Debug)]
pub struct ScriptTaskDmWipeDownStriped {
    pos: usize,
}

impl ScriptTaskDmWipeDownStriped {
    pub fn new() -> Self {
        Self { pos: 0 }
    }
    pub fn run(&mut self, table: &mut Table) -> bool {
        if self.pos == 4 {
            return false;
        }
        table.dm.pixels[self.pos] = [false; 160];
        table.dm.pixels[self.pos + 4] = [false; 160];
        table.dm.pixels[self.pos + 8] = [false; 160];
        table.dm.pixels[self.pos + 12] = [false; 160];
        self.pos += 1;
        true
    }
}

#[derive(Debug)]
pub struct ScriptTaskDmMsgScroll {
    msg: MsgId,
    pos: i16,
    target: i16,
    down: bool,
}

impl ScriptTaskDmMsgScroll {
    pub fn new(msg: MsgId, target: i16, down: bool) -> Self {
        Self {
            msg,
            pos: if down { -13 } else { 16 },
            target,
            down,
        }
    }
    pub fn run(&mut self, table: &mut Table) -> bool {
        if self.down {
            self.pos += 1;
        } else {
            self.pos -= 1;
        }
        table.dm.clear();
        table.dm_puts(
            DmFont::H13,
            DmCoord { x: 0, y: self.pos },
            &table.assets.msgs[self.msg].clone(),
        );
        self.pos != self.target
    }
}

#[derive(Debug)]
pub struct ScriptTaskDmLongMsg {
    msg: MsgId,
    pos: usize,
    x: i16,
}

impl ScriptTaskDmLongMsg {
    pub fn new(msg: MsgId) -> Self {
        Self { msg, pos: 0, x: 0 }
    }
    pub fn run(&mut self, table: &mut Table) -> bool {
        let msg = table.assets.msgs[self.msg].clone();
        if self.pos + 20 >= msg.len() {
            return false;
        }
        table.dm_puts(DmFont::H13, DmCoord { x: self.x, y: 1 }, &msg[self.pos..]);
        self.x -= 1;
        table.dm_puts(DmFont::H13, DmCoord { x: self.x, y: 1 }, &msg[self.pos..]);
        self.x -= 1;
        if self.x == -8 {
            self.x = 0;
            self.pos += 1;
        }
        true
    }
}

#[derive(Debug)]
pub struct ScriptTaskDmTowerHunt {
    target: u16,
    pos: u16,
}

impl ScriptTaskDmTowerHunt {
    pub fn new(target: u16) -> Self {
        Self { target, pos: 152 }
    }

    pub fn run(&mut self, table: &mut Table) -> bool {
        self.pos -= 1;
        table.dm.pixels.copy_from_slice(
            &table.assets.dm_tower.as_ref().unwrap()[self.pos as usize..self.pos as usize + 16],
        );
        self.target != self.pos
    }
}
