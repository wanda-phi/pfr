use rand::{thread_rng, Rng};

use crate::{
    assets::table::{
        lights::LightBind,
        physics::{Layer, PhysmapBind, RollTrigger},
        script::{EffectBind, ScriptBind},
        sound::{JingleBind, SfxBind},
    },
    bcd::Bcd,
    config::Resolution,
};

use super::{tasks::TaskKind, Table};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PrizeState {
    None,
    Lit,
    Taken,
}

pub struct ShowState {
    pub score_cashpot: Bcd,
    pub prizes: [PrizeState; 6],
    pub prize_sets: u8,
    pub timeout_wheel_tick: u16,
    pub timeout_mb: u16,
    pub timeout_top_loop: u16,
    pub timeout_tv: u16,
    pub timeout_trip: u16,
    pub timeout_car: u16,
    pub timeout_boat: u16,
    pub timeout_house: u16,
    pub timeout_plane: u16,
    pub timeout_cashpot_x5: u16,
    pub timeout_jackpot: u16,
    pub timeout_super_jackpot: u16,
    pub billion_lit: bool,
    pub light_phase_prize: u8,
    pub wheel_cycle: usize,
    pub wheel_pos: u8,
    pub wheel_timing: &'static [u16],
}

impl ShowState {
    pub fn new(hifps: bool) -> Self {
        Self {
            score_cashpot: Bcd::from_ascii(b"500000"),
            prizes: [PrizeState::None; 6],
            prize_sets: 0,
            timeout_wheel_tick: 0,
            timeout_mb: 0,
            timeout_top_loop: 0,
            timeout_tv: 0,
            timeout_trip: 0,
            timeout_car: 0,
            timeout_boat: 0,
            timeout_house: 0,
            timeout_plane: 0,
            timeout_cashpot_x5: 0,
            timeout_jackpot: 0,
            timeout_super_jackpot: 0,
            billion_lit: false,
            light_phase_prize: 0,
            wheel_cycle: 0,
            wheel_pos: 0,
            wheel_timing: if hifps {
                &[
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 6, 7, 7, 7, 7, 8, 8, 8, 9,
                    10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 14, 16, 19, 22, 25, 50,
                ]
            } else {
                &[
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8,
                    8, 8, 8, 9, 9, 10, 10, 12, 14, 17, 20, 24, 32, 47,
                ]
            },
        }
    }
}

impl Table {
    pub fn show_frame(&mut self) {
        if self.show.timeout_super_jackpot != 0 {
            self.show.timeout_super_jackpot -= 1;
            if self.show.timeout_super_jackpot == 0 {
                self.light_set(LightBind::ShowSuperJackpot, 0, false);
            } else if self.show.timeout_super_jackpot == 120 {
                self.light_blink(LightBind::ShowSuperJackpot, 0, 2, 0);
            }
        }
        if self.show.timeout_jackpot != 0 {
            self.show.timeout_jackpot -= 1;
            if self.show.timeout_jackpot == 0 {
                self.light_set(LightBind::ShowJackpot, 0, false);
            } else if self.show.timeout_jackpot == 120 {
                self.light_blink(LightBind::ShowJackpot, 0, 2, 0);
            }
        }
        self.show.light_phase_prize += 1;
        if self.show.light_phase_prize == 20 {
            self.show.light_phase_prize = 0;
        }
        if self.show.timeout_wheel_tick != 0 {
            self.show.timeout_wheel_tick -= 1;
            if self.show.timeout_wheel_tick == 0 {
                self.show_wheel_tick();
            }
        }
        if self.show.timeout_mb != 0 {
            self.show.timeout_mb -= 1;
        }
        if self.show.timeout_tv != 0 {
            self.show.timeout_tv -= 1;
        }
        if self.show.timeout_trip != 0 {
            self.show.timeout_trip -= 1;
        }
        if self.show.timeout_car != 0 {
            self.show.timeout_car -= 1;
        }
        if self.show.timeout_boat != 0 {
            self.show.timeout_boat -= 1;
        }
        if self.show.timeout_house != 0 {
            self.show.timeout_house -= 1;
        }
        if self.show.timeout_plane != 0 {
            self.show.timeout_plane -= 1;
        }
        if self.show.timeout_cashpot_x5 != 0 {
            self.show.timeout_cashpot_x5 -= 1;
            if self.show.timeout_cashpot_x5 == 0 {
                self.light_set(LightBind::ShowCashpotX5, 0, false);
            } else if self.show.timeout_cashpot_x5 == 120 {
                self.light_blink(LightBind::ShowCashpotX5, 0, 2, 0)
            }
        }
        if self.show.timeout_top_loop != 0 {
            self.show.timeout_top_loop -= 1;
            if self.show.timeout_top_loop == 0 {
                self.light_set(LightBind::ShowTopLoop, 0, false);
            } else if self.show.timeout_top_loop == 120 {
                self.light_blink(LightBind::ShowTopLoop, 0, 3, 0)
            }
        }
    }

