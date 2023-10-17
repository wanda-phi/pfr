use crate::assets::table::script::CheatEffect;

use super::Table;

#[derive(Debug)]
pub struct CheatState {
    pub no_tilt: bool,
    pub slowdown: bool,
    pub buf: Vec<u8>,
}

impl CheatState {
    pub fn new() -> Self {
        CheatState {
            no_tilt: false,
            slowdown: false,
            buf: vec![],
        }
    }
}

impl Table {
    pub fn handle_cheat(&mut self, chr: u8) {
        self.cheat.buf.push(chr);
        let mut found_prefix = false;
        for cheat in &self.assets.cheats {
            if self.cheat.buf[..] == cheat.keys[..] {
                self.cheat.buf.clear();
                match cheat.effect {
                    CheatEffect::None => (),
                    CheatEffect::Tilt => self.cheat.no_tilt = true,
                    CheatEffect::Slowdown => self.cheat.slowdown = true,
                    CheatEffect::Balls => self.total_balls = 5,
                    CheatEffect::Reset => {
                        self.cheat.no_tilt = false;
                        self.cheat.slowdown = false;
                        self.total_balls = 3;
                    }
                }
                self.start_script_raw(cheat.script);
                self.script.enter_attract = true;
                return;
            } else if cheat.keys.starts_with(&self.cheat.buf) {
                found_prefix = true;
            }
        }
        if !found_prefix {
            self.cheat.buf = vec![chr];
        }
    }
}
