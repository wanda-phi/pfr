use crate::{
    assets::table::{
        lights::LightBind,
        physics::Layer,
        script::{EffectBind, ScriptBind},
        sound::{JingleBind, SfxBind},
    },
    bcd::Bcd,
};

use super::{tasks::TaskKind, Table};

pub struct SpeedState {
    pub blink_bur: [bool; 3],
    pub blink_nin: [bool; 3],
    pub timeout_pit_all: u16,
    pub timeout_pit: [u16; 3],
    pub cur_place: u8,
    pub max_place: u8,
    pub cur_gear: u8,
    pub light_phase_place: u8,
    pub timeout_gear_blink: u16,
    pub timeout_miles_left: u16,
    pub timeout_miles_right: u16,
    pub timeout_jackpot: u16,
    pub mb_active: u8,
    pub mb_pending: u8,
    pub car_mods: u8,
    pub pedal_metal: bool,
    pub cur_speed: u8,
    pub num_cyclone_target_jump: u16,
}

impl SpeedState {
    pub fn new() -> Self {
        Self {
            blink_bur: [false; 3],
            blink_nin: [false; 3],
            timeout_pit_all: 0,
            timeout_pit: [0; 3],
            cur_place: 0,
            max_place: 0,
            cur_gear: 0,
            light_phase_place: 0,
            timeout_gear_blink: 0,
            timeout_miles_left: 0,
            timeout_miles_right: 0,
            timeout_jackpot: 0,
            mb_active: 0,
            mb_pending: 0,
            car_mods: 0,
            pedal_metal: false,
            cur_speed: 0,
            num_cyclone_target_jump: 30,
        }
    }
}

impl Table {
    pub fn speed_frame(&mut self) {
        self.speed.light_phase_place += 1;
        if self.speed.light_phase_place == 30 {
            self.speed.light_phase_place = 0;
        }
        for i in 0..3 {
            if self.speed.timeout_pit[i] != 0 {
                self.speed.timeout_pit[i] -= 1;
                if self.speed.timeout_pit[i] == 0 {
                    self.light_set(LightBind::SpeedPit, i as u8, true);
                }
            }
        }
        if self.speed.timeout_pit_all != 0 {
            self.speed.timeout_pit_all -= 1;
            if self.speed.timeout_pit_all == 0 {
                self.light_set_all(LightBind::SpeedPit, false);
            }
        }
        if self.speed.timeout_miles_left != 0 {
            self.speed.timeout_miles_left -= 1;
        }
        if self.speed.timeout_miles_right != 0 {
            self.speed.timeout_miles_right -= 1;
        }
        if self.speed.timeout_gear_blink != 0 {
            self.speed.timeout_gear_blink -= 1;
            if self.speed.timeout_gear_blink == 0 {
                self.speed.cur_gear = 0;
                self.light_set_all(LightBind::SpeedGearNum, false);
                if !self.light_state(LightBind::SpeedPitStopHoldBonus, 0) {
                    self.light_set(LightBind::SpeedPitStopHoldBonus, 0, true);
                    self.light_blink(
                        LightBind::SpeedPitStopHoldBonus,
                        0,
                        15,
                        self.speed.light_phase_place,
                    );
                }
            }
        }
        if self.speed.timeout_jackpot != 0 {
            self.speed.timeout_jackpot -= 1;
            if self.speed.timeout_jackpot == 0 {
                self.light_set(LightBind::SpeedMiniRampJackpot, 0, false);
            }
        }
    }

    pub fn speed_flipper_pressed(&mut self) {
        self.light_rotate(LightBind::SpeedBur);
        self.speed.blink_bur = [false; 3];
        self.light_rotate(LightBind::SpeedNin);
        self.speed.blink_nin = [false; 3];
        if self.speed.timeout_pit_all == 0 {
            self.speed.timeout_pit = [0; 3];
            self.light_rotate(LightBind::SpeedPit);
        }
    }

    pub fn speed_drained(&mut self) {
        self.sequencer.reset_priority();
        self.effect(EffectBind::Drained);
        self.sequencer.reset_priority();
        self.add_task(TaskKind::DrainSfx);
    }