    pub fn show_flipper_pressed(&mut self) {}

    pub fn show_drained(&mut self) {
        self.effect(EffectBind::Drained);
        self.set_music_silence();
        self.add_task(TaskKind::DrainSfx);
    }

    pub fn show_mode_check(&mut self) {
        if self.mode_timeout_secs == 0 {
            self.play_jingle_bind(JingleBind::ModeEndHit);
            self.sequencer.set_music(3);
            self.sequencer.reset_priority();
            self.in_mode_hit = false;
            self.in_mode_ramp = false;
        }
    }

    pub fn show_hit_center(&mut self, which: u8) {
        self.mode_count_hit();
        if !self.light_state(LightBind::ShowDropCenter, which) {
            return;
        }
        self.effect(EffectBind::ShowDropCenter);
        self.play_sfx_bind(SfxBind::ShowHitTrigger);
        self.drop_physmap(match which {
            0 => PhysmapBind::ShowHitCenter0,
            1 => PhysmapBind::ShowHitCenter1,
            _ => unreachable!(),
        });
        self.light_set(LightBind::ShowDropCenter, which, false);
        if self.light_all_unlit(LightBind::ShowDropCenter) {
            self.add_task(TaskKind::ShowResetDropCenter);
        }
    }

    pub fn show_hit_left(&mut self, which: u8) {
        self.mode_count_hit();
        if !self.light_state(LightBind::ShowDropLeft, which) {
            return;
        }
        self.effect(EffectBind::ShowDropLeft);
        self.play_sfx_bind(SfxBind::ShowHitTrigger);
        self.drop_physmap(match which {
            0 => PhysmapBind::ShowHitLeft0,
            1 => PhysmapBind::ShowHitLeft1,
            _ => unreachable!(),
        });
        self.light_set(LightBind::ShowDropLeft, which, false);
        if self.light_all_unlit(LightBind::ShowDropLeft) {
            self.add_task(TaskKind::ShowResetDropLeft);
        }
    }

    pub fn show_hit_dollar(&mut self, which: u8) {
        self.mode_count_hit();
        self.play_sfx_bind(SfxBind::ShowHitTrigger);
        self.light_set(LightBind::ShowDollar, which, true);
        if self.light_all_lit(LightBind::ShowDollar) {
            for i in 0..2 {
                self.light_blink(LightBind::ShowDollar, i, 2, 0);
            }
            self.effect(EffectBind::ShowDollarBoth);
            self.add_task(TaskKind::ShowUnblinkDollarAll);
            self.light_set(LightBind::ShowSpinWheel, 0, true);
            self.light_blink(
                LightBind::ShowSpinWheel,
                0,
                10,
                (self.show.light_phase_prize + 10) % 20,
            );
            self.drop_physmap(PhysmapBind::ShowGateVaultEntry);
        } else {
            self.light_blink(LightBind::ShowDollar, which, 6, 0);
            self.effect(EffectBind::ShowDollar);
            self.add_task(TaskKind::ShowUnblinkDollar(which));
        }
    }

