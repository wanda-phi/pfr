use rand::{thread_rng, Rng};

use crate::{
    assets::table::{
        dm::DmFont,
        lights::LightBind,
        physics::{Layer, PhysmapBind},
        script::{DmCoord, Effect, EffectBind, EffectSound, ScriptBind},
        sound::{JingleBind, SfxBind},
    },
    bcd::Bcd,
    config::TableId,
};

use super::{
    party::PartyState, show::ShowState, speed::SpeedState, stones::StonesState, tasks::TaskKind,
    KbdState, Table,
};

impl Table {
    pub fn init_game(&mut self) {
        self.kbd_state = KbdState::Main;
        self.script.enter_attract = false;
        self.cur_ball = 1;
        self.cur_player = 1;
        self.got_top_score = false;
        self.got_high_score = false;
        self.in_game_start = true;
        self.score_jackpot = self.assets.score_jackpot_init;
        self.reset_player_state();
        self.extra_balls = 0;
        self.party_on = false;
        self.match_digit = None;
        self.score_main = Bcd::ZERO;
        self.score_bonus = Bcd::ZERO;
        self.num_cyclone = 0;
        self.bcd_num_cyclone = Bcd::ZERO;
        self.score_cyclone_bonus = Bcd::ZERO;
    }

    pub fn reset_player_state(&mut self) {
        self.in_mode = false;
        self.in_mode_hit = false;
        self.in_mode_ramp = false;
        self.score_mode_hit = Bcd::ZERO;
        self.score_mode_ramp = Bcd::ZERO;
        self.bonus_mult_early = 1;
        self.bonus_mult_late = 1;
        self.hold_bonus = false;
        self.lights.reset();
        match self.assets.table {
            TableId::Table1 => {
                self.party = PartyState::new();
                self.light_set_all(LightBind::PartyDuckDrop, true);
                self.light_blink(LightBind::PartyTunnel, 0, 8, 0);
                self.light_blink(LightBind::PartyRightOrbitScore, 0, 9, 0);
                self.raise_physmap(PhysmapBind::PartyHitDuck0);
                self.raise_physmap(PhysmapBind::PartyHitDuck1);
                self.raise_physmap(PhysmapBind::PartyHitDuck2);
            }
            TableId::Table2 => {
                self.speed = SpeedState::new();
            }
            TableId::Table3 => {
                self.show = ShowState::new(self.hifps);
                self.light_set_all(LightBind::ShowDropCenter, true);
                self.light_set_all(LightBind::ShowDropLeft, true);
                self.light_blink(LightBind::ShowSkills, 0, 15, 0);
                self.raise_physmap(PhysmapBind::ShowHitCenter0);
                self.raise_physmap(PhysmapBind::ShowHitCenter1);
                self.raise_physmap(PhysmapBind::ShowHitLeft0);
                self.raise_physmap(PhysmapBind::ShowHitLeft1);
                self.raise_physmap(PhysmapBind::ShowGateRampRight);
                self.raise_physmap(PhysmapBind::ShowGateVaultEntry);
                self.raise_physmap(PhysmapBind::ShowGateVaultExit);
            }
            TableId::Table4 => {
                self.stones = StonesState::new();
                self.raise_physmap(PhysmapBind::StonesGateTowerEntry);
                self.raise_physmap(PhysmapBind::StonesGateKickback);
                let target = thread_rng().gen_range(0..3);
                self.stones.key_skillshot = Some(target);
                self.light_blink(LightBind::StonesKey, target, 1, 0)
            }
        }
    }

    pub fn init_ball(&mut self) {
        self.roll_trigger = None;
        self.at_spring = true;
        self.flipper_pressed = false;
        self.silence_effect = false;
        self.in_drain = false;
        self.in_mode = false;
        self.in_mode_hit = false;
        self.in_mode_ramp = false;
        self.timer_stop = false;
        self.lights.reset();
        self.tasks.clear();
        if !self.special_plunger_event {
            self.dm.stop_blink();
            if self.in_game_start {
                self.in_game_start = false;
            } else {
                self.start_script(ScriptBind::Main);
            }
        }
        self.reset_player_state();
        self.load_cur_player();
        if self.assets.table == TableId::Table1 {
            self.light_set_all(LightBind::PartyDuckDrop, true);
            for bind in [
                PhysmapBind::PartyHitDuck0,
                PhysmapBind::PartyHitDuck1,
                PhysmapBind::PartyHitDuck2,
            ] {
                self.raise_physmap(bind);
            }
            if self.extra_balls != 0 {
                self.light_set(LightBind::PartyExtraBall, 0, true);
            }
        }
    }