    pub fn speed_mode_check(&mut self) {
        if self.mode_timeout_secs == 0 {
            if self.in_mode_ramp {
                self.play_jingle_bind(JingleBind::ModeEndRamp);
            }
            if self.in_mode_hit {
                self.play_jingle_bind(JingleBind::ModeEndHit);
            }
            self.sequencer.set_music(3);
            self.sequencer.reset_priority();
            self.in_mode_hit = false;
            self.in_mode_ramp = false;
        }
    }

    pub fn speed_hit_bur(&mut self, which: u8) {
        if self.speed.blink_bur[which as usize] {
            return;
        }
        self.speed.blink_bur[which as usize] = true;
        self.light_set(LightBind::SpeedBur, which, true);
        self.mode_count_hit();
        self.score_premult(Bcd::from_ascii(b"7510"), Bcd::from_ascii(b"550"));
        self.play_sfx_bind(SfxBind::SpeedHitTarget);
        if self.light_all_lit(LightBind::SpeedBur) {
            self.incr_jackpot();
            self.speed.blink_bur = [true; 3];
            for i in 0..3 {
                self.light_blink(LightBind::SpeedBur, i, 1, 0);
            }
            self.add_task(TaskKind::SpeedUnblinkBurAll);
            self.speed_gear(2);
        } else {
            self.light_blink(LightBind::SpeedBur, which, 1, 0);
            self.add_task(TaskKind::SpeedUnblinkBur(which));
        }
    }

    pub fn speed_hit_nin(&mut self, which: u8) {
        if self.speed.blink_nin[which as usize] {
            return;
        }
        self.speed.blink_nin[which as usize] = true;
        self.light_set(LightBind::SpeedNin, which, true);
        self.mode_count_hit();
        self.score_premult(Bcd::from_ascii(b"7510"), Bcd::from_ascii(b"550"));
        self.play_sfx_bind(SfxBind::SpeedHitTarget);
        if self.light_all_lit(LightBind::SpeedNin) {
            self.incr_jackpot();
            self.speed.blink_nin = [true; 3];
            for i in 0..3 {
                self.light_blink(LightBind::SpeedNin, i, 1, 0);
            }
            self.add_task(TaskKind::SpeedUnblinkNinAll);
            self.speed_gear(3);
        } else {
            self.light_blink(LightBind::SpeedNin, which, 1, 0);
            self.add_task(TaskKind::SpeedUnblinkNin(which));
        }
    }

    pub fn speed_gear(&mut self, which: u8) -> bool {
        self.light_set(LightBind::SpeedGear, which, true);
        self.light_blink(LightBind::SpeedGear, which, 1, 0);
        if self.light_all_lit(LightBind::SpeedGear) {
            if self.speed.max_place < 10 {
                self.light_blink(
                    LightBind::SpeedPlace,
                    self.speed.max_place,
                    15,
                    self.speed.light_phase_place,
                );
                self.light_blink(
                    LightBind::SpeedPlace,
                    self.speed.max_place + 1,
                    15,
                    (self.speed.light_phase_place + 15) % 30,
                );
                self.speed.max_place += 2;
            } else {
                self.effect(EffectBind::SpeedExtraGear);
            }
            if self.speed.cur_gear < 5 {
                self.light_set(LightBind::SpeedGearNum, self.speed.cur_gear, true);
                self.speed.cur_gear += 1;
            } else {
                self.speed.timeout_gear_blink = 30;
                for i in 0..6 {
                    self.light_blink(LightBind::SpeedGearNum, i, 1, 0);
                }
            }
            self.light_set_all(LightBind::SpeedGear, false);
            for i in 0..4 {
                self.light_blink(LightBind::SpeedGear, i, 2, 0);
            }
            self.add_task(TaskKind::SpeedUnblinkGearAll);
            self.effect(EffectBind::SpeedGear);
            true
        } else {
            false
        }
    }

    pub fn speed_goal(&mut self) {
        self.light_set_all(LightBind::SpeedPlace, false);
        self.speed.cur_place = 0;
        self.speed.max_place = 0;
        if self.in_mode {
            self.add_task(TaskKind::SpeedTurbo);
        } else {
            self.speed_do_turbo();
        }
    }

    pub fn speed_do_turbo(&mut self) {
        self.effect(EffectBind::SpeedTurbo);
        self.in_mode = true;
        self.in_mode_ramp = true;
    }

    pub fn speed_offroad(&mut self) {
        if self.in_mode {
            self.add_task(TaskKind::SpeedOffroad);
        } else {
            self.speed_do_offroad();
        }
    }