    pub fn show_vault(&mut self) {
        self.drop_physmap(PhysmapBind::ShowGateVaultExit);
        self.ball.teleport_freeze(Layer::Ground, (4, 529));
        if self.in_mode || self.tilted {
            self.add_task(TaskKind::ShowVaultEject);
        } else if self.show.billion_lit {
            self.show.billion_lit = false;
            self.effect(EffectBind::ShowBillion);
            self.light_blink(LightBind::ShowBillion, 0, 4, 0);
            self.add_task(TaskKind::ShowBillionRelease);
        } else {
            if !self.light_state(LightBind::ShowCollectPrize, 0) {
                self.raise_physmap(PhysmapBind::ShowGateVaultEntry);
            }
            self.play_jingle_bind(JingleBind::ShowSpinWheel);
            self.show.wheel_cycle = 0;
            self.show.timeout_wheel_tick = self.show.wheel_timing[0];
            self.start_script(ScriptBind::ShowSpinWheelClearHalt);
            self.light_set_all(LightBind::ShowWheel, false);
            let target: u8 = if !self.light_state(LightBind::ShowCollectPrize, 0) {
                thread_rng().gen_range(0..8)
            } else if self.show.prizes[0] == PrizeState::Lit {
                0
            } else if self.show.prizes[1] == PrizeState::Lit {
                1
            } else if self.show.prizes[2] == PrizeState::Lit {
                2
            } else if self.show.prizes[3] == PrizeState::Lit {
                6
            } else if self.show.prizes[4] == PrizeState::Lit {
                5
            } else if self.show.prizes[5] == PrizeState::Lit {
                4
            } else {
                unreachable!()
            };
            self.show.wheel_pos = target.wrapping_sub(self.show.wheel_timing.len() as u8) & 7;
            self.scroll
                .set_special_target(match self.options.resolution {
                    Resolution::Normal => 270,
                    Resolution::High => 220,
                    Resolution::Full => 0,
                });
        }
    }

    pub fn show_wheel_tick(&mut self) {
        self.show.wheel_cycle += 1;
        if self.show.wheel_cycle == self.show.wheel_timing.len() {
            self.start_script(ScriptBind::ShowSpinWheelBlink);
            if self.light_state(LightBind::ShowCollectPrize, 0) {
                self.add_task(TaskKind::ShowGivePrize);
            } else {
                self.add_task(TaskKind::ShowSpinWheelEnd);
            }
        } else {
            self.show.timeout_wheel_tick = self.show.wheel_timing[self.show.wheel_cycle];
            self.light_set_all(LightBind::ShowWheel, false);
            self.show.wheel_pos += 1;
            self.show.wheel_pos %= 8;
            self.light_set(LightBind::ShowWheel, self.show.wheel_pos, true);
            self.start_script(ScriptBind::ShowSpinWheelScore);
        }
    }

    pub fn show_give_prize(&mut self) {
        for i in 0..6 {
            if self.show.prizes[i as usize] != PrizeState::Taken {
                self.show.prizes[i as usize] = PrizeState::Taken;
                self.light_set(LightBind::ShowPrize, i, true);
                self.sequencer.reset_priority();
                self.effect(
                    [
                        EffectBind::ShowPrizeTv,
                        EffectBind::ShowPrizeTrip,
                        EffectBind::ShowPrizeCar,
                        EffectBind::ShowPrizeBoat,
                        EffectBind::ShowPrizeHouse,
                        EffectBind::ShowPrizePlane,
                    ][i as usize],
                );
                if i == 2 {
                    self.light_set(LightBind::ShowCollectPrize, 0, false);
                    self.raise_physmap(PhysmapBind::ShowGateVaultEntry);
                    self.light_set(LightBind::ShowJackpot, 0, true);
                    self.light_blink(LightBind::ShowJackpot, 0, 10, self.show.light_phase_prize);
                    self.show.timeout_jackpot = 1500;
                    self.show.prize_sets = 1;
                } else if i == 5 {
                    self.light_set(LightBind::ShowCollectPrize, 0, false);
                    self.raise_physmap(PhysmapBind::ShowGateVaultEntry);
                    self.show.prize_sets = 2;
                }
                return;
            }
        }
        unreachable!();
    }