    pub fn issue_ball(&mut self) {
        self.in_drain = false;
        self.drained = false;
        self.in_plunger = true;
        self.ball
            .teleport_freeze(Layer::Ground, self.assets.issue_ball_pos);
        if !self.in_game_start && !self.party_on {
            self.play_jingle_plunger();
        } else {
            self.set_music_plunger();
        }
        self.init_ball();
        self.ball_scored_points = false;
        if self.in_game_start {
            self.add_task(TaskKind::IssueBallFinish);
        } else {
            self.issue_ball_finish();
        }
    }

    pub fn issue_ball_finish(&mut self) {
        self.add_task(TaskKind::IssueBallSfx);
        self.add_task(TaskKind::IssueBallRelease);
        if self.assets.sfx_binds[SfxBind::RaiseHitTargets].is_some() {
            self.add_task(TaskKind::IssueBallRaiseSfx);
        }
        self.flippers_enabled = true;
        self.tilted = false;
        self.tilt_counter = 0;
    }

    pub fn issue_ball_release(&mut self) {
        self.ball
            .teleport(Layer::Ground, self.assets.issue_ball_release_pos, (10, 0));
    }

    pub fn abort_game(&mut self) {
        self.ball.teleport(Layer::Ground, (300, 570), (0, 0));
        self.kbd_state = KbdState::Main;
        self.add_task(TaskKind::GameOver);
        self.play_jingle_bind_force(JingleBind::Attract);
        self.dm.stop_blink();
        self.start_script(ScriptBind::Attract);
    }

    pub fn score(&mut self, main: Bcd, bonus: Bcd) {
        self.score_main += main;
        self.score_bonus += bonus;
        self.ball_scored_points = true;
        self.reset_idle();
    }

    pub fn score_premult(&mut self, main: Bcd, bonus: Bcd) {
        self.score_main += main;
        for _ in 0..self.bonus_mult_early {
            self.score_bonus += bonus;
        }
        self.ball_scored_points = true;
        self.reset_idle();
    }

    pub fn effect_force_raw(&mut self, effect: Effect) {
        match effect.sound {
            EffectSound::Jingle(jingle) => {
                self.sequencer.play_jingle(jingle, true, None);
            }
            EffectSound::Silent(_) => (),
        };
        self.score(effect.score_main, effect.score_bonus);
        if let Some(script) = effect.script {
            self.start_script_raw(script);
        }
    }

    pub fn effect_raw(&mut self, effect: Effect) -> bool {
        let present = match effect.sound {
            EffectSound::Jingle(jingle) => {
                if (self.silence_effect || self.in_mode)
                    && jingle.position
                        != self.assets.jingle_binds[JingleBind::Drained]
                            .unwrap()
                            .position
                {
                    false
                } else {
                    self.sequencer.play_jingle(jingle, false, None)
                }
            }
            EffectSound::Silent(priority) => priority >= self.sequencer.priority(),
        };
        self.score(effect.score_main, effect.score_bonus);
        if present {
            if let Some(script) = effect.script {
                self.start_script_raw(script);
            }
        }
        present
    }

    pub fn effect_force(&mut self, effect: EffectBind) {
        self.effect_force_raw(self.assets.effects[effect].unwrap());
    }

    pub fn effect(&mut self, effect: EffectBind) -> bool {
        self.effect_raw(self.assets.effects[effect].unwrap())
    }

    pub fn enter(&mut self) {
        self.start_keys_active = false;
        self.in_game_start = false;
        let jingle = self.assets.jingle_binds[if self.options.no_music {
            JingleBind::Silence
        } else {
            JingleBind::Main
        }]
        .unwrap();
        self.sequencer
            .play_jingle(jingle, true, Some(jingle.position));
        self.start_script(ScriptBind::Main);
        self.in_plunger = false;
        self.at_spring = false;
        self.party_on = false;
        self.special_plunger_event = false;
    }

    pub fn incr_jackpot(&mut self) {
        self.score_jackpot += self.assets.score_jackpot_incr;
    }

    pub fn extra_ball(&mut self) {
        self.extra_balls += 1;
        match self.assets.table {
            TableId::Table1 => {
                self.light_set(LightBind::PartyExtraBall, 0, true);
            }
            TableId::Table2 => {
                self.light_set(LightBind::SpeedExtraBall, 0, true);
            }
            TableId::Table3 => {
                self.light_set(LightBind::ShowExtraBall, 0, true);
            }
            _ => (),
        }
    }