    pub fn speed_do_offroad(&mut self) {
        self.play_jingle_bind_force(JingleBind::SpeedModeHit);
        self.in_mode = true;
        self.in_mode_hit = true;
        self.start_script(ScriptBind::SpeedModeHit);
    }

    pub fn speed_pit_stop(&mut self) {
        if self.light_state(LightBind::SpeedPitStopSuperJackpot, 0) {
            self.light_set(LightBind::SpeedPitStopSuperJackpot, 0, false);
            if self.light_state(LightBind::SpeedPitStopGoal, 0) {
                self.effect(EffectBind::SpeedSuperJackpotGoal);
                self.ball.teleport_freeze(Layer::Ground, (256, 41));
                self.add_task(TaskKind::SpeedPitStop(150));
                return;
            }
            self.timer_stop = true;
            self.effect_force(EffectBind::SpeedSuperJackpot);
        }
        if self.light_state(LightBind::SpeedPitStopGoal, 0) {
            self.light_set(LightBind::SpeedPitStopGoal, 0, false);
            self.speed_goal();
        }
        if self.light_state(LightBind::SpeedPitStopHoldBonus, 0) {
            self.light_set(LightBind::SpeedPitStopHoldBonus, 0, false);
            self.effect(EffectBind::SpeedHoldBonus);
            self.hold_bonus = true;
        }
        self.ball.teleport_freeze(Layer::Ground, (256, 41));
        self.add_task(TaskKind::SpeedPitStop(if self.in_mode { 80 } else { 20 }));
    }

    pub fn speed_car_mod(&mut self, which: u8) {
        if self.light_state(LightBind::SpeedCarPartLit, which) {
            self.light_set(LightBind::SpeedCarPartLit, which, false);
            self.effect(match which {
                0 => EffectBind::SpeedCar0,
                1 => EffectBind::SpeedCar1,
                2 => EffectBind::SpeedCar2,
                3 => EffectBind::SpeedCar3,
                4 => EffectBind::SpeedCar4,
                _ => unreachable!(),
            });
            self.light_set(LightBind::SpeedCarPart, which, true);
            self.light_blink(
                LightBind::SpeedCarPart,
                which,
                15,
                if matches!(which, 0 | 2) {
                    (self.speed.light_phase_place + 15) % 30
                } else {
                    self.speed.light_phase_place
                },
            );
            if self.light_all_lit(LightBind::SpeedCarPart) {
                self.light_set_all(LightBind::SpeedCarPart, false);
                for i in 0..5 {
                    self.light_blink(LightBind::SpeedCarPart, i, 1, 0);
                    self.speed.car_mods = 0;
                    self.add_task(TaskKind::SpeedUnblinkCar);
                }
            }
        }
    }

    pub fn speed_ramp_offroad(&mut self) {
        if self.in_mode_ramp {
            self.effect(EffectBind::SpeedTurboRamp);
        }
        self.mode_count_ramp();
        self.effect(EffectBind::SpeedRampOffroad);
        self.speed_car_mod(0);
        self.speed_car_mod(3);
        if self.light_state(LightBind::SpeedOffroadMultiBonus, 0) {
            self.effect(match self.speed.mb_active {
                0 => EffectBind::SpeedMb2,
                1 => EffectBind::SpeedMb3,
                2 => EffectBind::SpeedMb4,
                3 => EffectBind::SpeedMb5,
                4 => EffectBind::SpeedMb6,
                5 => EffectBind::SpeedMb7,
                6 => EffectBind::SpeedMb8,
                7 => EffectBind::SpeedMb9,
                _ => unreachable!(),
            });
            self.light_set(LightBind::SpeedBonus, self.speed.mb_active, true);
            self.speed.mb_active += 1;
            self.bonus_mult_early = self.speed.mb_active + 1;
            self.bonus_mult_late = self.speed.mb_active + 1;
            self.speed.mb_pending -= 1;
            if self.speed.mb_pending == 0 {
                self.light_set(LightBind::SpeedOffroadMultiBonus, 0, false);
            }
        }
        self.incr_jackpot();
        if !self.speed_gear(1) {
            self.add_task(TaskKind::SpeedUnblinkGear(1));
        }
    }