    pub fn show_wheel_score(&self) -> Bcd {
        [
            Bcd::from_ascii(b"25000"),
            Bcd::from_ascii(b"50000"),
            Bcd::from_ascii(b"100000"),
            Bcd::from_ascii(b"250000"),
            Bcd::from_ascii(b"500000"),
            Bcd::from_ascii(b"1000000"),
            Bcd::from_ascii(b"2500000"),
            Bcd::from_ascii(b"5000000"),
        ][self.show.wheel_pos as usize]
    }

    pub fn show_cashpot(&mut self) {
        if self.in_mode {
            self.ball.teleport_freeze(Layer::Ground, (103, 233));
            self.light_set(LightBind::ShowCashpot, 0, true);
            self.add_task(TaskKind::ShowCashpotEject(30));
        } else if self.show.prize_sets == 2 {
            self.light_set(LightBind::ShowCashpot, 0, true);
            self.light_blink(LightBind::ShowBillion, 0, 10, self.show.light_phase_prize);
            self.show.billion_lit = true;
            self.effect(EffectBind::ShowCashpotLock);
            self.sequencer.set_music(0);
            self.sequencer.reset_priority();
            self.ball.teleport(Layer::Ground, (304, 535), (10, 0));
            self.drop_physmap(PhysmapBind::ShowGateVaultEntry);
        } else {
            self.incr_jackpot();
            if self.show.timeout_cashpot_x5 != 0 {
                self.show.timeout_cashpot_x5 = 10;
                let mut effect = self.assets.effects[EffectBind::ShowCashpotX5].unwrap();
                for _ in 0..5 {
                    effect.score_main += self.show.score_cashpot;
                }
                self.effect_raw(effect);
            } else {
                let mut effect = self.assets.effects[EffectBind::ShowCashpot].unwrap();
                effect.score_main = self.show.score_cashpot;
                self.effect_raw(effect);
            }
            self.ball.teleport_freeze(Layer::Ground, (103, 233));
            self.add_task(TaskKind::ShowCashpot);
        }
    }

    pub fn show_cashpot_eject(&mut self) {
        self.play_sfx_bind(SfxBind::ShowEjectCashpot);
        self.light_set(LightBind::ShowCashpot, 0, false);
        self.ball.teleport(Layer::Ground, (103, 233), (83, 1416));
    }

    pub fn show_ramp_right(&mut self) {
        self.show.score_cashpot += Bcd::from_ascii(b"7130");
        self.mode_count_ramp();
        self.effect(EffectBind::ShowRampRight);
        self.show.timeout_tv = 240;
        self.show.timeout_car = 240;
        self.drop_physmap(PhysmapBind::ShowGateRampRight);
        if self.show.timeout_jackpot != 0 {
            self.show.timeout_jackpot = 1;
            let mut effect = self.assets.effects[EffectBind::ShowJackpot].unwrap();
            effect.score_main = self.score_jackpot;
            self.effect_raw(effect);
            self.score_jackpot = self.assets.score_jackpot_init;
            self.show.timeout_super_jackpot = 300;
            self.light_blink(LightBind::ShowSuperJackpot, 0, 10, 0);
            self.play_jingle_bind(JingleBind::ShowJackpot);
        }
    }

