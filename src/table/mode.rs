use crate::{
    assets::table::{
        dm::DmFont,
        script::{DmCoord, ScriptScore},
    },
    config::TableId,
};

use super::Table;

impl Table {
    pub fn mode_count_hit(&mut self) {
        if self.in_mode_hit {
            self.score_mode_hit += self.assets.score_mode_hit_incr;
        }
    }

    pub fn mode_count_ramp(&mut self) {
        if self.in_mode_ramp {
            match self.assets.table {
                TableId::Table3 => self.score_mode_hit += self.assets.score_mode_ramp_incr,
                _ => self.score_mode_ramp += self.assets.score_mode_ramp_incr,
            }
        }
    }

    pub fn mode_frame(&mut self, score: ScriptScore) -> bool {
        self.dm_put_bcd(
            DmFont::H13,
            DmCoord { x: 16, y: 1 },
            match score {
                ScriptScore::ModeHit => self.score_mode_hit,
                ScriptScore::ModeRamp => self.score_mode_ramp,
                _ => unreachable!(),
            },
            false,
        );
        if self.timer_stop {
            return true;
        }
        self.mode_timeout_frames -= 1;
        if self.mode_timeout_frames != 0 {
            return true;
        }
        self.mode_timeout_frames = if self.hifps { 71 } else { 60 };
        if self.mode_timeout_secs == 0 {
            return false;
        }
        self.mode_timeout_secs -= 1;
        match self.assets.table {
            TableId::Table1 => self.party_mode_check(),
            TableId::Table2 => self.speed_mode_check(),
            TableId::Table3 => self.show_mode_check(),
            TableId::Table4 => self.stones_mode_check(),
        }
        let timeout_msg = [
            if self.mode_timeout_secs < 10 {
                b'_'
            } else {
                b'0' + self.mode_timeout_secs / 10
            },
            b'0' + self.mode_timeout_secs % 10,
        ];
        self.dm_puts(DmFont::H11, DmCoord { x: 144, y: 2 }, &timeout_msg);
        true
    }
}