    pub fn add_cyclone(&mut self, cnt: u8) {
        assert!(cnt < 10);
        self.num_cyclone += cnt as u16;
        self.bcd_num_cyclone += Bcd::from_digit(cnt);
        self.score_cyclone_bonus += Bcd::from_bytes([0, 0, 0, 0, 0, 0, cnt, 0, 0, 0, 0, 0]);
        if self.num_cyclone == 1 {
            self.add_cyclone(1);
        }
    }

    pub fn match_done(&mut self, digit: u8) {
        self.match_digit = Some(digit);
        if self
            .players
            .iter()
            .any(|player| player.score_main.digits[10] == digit)
        {
            self.dm.start_blink(3);
            for i in 0..self.players.len() {
                if self.players[i].score_main.digits[10] != digit {
                    self.dm_puts(
                        DmFont::H5,
                        DmCoord {
                            x: i as i16 * 16,
                            y: 0,
                        },
                        b"_",
                    );
                }
            }
            self.play_jingle_bind(JingleBind::MatchWin);
            self.sequencer.reset_priority();
        }
    }
}

#[derive(Debug)]
pub struct ScriptTaskAccBonus {
    frame: i8,
    digit: usize,
    score: Bcd,
}

impl ScriptTaskAccBonus {
    pub fn new(score: Bcd) -> Self {
        Self {
            frame: 0,
            digit: 11,
            score,
        }
    }

    pub fn run(&mut self, table: &mut Table) -> bool {
        self.frame += 1;
        if self.frame != 4 {
            return true;
        }
        self.frame = 0;
        while self.score.digits[self.digit] == 0 {
            if self.digit == 0 {
                table.dm_puts(DmFont::H11, DmCoord { x: -32, y: 6 }, b"___________");
                return false;
            }
            self.digit -= 1;
        }
        self.score.digits[self.digit] -= 1;
        if self.score.digits[self.digit] == 0 && self.score != Bcd::ZERO {
            self.frame = -10;
        }
        let mut delta = Bcd::ZERO;
        delta.digits[self.digit] = 1;
        table.score_main += delta;
        table.play_sfx_bind(SfxBind::TickBonus);
        table.dm_put_bcd(DmFont::H8, DmCoord { x: -32, y: 6 }, self.score, false);
        table.dm_put_bcd(
            DmFont::H13,
            DmCoord { x: 64, y: 1 },
            table.score_main,
            false,
        );
        true
    }
}

#[derive(Debug)]
pub struct ScriptTaskMatch {
    pub count: u16,
    pub frames: u16,
    pub frames_reload: u16,
    pub digit: u8,
}

#[derive(Debug)]
pub struct ScriptTaskMatchStones {
    pub frames: u16,
    pub timing_idx: usize,
    pub digit: u8,
}

impl ScriptTaskMatch {
    pub fn run(&mut self, table: &mut Table) -> bool {
        self.frames -= 1;
        if self.frames != 0 {
            return true;
        }
        self.frames = self.frames_reload;
        table.dm_puts(
            DmFont::H5,
            DmCoord {
                x: self.digit as i16 * 16,
                y: 7,
            },
            b"_",
        );
        let mut new_digit = thread_rng().gen_range(0..10);
        if new_digit == self.digit {
            new_digit += 1;
            if new_digit == 10 {
                new_digit = 0;
            }
        }
        self.digit = new_digit;
        table.dm_puts(
            DmFont::H5,
            DmCoord {
                x: self.digit as i16 * 16,
                y: 7,
            },
            &[b'0' + self.digit],
        );
        self.count -= 1;
        if self.count == 0 {
            table.match_done(self.digit);
            false
        } else {
            true
        }
    }
}

impl ScriptTaskMatchStones {
    pub fn run(&mut self, table: &mut Table) -> bool {
        self.frames -= 1;
        if self.frames != 0 {
            return true;
        }
        self.frames = table.match_timing[self.timing_idx];
        self.timing_idx += 1;
        table.dm_puts(
            DmFont::H5,
            DmCoord {
                x: self.digit as i16 * 16,
                y: 7,
            },
            b"_",
        );
        if self.digit == 0 {
            self.digit = 9
        } else {
            self.digit -= 1;
        }
        table.dm_puts(
            DmFont::H5,
            DmCoord {
                x: self.digit as i16 * 16,
                y: 7,
            },
            &[b'0' + self.digit],
        );
        if self.timing_idx == table.match_timing.len() {
            table.match_done(self.digit);
            false
        } else {
            true
        }
    }
}