    fn show_lit_prize(&mut self, which: u8) {
        self.show.prizes[which as usize] = PrizeState::Lit;
        self.effect(
            [
                EffectBind::ShowLitTv,
                EffectBind::ShowLitTrip,
                EffectBind::ShowLitCar,
                EffectBind::ShowLitBoat,
                EffectBind::ShowLitHouse,
                EffectBind::ShowLitPlane,
            ][which as usize],
        );
        self.light_blink(
            LightBind::ShowPrize,
            which,
            10,
            if matches!(which, 1 | 4) {
                (self.show.light_phase_prize + 10) % 20
            } else {
                self.show.light_phase_prize
            },
        );
        if (self.show.prizes[0] == PrizeState::Lit
            && self.show.prizes[1] == PrizeState::Lit
            && self.show.prizes[2] == PrizeState::Lit)
            || (self.show.prizes[3] == PrizeState::Lit
                && self.show.prizes[4] == PrizeState::Lit
                && self.show.prizes[5] == PrizeState::Lit)
        {
            self.light_set(LightBind::ShowCollectPrize, 0, true);
            self.light_blink(
                LightBind::ShowCollectPrize,
                0,
                10,
                (self.show.light_phase_prize + 10) % 20,
            );
            self.drop_physmap(PhysmapBind::ShowGateVaultEntry);
        }
    }

    pub fn show_ramp_loop(&mut self) {
        self.show.score_cashpot += Bcd::from_ascii(b"7130");
        self.incr_jackpot();
        self.mode_count_ramp();
        self.effect(EffectBind::ShowRampLoop);
        if self.show.timeout_super_jackpot != 0 {
            self.show.timeout_super_jackpot = 1;
            self.effect(EffectBind::ShowSuperJackpot);
        }
        self.show.timeout_cashpot_x5 = 660;
        self.light_blink(
            LightBind::ShowCashpotX5,
            0,
            10,
            (self.show.light_phase_prize + 10) % 20,
        );
        if self.show.timeout_car != 0 {
            self.show.timeout_car = 0;
            if self.show.prizes[2] == PrizeState::None {
                self.show_lit_prize(2);
            } else if self.show.prizes[5] == PrizeState::None && self.show.prize_sets == 1 {
                self.show.timeout_plane = 600;
                if !self.in_mode {
                    self.start_script(ScriptBind::ShowHintLoopLeft);
                }
            } else {
                self.score_raising_millions += Bcd::from_ascii(b"1000000");
                let mut effect = self.assets.effects[EffectBind::ShowRaisingMillions].unwrap();
                effect.score_main = self.score_raising_millions;
                self.effect_raw(effect);
            }
        }
        if self.show.timeout_mb != 0 {
            let which = self.light_sequence(LightBind::ShowBonus);
            if which < 6 {
                if self.play_jingle_bind(JingleBind::ShowMultiBonus) {
                    self.start_script(
                        [
                            ScriptBind::ShowMbX2,
                            ScriptBind::ShowMbX3,
                            ScriptBind::ShowMbX4,
                            ScriptBind::ShowMbX6,
                            ScriptBind::ShowMbX8,
                            ScriptBind::ShowMbX10,
                        ][which as usize],
                    );
                }
                self.bonus_mult_late = [2, 3, 4, 6, 8, 10][which as usize];
            }
        }
    }

    pub fn show_orbit_left(&mut self) {
        self.effect(EffectBind::ShowOrbitLeft);
        if self.prev_roll_trigger != Some(RollTrigger::ShowOrbitRight) {
            return;
        }
        if self.show.timeout_boat != 0 && self.show.prizes[3] == PrizeState::None {
            self.show.timeout_boat = 0;
            self.show_lit_prize(3);
        }
        if self.show.timeout_house != 0 && self.show.prizes[4] == PrizeState::None {
            self.show.timeout_house = 0;
            self.show_lit_prize(4);
        }
    }