    pub fn speed_ramp_jump(&mut self) {
        if self.in_mode_ramp {
            self.effect(EffectBind::SpeedTurboRamp);
        }
        self.mode_count_ramp();
        self.incr_jackpot();
        if self.light_state(LightBind::SpeedMiniRampJackpot, 0) {
            self.score_main += self.score_jackpot;
            self.score_jackpot = self.assets.score_jackpot_init;
            if self.in_mode_ramp {
                self.effect_force(EffectBind::SpeedJackpot);
                self.timer_stop = true;
            } else {
                self.effect(EffectBind::SpeedJackpot);
            }
            self.light_set(LightBind::SpeedMiniRampJackpot, 0, false);
            self.light_set(LightBind::SpeedPitStopSuperJackpot, 0, true);
            self.light_blink(
                LightBind::SpeedPitStopSuperJackpot,
                0,
                15,
                self.speed.light_phase_place,
            );
            self.add_task(TaskKind::SpeedResetSuperJackpot);
        }
        if self.light_state(LightBind::SpeedMiniRampJump, 0) {
            self.effect(EffectBind::SpeedJump);
            self.light_set(LightBind::SpeedMiniRampJump, 0, false);
        }
        self.speed_car_mod(1);
        if self.speed.pedal_metal {
            self.speed.pedal_metal = false;
            self.effect(EffectBind::SpeedPedalMetal);
            if (self.speed.cur_speed & 1) == 1 && self.speed.car_mods != 5 {
                self.light_set(LightBind::SpeedCarPartLit, self.speed.car_mods, true);
                self.light_blink(
                    LightBind::SpeedCarPartLit,
                    self.speed.car_mods,
                    15,
                    self.speed.light_phase_place,
                );
                self.speed.car_mods += 1;
            }
            if self.speed.cur_speed < 12 {
                self.light_set(LightBind::SpeedSpeed, self.speed.cur_speed, true);
            }
            self.speed.cur_speed += 1;
            if self.speed.cur_speed == 14 {
                self.speed.cur_speed = 12;
            }
        }
        if !self.speed_gear(0) {
            self.add_task(TaskKind::SpeedUnblinkGear(0));
        }
        self.incr_jackpot();
    }

    pub fn speed_pit_loop(&mut self) {
        self.speed_car_mod(2);
        self.speed_car_mod(4);
        if self.light_state(LightBind::SpeedPitLoopExtraBall, 0) {
            self.effect(EffectBind::SpeedExtraBall);
            self.light_set(LightBind::SpeedPitLoopExtraBall, 0, false);
            self.extra_ball();
        }
    }

    pub fn speed_roll_pit(&mut self, which: u8) {
        if self.speed.timeout_pit[which as usize] != 0 {
            return;
        }
        self.light_set(LightBind::SpeedPit, which, true);
        self.mode_count_hit();
        self.play_sfx_bind(SfxBind::SpeedHitTarget);
        self.effect(EffectBind::SpeedPit);
        if self.light_all_lit(LightBind::SpeedPit) && self.speed.timeout_pit_all == 0 {
            self.effect(EffectBind::SpeedPitAll);
            if self.speed.mb_active + self.speed.mb_pending < 8 {
                if self.speed.mb_pending == 0 {
                    self.light_set(LightBind::SpeedOffroadMultiBonus, 0, true);
                    self.light_blink(LightBind::SpeedOffroadMultiBonus, 0, 10, 0);
                }
                self.speed.mb_pending += 1;
            } else {
                self.effect(EffectBind::SpeedMillion);
            }
            for i in 0..3 {
                self.light_blink(LightBind::SpeedPit, i, 2, 0);
            }
            self.speed.timeout_pit_all = 40;
        } else {
            self.light_blink(LightBind::SpeedPit, which, 1, 0);
            self.speed.timeout_pit[which as usize] = 20;
        }
    }

    pub fn speed_overtake(&mut self) {
        if self.speed.cur_place < self.speed.max_place {
            self.light_set(LightBind::SpeedPlace, self.speed.cur_place, true);
            self.speed.cur_place += 1;
            self.effect(EffectBind::SpeedOvertake);
            if self.speed.cur_place == 10 {
                self.light_set(LightBind::SpeedPitStopGoal, 0, true);
                self.light_blink(
                    LightBind::SpeedPitStopGoal,
                    0,
                    15,
                    (self.speed.light_phase_place + 15) % 30,
                );
                self.light_set(LightBind::SpeedMiniRampJackpot, 0, true);
                self.light_blink(
                    LightBind::SpeedMiniRampJackpot,
                    0,
                    15,
                    self.speed.light_phase_place,
                );
                self.speed.timeout_jackpot = 1200;
                self.effect(EffectBind::SpeedOvertakeFinal);
            }
        }
        self.effect(EffectBind::SpeedMillion);
        self.speed.pedal_metal = true;
    }

    pub fn speed_bump_miles(&mut self) {
        self.incr_jackpot();
        self.effect(match self.speed.cur_speed {
            0 => EffectBind::SpeedMiles0,
            1 => EffectBind::SpeedMiles1,
            2 => EffectBind::SpeedMiles2,
            3 => EffectBind::SpeedMiles3,
            4 => EffectBind::SpeedMiles4,
            5 => EffectBind::SpeedMiles5,
            6 => EffectBind::SpeedMiles6,
            7 => EffectBind::SpeedMiles7,
            8 => EffectBind::SpeedMiles8,
            9 => EffectBind::SpeedMiles9,
            10 => EffectBind::SpeedMiles10,
            _ => EffectBind::SpeedMiles11,
        });
        if self.in_mode_ramp {
            self.effect(EffectBind::SpeedTurboRamp);
        }
        self.mode_count_ramp();
        self.add_cyclone(1);
        self.num_cyclone_target = self.num_cyclone / 10 * 10 + 10;
        match (self.num_cyclone, self.num_cyclone % 20) {
            (0..=9, _) => {
                self.effect(EffectBind::SpeedMilesToFirstOffroad);
            }
            (10, _) => self.speed_offroad(),
            (11..=19, _) => {
                self.effect(EffectBind::SpeedMilesToExtraBall);
            }
            (20, _) => {
                self.light_set(LightBind::SpeedPitLoopExtraBall, 0, true);
                self.light_blink(LightBind::SpeedPitLoopExtraBall, 0, 15, 0);
                self.effect(EffectBind::SpeedMilesExtraBall);
            }
            (_, 1..=9) => {
                self.effect(EffectBind::SpeedMilesToJump);
            }
            (_, 10) => {
                if !self.light_state(LightBind::SpeedMiniRampJump, 0) {
                    self.light_set(LightBind::SpeedMiniRampJump, 0, true);
                    self.light_blink(LightBind::SpeedMiniRampJump, 0, 15, 0);
                    self.effect(EffectBind::SpeedMilesJump);
                }
            }
            (_, 11..=19) => {
                self.effect(EffectBind::SpeedMilesToOffroad);
            }
            (_, 0) => self.speed_offroad(),

            _ => unreachable!(),
        }
    }

    pub fn speed_load_fixup(&mut self) {
        if self.light_state(LightBind::SpeedPitStopGoal, 0) {
            self.light_blink(
                LightBind::SpeedPitStopGoal,
                0,
                15,
                (self.speed.light_phase_place + 15) % 30,
            );
        }
        for i in 0..self.speed.cur_gear {
            self.light_set(LightBind::SpeedGearNum, i, true);
        }
        for i in 0..5 {
            if self.light_state(LightBind::SpeedCarPart, i) {
                self.light_blink(
                    LightBind::SpeedCarPart,
                    i,
                    15,
                    if matches!(i, 0 | 2) {
                        (self.speed.light_phase_place + 15) % 30
                    } else {
                        self.speed.light_phase_place
                    },
                );
            }
        }
        for i in 0..5 {
            if self.light_state(LightBind::SpeedCarPartLit, i) {
                self.light_blink(
                    LightBind::SpeedCarPartLit,
                    i,
                    15,
                    self.speed.light_phase_place,
                );
            }
        }
        for i in 0..self.speed.cur_place {
            self.light_set(LightBind::SpeedPlace, i, true);
        }
        for i in self.speed.cur_place..self.speed.max_place {
            self.light_blink(
                LightBind::SpeedPlace,
                i,
                15,
                if i % 2 == 0 {
                    self.speed.light_phase_place
                } else {
                    (self.speed.light_phase_place + 15) % 30
                },
            );
        }
        for i in 0..self.speed.cur_speed {
            if i < 12 {
                self.light_set(LightBind::SpeedSpeed, i, true);
            }
        }
    }
}