    pub fn show_orbit_right(&mut self) {
        self.mode_count_ramp();
        self.effect(EffectBind::ShowOrbitRight);
        if self.prev_roll_trigger != Some(RollTrigger::ShowOrbitLeft) {
            return;
        }
        if self.light_state(LightBind::ShowOrbitExtraBall, 0) {
            self.light_set(LightBind::ShowOrbitExtraBall, 0, false);
            self.effect(EffectBind::ShowExtraBall);
            self.extra_ball();
        }
        if self.show.timeout_plane != 0 && self.show.prizes[5] == PrizeState::None {
            self.show.timeout_plane = 0;
            self.show_lit_prize(5);
        } else {
            self.raise_physmap(PhysmapBind::ShowGateRampRight);
            self.show.timeout_mb = 240;
            self.show.timeout_trip = 240;
        }
    }

    pub fn show_ramp_skills(&mut self) {
        self.effect(EffectBind::ShowRampSkills);
        self.incr_jackpot();
        self.show.score_cashpot += Bcd::from_ascii(b"7130");
        self.mode_count_ramp();
        self.add_cyclone(1);
        self.num_cyclone_target = self.num_cyclone / 6 * 6 + 6;
        match (self.num_cyclone, self.num_cyclone % 12) {
            (0..=5, _) => {
                self.effect(EffectBind::ShowSkillsToMoneyMania);
            }
            (6, _) => {
                self.effect(EffectBind::ShowModeHit);
                self.in_mode = true;
                self.in_mode_hit = true;
                self.light_set(LightBind::ShowMoneyMania, 0, true);
            }
            (7..=11, _) => {
                self.effect(EffectBind::ShowSkillsToExtraBall);
            }
            (12, _) => {
                self.play_jingle_bind(JingleBind::ShowExtraBallLit);
                self.light_set(LightBind::ShowOrbitExtraBall, 0, true);
                self.light_blink(
                    LightBind::ShowOrbitExtraBall,
                    0,
                    15,
                    (self.show.light_phase_prize + 10) % 20,
                );
            }
            (_, 6) => {
                self.effect(EffectBind::ShowModeRamp);
                self.in_mode = true;
                self.in_mode_ramp = true;
                self.light_set(LightBind::ShowMoneyMania, 0, true);
            }
            (_, 0) => {
                self.effect(EffectBind::ShowModeHit);
                self.in_mode = true;
                self.in_mode_hit = true;
                self.light_set(LightBind::ShowMoneyMania, 0, true);
            }
            _ => {
                self.effect(EffectBind::ShowSkillsToMoneyMania);
            }
        }
        if self.show.timeout_tv != 0 {
            if self.show.prizes[0] == PrizeState::None {
                self.show.timeout_tv = 0;
                self.show_lit_prize(0);
            } else if self.show.prizes[3] == PrizeState::None && self.show.prize_sets == 1 {
                self.show.timeout_boat = 600;
                if !self.in_mode {
                    self.start_script(ScriptBind::ShowHintLoopRight);
                }
            }
        }
        if self.show.timeout_trip != 0 {
            if self.show.prizes[1] == PrizeState::None {
                self.show.timeout_trip = 0;
                self.show_lit_prize(1);
            } else if self.show.prizes[4] == PrizeState::None && self.show.prize_sets == 1 {
                self.show.timeout_house = 600;
                if !self.in_mode {
                    self.start_script(ScriptBind::ShowHintLoopRight);
                }
            }
        }
    }

    pub fn show_ramp_top(&mut self) {
        self.show.score_cashpot += Bcd::from_ascii(b"7130");
        self.effect(EffectBind::ShowRampTop);
        if self.show.timeout_top_loop != 0 {
            self.effect(EffectBind::ShowRampTopTwice);
        }
        self.show.timeout_top_loop = 600;
        if self.light_state(LightBind::ShowCollectPrize, 0) {
            self.play_jingle_bind(JingleBind::ShowPrizeIncoming);
        }
        self.light_blink(LightBind::ShowTopLoop, 0, 10, self.show.light_phase_prize);
    }
}
