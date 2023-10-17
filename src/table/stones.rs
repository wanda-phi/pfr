use crate::{
    assets::table::{
        lights::LightBind,
        physics::{Layer, PhysmapBind},
        script::EffectBind,
        sound::{JingleBind, SfxBind},
    },
    bcd::Bcd,
};

use super::{tasks::TaskKind, Table};

pub struct StonesState {
    pub flipper_lock_key: bool,
    pub flipper_lock_rip: bool,

    pub cur_ghost: u8,
    pub ghost_active: bool,
    pub vault_from_ramp: bool,
    pub in_vault: bool,
    pub vault_hold: bool, // TODO: wtf

    pub bone_blinking: [bool; 4],
    pub stone_blinking: [bool; 5],
    pub stones_bones_blinking: bool,
    pub ghosts_blinking: bool,

    pub ball_locked: bool,

    pub million_plus: bool,
    pub score_million_plus: Bcd,
    pub score_skill_shot: Bcd,
    pub key_blinking: bool,
    pub key_skillshot: Option<u8>,
    pub key_tower_cycle: u8,

    pub in_tower: bool,
    pub tower_open: bool,
    pub tower_extra_ball: bool,
    pub tower_jackpot: bool,
    pub tower_super_jackpot: bool,
    pub tower_1m: bool,
    pub tower_5m: bool,
    pub tower_double_bonus: bool,
    pub tower_hold_bonus: bool,
    pub tower_hunt: bool,
    pub tower_hunt_ctr: u8,
    pub tower_resume_mode: bool,
    pub tower_resume_mode_ramp: bool,

    pub scream_x2: bool,
    pub scream_demon: bool,
    pub in_well: bool,
    pub well_multi_bonus: bool,
    pub loop_combo: u8,
    pub kickback: bool,
    pub rip_blinking: bool,

    pub lock_ready: bool,
    pub lock_well_ready: bool,
    pub lock_vault_ready: bool,

    pub light_phase_right: u8,
    pub light_phase_tower: u8,

    pub timeout_top_loop: u16,
    pub timeout_left_ramp: u16,
    pub timeout_multi_bonus: u16,
    pub timeout_loop_combo: u16,
    pub timeout_tower_hunt: u16,
    pub timeout_lock: u16,

    pub score_vault: Bcd,
    pub score_well: Bcd,
    pub score_tower_bonus: Bcd,
}

impl StonesState {
    pub fn new() -> Self {
        Self {
            flipper_lock_key: false,
            flipper_lock_rip: false,

            cur_ghost: 0,
            ghost_active: false,
            vault_from_ramp: false,
            in_vault: false,
            vault_hold: false,

            bone_blinking: [false; 4],
            stone_blinking: [false; 5],
            stones_bones_blinking: false,
            ghosts_blinking: false,

            ball_locked: false,

            million_plus: false,
            score_million_plus: Bcd::ZERO,
            score_skill_shot: Bcd::ZERO,
            key_blinking: false,
            key_skillshot: None,
            key_tower_cycle: 0,

            in_tower: false,
            tower_open: false,
            tower_extra_ball: false,
            tower_jackpot: false,
            tower_super_jackpot: false,
            tower_1m: false,
            tower_5m: false,
            tower_double_bonus: false,
            tower_hold_bonus: false,
            tower_hunt: false,
            tower_hunt_ctr: 0,
            tower_resume_mode: false,
            tower_resume_mode_ramp: false,

            scream_x2: false,
            scream_demon: false,
            in_well: false,
            well_multi_bonus: false,
            loop_combo: 0,
            kickback: false,
            rip_blinking: false,

            lock_ready: false,
            lock_vault_ready: false,
            lock_well_ready: false,

            light_phase_tower: 0,
            light_phase_right: 0,

            timeout_top_loop: 0,
            timeout_left_ramp: 0,
            timeout_multi_bonus: 0,
            timeout_loop_combo: 0,
            timeout_tower_hunt: 0,
            timeout_lock: 0,

            score_vault: Bcd::from_ascii(b"500000"),
            score_well: Bcd::from_ascii(b"100000"),
            score_tower_bonus: Bcd::from_ascii(b"1000000"),
        }
    }
}

impl Table {
    pub fn stones_frame(&mut self) {
        self.stones.light_phase_right += 1;
        if self.stones.light_phase_right == 32 {
            self.stones.light_phase_right = 0;
        }
        self.stones.light_phase_tower += 1;
        if self.stones.light_phase_tower == 36 {
            self.stones.light_phase_tower = 0;
        }
        if self.in_drain || self.timer_stop {
            return;
        }
        if self.stones.timeout_top_loop != 0 {
            self.stones.timeout_top_loop -= 1;
        }
        if self.stones.timeout_lock != 0 && !self.stones.ball_locked {
            self.stones.timeout_lock -= 1;
            if self.stones.timeout_lock == 0 {
                self.stones.lock_ready = false;
                if self.stones.lock_vault_ready {
                    self.stones.lock_vault_ready = false;
                    self.light_set(LightBind::StonesVaultLock, 0, false);
                }
                if self.stones.lock_well_ready {
                    self.stones.lock_well_ready = false;
                    self.light_set(LightBind::StonesWellLock, 0, false);
                }
            }
        }
        if self.stones.timeout_left_ramp != 0 {
            self.stones.timeout_left_ramp -= 1;
            if self.stones.timeout_left_ramp == 0 {
                if self.stones.million_plus {
                    self.stones.million_plus = false;
                    self.light_set(LightBind::StonesMillionPlus, 0, false);
                }
                if self.stones.scream_x2 {
                    self.stones.scream_x2 = false;
                    self.light_set(LightBind::StonesScreamX2, 0, false);
                }
            }
            if self.stones.timeout_left_ramp == 90 {
                if self.stones.million_plus {
                    self.light_blink(LightBind::StonesMillionPlus, 0, 1, 0);
                }
                if self.stones.scream_x2 {
                    self.light_blink(LightBind::StonesScreamX2, 0, 1, 0);
                }
            }
        }
        if self.stones.timeout_multi_bonus != 0 {
            self.stones.timeout_multi_bonus -= 1;
            if self.stones.timeout_multi_bonus == 0 && self.stones.well_multi_bonus {
                self.stones.well_multi_bonus = false;
                self.light_set(LightBind::StonesWellMultiBonus, 0, false);
            }
            if self.stones.timeout_multi_bonus == 90 && self.stones.well_multi_bonus {
                self.light_blink(LightBind::StonesWellMultiBonus, 0, 1, 0);
            }
        }
        if self.stones.timeout_loop_combo != 0 {
            self.stones.timeout_loop_combo -= 1;
            if self.stones.timeout_loop_combo == 0 {
                self.stones.loop_combo = 0;
            }
        }
        if self.stones.timeout_tower_hunt != 0 {
            self.stones.timeout_tower_hunt -= 1;
            if self.stones.timeout_tower_hunt == 0 {
                self.stones.tower_hunt = false;
                self.stones.tower_hunt_ctr = 0;
                self.play_jingle_bind(JingleBind::StonesTowerHuntEnd);
                self.set_music_main();
            }
        }
    }

    pub fn stones_flipper_pressed(&mut self) {
        if !self.stones.flipper_lock_key {
            self.light_rotate(LightBind::StonesKey);
        }
        if !self.stones.flipper_lock_rip {
            self.light_rotate(LightBind::StonesRip);
        }
    }

    pub fn stones_drained(&mut self) {
        self.ball.frozen = true;
        self.flippers_enabled = false;
        self.in_mode = false;
        self.in_mode_hit = false;
        self.in_mode_ramp = false;
        self.in_drain = true;
        self.add_task(TaskKind::DrainSfx);
        self.light_set(LightBind::StonesGhost, 7, false);
        self.sequencer.reset_priority();
        self.effect(EffectBind::Drained);
    }

    pub fn stones_mode_check(&mut self) {
        if self.mode_timeout_secs == 0 {
            if self.in_mode_hit {
                self.play_jingle_bind(JingleBind::ModeEndHit);
            } else {
                self.play_jingle_bind(JingleBind::ModeEndRamp);
            }
            self.sequencer.set_music(3);
            self.sequencer.reset_priority();
            self.in_mode_hit = false;
            self.in_mode_ramp = false;
        }
    }

    pub fn stones_stones_bones_all(&mut self) {
        self.incr_jackpot();
        self.stones.stones_bones_blinking = true;
        if !self.stones.ghost_active {
            self.effect(
                [
                    EffectBind::StonesGhostLit0,
                    EffectBind::StonesGhostLit1,
                    EffectBind::StonesGhostLit2,
                    EffectBind::StonesGhostLit3,
                    EffectBind::StonesGhostLit4,
                    EffectBind::StonesGhostLit5,
                    EffectBind::StonesGhostLit6,
                    EffectBind::StonesGhostLit7,
                ][self.stones.cur_ghost as usize],
            );
            self.stones.ghost_active = true;
            self.light_blink(LightBind::StonesGhost, self.stones.cur_ghost, 32, 0);
            self.light_blink(LightBind::StonesVaultGhost, 0, 18, 0);
        } else {
            self.effect(EffectBind::StonesStonesBonesAllRedundant);
        }
        for i in 0..5 {
            self.light_blink(LightBind::StonesStone, i, 2, 0);
        }
        for i in 0..4 {
            self.light_blink(LightBind::StonesBone, i, 2, 0);
        }
        self.add_task(TaskKind::StonesUnblinkStonesBones);
    }

    pub fn stones_hit_stone(&mut self, which: u8) {
        if self.stones.ghosts_blinking
            || self.stones.stone_blinking[which as usize]
            || self.stones.stones_bones_blinking
        {
            return;
        }
        self.mode_count_hit();
        self.play_sfx_bind(SfxBind::StonesHitStone);
        self.score_premult(Bcd::from_ascii(b"17520"), Bcd::from_ascii(b"750"));
        self.light_set(LightBind::StonesStone, which, true);
        if self.light_all_lit(LightBind::StonesStone) && self.light_all_lit(LightBind::StonesBone) {
            self.stones_stones_bones_all();
        } else {
            self.stones.stone_blinking[which as usize] = true;
            self.light_blink(LightBind::StonesStone, which, 2, 0);
            self.add_task(TaskKind::StonesUnblinkStone(which));
        }
    }

    pub fn stones_hit_bone(&mut self, which: u8) {
        if self.stones.ghosts_blinking
            || self.stones.bone_blinking[which as usize]
            || self.stones.stones_bones_blinking
        {
            return;
        }
        self.mode_count_hit();
        self.play_sfx_bind(SfxBind::StonesHitBone);
        self.score_premult(Bcd::from_ascii(b"27530"), Bcd::from_ascii(b"510"));
        self.light_set(LightBind::StonesBone, which, true);
        if self.light_all_lit(LightBind::StonesStone) && self.light_all_lit(LightBind::StonesBone) {
            self.stones_stones_bones_all();
        } else {
            self.stones.bone_blinking[which as usize] = true;
            self.light_blink(LightBind::StonesBone, which, 2, 0);
            self.add_task(TaskKind::StonesUnblinkBone(which));
        }
    }

    pub fn stones_roll_key_entry(&mut self) {
        self.raise_physmap(PhysmapBind::StonesGateRampTower);
        if self.stones.million_plus {
            self.stones.score_million_plus += Bcd::from_ascii(b"1000000");
            self.score(self.stones.score_million_plus, Bcd::ZERO);
            self.effect(EffectBind::StonesMillionPlus);
            self.light_set(LightBind::StonesMillionPlus, 0, false);
            self.stones.million_plus = false;
        }
        self.stones.ball_locked = false;
        self.score(Bcd::from_ascii(b"10000"), Bcd::from_ascii(b"1000"));
    }

    pub fn stones_roll_key(&mut self, which: u8) {
        if self.stones.key_blinking {
            return;
        }
        self.play_sfx_bind(SfxBind::RollTrigger);
        self.score_premult(Bcd::from_ascii(b"10060"), Bcd::from_ascii(b"1010"));
        self.light_set(LightBind::StonesKey, which, true);
        if let Some(target) = self.stones.key_skillshot {
            if which == target {
                self.stones.score_skill_shot += Bcd::from_ascii(b"1000000");
                self.score_main += self.stones.score_skill_shot;
                self.effect(EffectBind::StonesSkillShot);
                self.stones_incr_vault();
                self.stones_incr_tower_bonus();
                self.stones_incr_well();
                self.incr_jackpot();
            } else {
                self.light_set(LightBind::StonesKey, target, false);
            }
            self.stones.key_skillshot = None;
        }
        if self.light_all_lit(LightBind::StonesKey) {
            self.stones_incr_vault();
            self.incr_jackpot();
            self.stones.key_blinking = true;
            self.stones.flipper_lock_key = true;
            for i in 0..3 {
                self.light_blink(LightBind::StonesKey, i, 2, 0);
            }
            match self.stones.key_tower_cycle {
                0 => {
                    if !self.stones.tower_1m {
                        self.stones.tower_1m = true;
                        self.light_blink(
                            LightBind::StonesTowerMillion,
                            0,
                            18,
                            self.stones.light_phase_tower,
                        );
                    }
                    self.stones.key_tower_cycle = 1;
                }
                1 => {
                    if !self.stones.tower_5m {
                        self.stones.tower_5m = true;
                        self.light_blink(
                            LightBind::StonesTower5M,
                            0,
                            18,
                            self.stones.light_phase_tower,
                        );
                    }
                    self.stones.key_tower_cycle = 2;
                }
                2 => {
                    if !self.stones.tower_double_bonus {
                        self.stones.tower_double_bonus = true;
                        self.light_blink(
                            LightBind::StonesTowerDoubleBonus,
                            0,
                            18,
                            self.stones.light_phase_tower,
                        );
                    }
                    self.stones.key_tower_cycle = 3;
                }
                3 => {
                    if !self.stones.tower_hold_bonus {
                        self.stones.tower_hold_bonus = true;
                        self.light_blink(
                            LightBind::StonesTowerHoldBonus,
                            0,
                            18,
                            self.stones.light_phase_tower,
                        );
                    }
                    self.stones.key_tower_cycle = 4;
                }
                _ => {
                    if !self.stones.tower_5m {
                        self.stones.tower_5m = true;
                        self.light_blink(
                            LightBind::StonesTower5M,
                            0,
                            18,
                            self.stones.light_phase_tower,
                        );
                    }
                }
            }
            if !self.stones.tower_open {
                self.effect(EffectBind::StonesTowerOpen);
            }
            self.stones_tower_open();
            self.add_task(TaskKind::StonesUnblinkKeyAll);
        } else {
            self.light_blink(LightBind::StonesKey, which, 2, 0);
            self.add_task(TaskKind::StonesUnblinkKey(which));
            self.stones.flipper_lock_key = true;
        }
    }

    pub fn stones_tower(&mut self) {
        self.ball.teleport_freeze(Layer::Overhead, (141, 143));
        self.stones.in_tower = true;
        self.mode_count_ramp();
        self.incr_jackpot();
        self.timer_stop = true;
        self.stones.tower_resume_mode = self.in_mode;
        self.stones.tower_resume_mode_ramp = self.in_mode_ramp;
        self.raise_physmap(PhysmapBind::StonesGateTowerEntry);
        self.drop_physmap(PhysmapBind::StonesGateRampTower);
        self.stones.tower_open = false;
        self.light_set(LightBind::StonesTower, 0, false);
        let mut visible_effect = false;
        if self.stones.tower_hunt && self.stones.tower_hunt_ctr < 3 {
            visible_effect |= self.effect(match self.stones.tower_hunt_ctr {
                0 => EffectBind::StonesTowerHunt0,
                1 => EffectBind::StonesTowerHunt1,
                2 => EffectBind::StonesTowerHunt2,
                _ => unreachable!(),
            });
            let music = self.sequencer.music();
            if music != 0x32 {
                self.sequencer.set_music(music + 1);
            }
            self.stones.tower_hunt_ctr += 1;
            self.stones_tower_open();
            self.silence_effect = true;
        }
        if self.stones.tower_super_jackpot {
            self.stones.tower_super_jackpot = false;
            self.light_set(LightBind::StonesTowerSuperJackpot, 0, false);
            self.effect_force(EffectBind::StonesTowerSuperJackpot);
            visible_effect = true;
            self.silence_effect = true;
        }
        if self.stones.tower_jackpot {
            self.stones.tower_jackpot = false;
            self.light_set(LightBind::StonesTowerJackpot, 0, false);
            self.effect_force(EffectBind::StonesTowerJackpot);
            self.score_main += self.score_jackpot;
            self.score_jackpot = self.assets.score_jackpot_init;
            visible_effect = true;
            self.silence_effect = true;
            self.stones.tower_super_jackpot = true;
            self.light_blink(
                LightBind::StonesTowerSuperJackpot,
                0,
                18,
                self.stones.light_phase_tower,
            );
            self.stones_tower_open();
            self.add_task(TaskKind::StonesResetSuperJackpot);
        }
        if self.stones.tower_extra_ball {
            self.stones.tower_extra_ball = false;
            self.light_set(LightBind::StonesTowerExtraBall, 0, false);
            visible_effect |= self.effect(EffectBind::StonesTowerExtraBall);
            self.extra_ball();
            self.silence_effect = true;
        }
        if self.stones.tower_double_bonus {
            self.stones.tower_double_bonus = false;
            self.light_set(LightBind::StonesTowerDoubleBonus, 0, false);
            visible_effect |= self.effect(EffectBind::StonesTowerDoubleBonus);
            self.score_bonus += self.score_bonus;
            self.silence_effect = true;
        }
        if self.stones.tower_hold_bonus {
            self.stones.tower_hold_bonus = false;
            self.light_set(LightBind::StonesTowerHoldBonus, 0, false);
            visible_effect |= self.effect(EffectBind::StonesTowerHoldBonus);
            self.hold_bonus = true;
            self.silence_effect = true;
        }
        if self.stones.tower_5m {
            self.stones.tower_5m = false;
            self.light_set(LightBind::StonesTower5M, 0, false);
            visible_effect |= self.effect(EffectBind::StonesTower5M);
            self.silence_effect = true;
        }
        if self.stones.tower_1m {
            self.stones.tower_1m = false;
            self.light_set(LightBind::StonesTowerMillion, 0, false);
            visible_effect |= self.effect(EffectBind::StonesTowerMillion);
            self.silence_effect = true;
        } else {
            visible_effect |= self.effect(EffectBind::StonesTowerBonus);
            self.score(self.stones.score_tower_bonus, Bcd::ZERO);
            self.stones.score_tower_bonus = Bcd::from_ascii(b"1000000");
            if self.stones.tower_hunt {
                self.stones_tower_open();
            }
        }
        if !visible_effect {
            self.add_task(TaskKind::StonesTowerEject);
        }
        self.silence_effect = false;
    }

    pub fn stones_tower_tilt(&mut self) {
        self.ball.teleport_freeze(Layer::Overhead, (141, 143));
        self.add_task(TaskKind::StonesTowerEject);
    }

    pub fn stones_end_mode(&mut self) {
        self.light_set(LightBind::StonesTowerJackpot, 0, false);
        self.light_set(LightBind::StonesTowerSuperJackpot, 0, false);
        self.stones.tower_jackpot = false;
        self.stones.tower_super_jackpot = false;
        self.stones_tower_check_close();
    }

    pub fn stones_tower_check_close(&mut self) {
        if !self.stones.tower_extra_ball
            && !self.stones.tower_jackpot
            && !self.stones.tower_super_jackpot
            && !self.stones.tower_1m
            && !self.stones.tower_5m
            && !self.stones.tower_double_bonus
            && !self.stones.tower_hold_bonus
            && !self.stones.tower_hunt
        {
            self.stones.tower_open = false;
            self.light_set(LightBind::StonesTower, 0, false);
            self.raise_physmap(PhysmapBind::StonesGateTowerEntry);
        }
    }

    pub fn stones_tower_open(&mut self) {
        self.drop_physmap(PhysmapBind::StonesGateTowerEntry);
        if !self.stones.tower_open {
            self.light_blink(LightBind::StonesTower, 0, 20, 0);
            self.stones.tower_open = true;
        }
    }

    pub fn stones_tower_eject(&mut self) {
        self.play_sfx_bind(SfxBind::StonesEject);
        self.ball.teleport(Layer::Overhead, (141, 143), (0, -3333));
        self.stones.in_tower = false;
    }

    pub fn stones_well(&mut self) {
        if self.light_state(LightBind::StonesWellLock, 0) {
            self.ball.frozen = true;
            self.add_task(TaskKind::StonesWellEject);
            return;
        }
        self.ball.teleport_freeze(Layer::Ground, (275, 245));
        self.stones.in_well = true;
        self.mode_count_ramp();
        self.incr_jackpot();
        let mut visible_effect = false;
        self.score_main += self.stones.score_well;
        if self.stones.lock_well_ready {
            self.stones.ball_locked = true;
            self.sequencer.reset_priority();
            visible_effect |= self.effect(EffectBind::StonesLock);
            self.silence_effect = true;
            self.ball.teleport(Layer::Ground, (300, 530), (10, 0));
            self.special_plunger_event = true;
            self.stones.in_well = false;
            self.set_music_plunger();
            self.stones.lock_well_ready = false;
            self.light_set(LightBind::StonesWellLock, 0, true);
        }
        visible_effect |= self.effect(EffectBind::StonesWell);
        if self.stones.well_multi_bonus {
            self.stones.well_multi_bonus = false;
            self.light_set(LightBind::StonesWellMultiBonus, 0, false);
            let which = self.light_sequence(LightBind::StonesBonus);
            self.bonus_mult_late = [2, 4, 6, 8, 10][which as usize];
            visible_effect |= self.effect(
                [
                    EffectBind::StonesWellMb2,
                    EffectBind::StonesWellMb4,
                    EffectBind::StonesWellMb6,
                    EffectBind::StonesWellMb8,
                    EffectBind::StonesWellMb10,
                ][which as usize],
            );
        }
        if !visible_effect {
            self.add_task(TaskKind::StonesWellEject);
        }
        self.silence_effect = false;
    }

    pub fn stones_well_tilt(&mut self) {
        self.add_task(TaskKind::StonesWellEject);
    }

    pub fn stones_vault(&mut self) {
        if !self.stones.vault_from_ramp {
            self.light_set(LightBind::StonesKickback, 0, false);
            self.stones.kickback = false;
        }
        self.ball.teleport_freeze(Layer::Ground, (2, 532));
        if self.tilted || self.light_state(LightBind::StonesVaultLock, 0) {
            self.add_task(TaskKind::StonesVaultEject);
            return;
        }
        self.stones.in_vault = true;
        self.incr_jackpot();
        let mut visible_effect = false;
        if self.stones.lock_vault_ready {
            self.stones.ball_locked = true;
            self.sequencer.reset_priority();
            visible_effect |= self.effect(EffectBind::StonesLock);
            self.silence_effect = true;
            self.ball.teleport(Layer::Ground, (300, 530), (10, 0));
            self.special_plunger_event = true;
            self.stones.in_vault = false;
            self.set_music_plunger();
            self.stones.lock_vault_ready = false;
            self.light_set(LightBind::StonesVaultLock, 0, true);
        }
        if self.stones.ghost_active {
            self.stones.ghost_active = false;
            self.light_set(LightBind::StonesVaultGhost, 0, false);
            self.light_set(LightBind::StonesGhost, self.stones.cur_ghost, true);
            match self.stones.cur_ghost {
                0 => {
                    visible_effect |= self.effect(EffectBind::StonesGhost5M);
                }
                1 => {
                    visible_effect |= self.effect(EffectBind::StonesGhostTowerHunt);
                    self.stones_tower_open();
                    self.stones.tower_hunt = true;
                    self.stones.tower_hunt_ctr = 0;
                    self.stones.timeout_tower_hunt = 2400;
                    self.sequencer.set_music(0x2e);
                }
                2 => {
                    visible_effect |= self.effect(EffectBind::StonesGhostExtraBall);
                    if !self.stones.tower_extra_ball {
                        self.stones.tower_extra_ball = true;
                        self.light_blink(
                            LightBind::StonesTowerExtraBall,
                            0,
                            18,
                            self.stones.light_phase_tower,
                        );
                        self.stones_tower_open();
                    }
                }
                3 => {
                    visible_effect |= self.effect(EffectBind::StonesGhost10M);
                }
                4 => {
                    self.add_task(TaskKind::StonesModeHit);
                    self.stones.vault_hold = true;
                    if !self.stones.tower_jackpot {
                        self.stones.tower_jackpot = true;
                        self.light_blink(
                            LightBind::StonesTowerJackpot,
                            0,
                            18,
                            self.stones.light_phase_tower,
                        );
                        self.stones_tower_open();
                    }
                }
                5 => {
                    visible_effect |= self.effect(EffectBind::StonesGhostDemon);
                    self.stones.lock_ready = true;
                    self.stones.lock_well_ready = true;
                    self.stones.lock_vault_ready = true;
                    self.stones.timeout_lock = 2100;
                    self.stones.ball_locked = false;
                    self.stones.scream_demon = true;
                    self.light_blink(LightBind::StonesWellLock, 0, 18, 0);
                    self.light_blink(LightBind::StonesVaultLock, 0, 18, 0);
                    self.light_blink(LightBind::StonesScreamDemon, 0, 18, 0);
                }
                6 => {
                    visible_effect |= self.effect(EffectBind::StonesGhost15M);
                }
                7 => {
                    self.add_task(TaskKind::StonesModeRamp);
                    if !self.stones.tower_jackpot {
                        self.stones.tower_jackpot = true;
                        self.light_blink(
                            LightBind::StonesTowerJackpot,
                            0,
                            18,
                            self.stones.light_phase_tower,
                        );
                        self.stones_tower_open();
                    }
                }
                _ => unreachable!(),
            }
            self.stones.cur_ghost += 1;
            if self.stones.cur_ghost == 8 {
                self.stones.cur_ghost = 0;
                self.stones.ghosts_blinking = true;
                for i in 0..8 {
                    self.light_blink(LightBind::StonesGhost, i, 2, 0);
                }
                self.add_task(TaskKind::StonesUnblinkGhosts);
            }
        }
        self.mode_count_ramp();
        self.score_main += self.stones.score_vault;
        if visible_effect {
            self.silence_effect = true;
        }
        visible_effect |= self.effect(EffectBind::StonesVault);
        self.silence_effect = false;
        if !visible_effect {
            self.add_task(TaskKind::StonesVaultEject);
        }
    }

    pub fn stones_ramp_top(&mut self) {
        self.mode_count_ramp();
        self.stones_incr_vault();
        if self.stones.timeout_loop_combo != 0 && self.stones.loop_combo == 2 {
            self.effect(EffectBind::StonesLoopCombo);
        }
        self.stones.loop_combo = 0;
        if self.stones.timeout_top_loop != 0 {
            self.effect(EffectBind::StonesTopMillion);
        } else {
            self.play_sfx_bind(SfxBind::RollTrigger);
            self.score_premult(Bcd::from_ascii(b"10030"), Bcd::from_ascii(b"1020"));
        }
        self.stones.timeout_top_loop = 300;
    }

    pub fn stones_roll_rip(&mut self, which: u8) {
        if self.stones.rip_blinking {
            return;
        }
        self.play_sfx_bind(SfxBind::RollTrigger);
        self.score_premult(Bcd::from_ascii(b"10070"), Bcd::from_ascii(b"1080"));
        self.light_set(LightBind::StonesRip, which, true);
        if self.light_all_lit(LightBind::StonesRip) {
            self.stones.rip_blinking = true;
            self.stones.flipper_lock_rip = true;
            for i in 0..3 {
                self.light_blink(LightBind::StonesRip, i, 2, 0);
            }
            if !self.stones.kickback {
                self.stones.kickback = true;
                self.light_blink(LightBind::StonesKickback, 0, 18, 0);
                self.drop_physmap(PhysmapBind::StonesGateKickback);
            }
            self.effect(EffectBind::StonesKickback);
            self.add_task(TaskKind::StonesUnblinkRipAll);
        } else {
            self.add_task(TaskKind::StonesUnblinkRip(which));
            self.light_blink(LightBind::StonesRip, which, 2, 0);
            self.stones.flipper_lock_rip = true;
        }
    }

    pub fn stones_ramp_screams(&mut self) {
        self.play_sfx_bind(SfxBind::RollTrigger);
        self.score_premult(Bcd::from_ascii(b"10060"), Bcd::from_ascii(b"1050"));
        if self.stones.scream_demon {
            self.stones.scream_demon = false;
            self.light_set(LightBind::StonesScreamDemon, 0, false);
            self.stones.timeout_lock = 1;
            let num_locked = u8::from(self.light_state(LightBind::StonesWellLock, 0))
                + u8::from(self.light_state(LightBind::StonesVaultLock, 0));
            self.effect(match num_locked {
                0 => EffectBind::StonesDemon5M,
                1 => EffectBind::StonesDemon10M,
                2 => EffectBind::StonesDemon20M,
                _ => unreachable!(),
            });
            self.light_set(LightBind::StonesWellLock, 0, false);
            self.light_set(LightBind::StonesVaultLock, 0, false);
            self.silence_effect = true;
        }
        if self.stones.timeout_loop_combo != 0 && self.stones.loop_combo == 1 {
            self.stones.loop_combo = 2;
        }
        self.mode_count_ramp();
        self.stones_incr_well();
        self.add_cyclone(1);
        if self.stones.scream_x2 {
            self.add_task(TaskKind::StonesScreamExtra);
            self.light_set(LightBind::StonesScreamX2, 0, false);
            self.stones.scream_x2 = false;
        }
        self.num_cyclone_target = self.num_cyclone / 10 * 10 + 10;
        if self.num_cyclone % 10 == 0 {
            if self.num_cyclone == 10 {
                if !self.stones.tower_extra_ball {
                    self.stones.tower_extra_ball = true;
                    self.light_blink(
                        LightBind::StonesTowerExtraBall,
                        0,
                        18,
                        self.stones.light_phase_tower,
                    );
                    self.stones_tower_open();
                    self.effect(EffectBind::StonesScreamsExtraBall);
                }
            } else {
                if !self.stones.tower_5m {
                    self.stones.tower_5m = true;
                    self.light_blink(
                        LightBind::StonesTower5M,
                        0,
                        18,
                        self.stones.light_phase_tower,
                    );
                    self.stones_tower_open();
                    self.effect(EffectBind::StonesTowerOpen);
                }
                self.effect(EffectBind::StonesScreamsTo5M);
            }
        } else if self.num_cyclone < 10 {
            self.effect(EffectBind::StonesScreamsToExtraBall);
        } else {
            self.effect(EffectBind::StonesScreamsTo5M);
        }
        self.silence_effect = false;
    }

    pub fn stones_ramp_left_to_lane(&mut self) {
        self.drop_physmap(PhysmapBind::StonesGateRampLeft0);
        if self.tilted {
            return;
        }
        self.mode_count_ramp();
        self.stones_incr_vault();
        self.play_sfx_bind(SfxBind::RollTrigger);
        self.score_premult(Bcd::from_ascii(b"10030"), Bcd::from_ascii(b"1040"));
        if !self.stones.million_plus {
            self.stones.million_plus = true;
            self.light_blink(
                LightBind::StonesMillionPlus,
                0,
                16,
                self.stones.light_phase_right,
            );
        }
        if !self.stones.scream_x2 {
            self.stones.scream_x2 = true;
            self.light_blink(
                LightBind::StonesScreamX2,
                0,
                16,
                self.stones.light_phase_right,
            );
        }
        self.stones.timeout_left_ramp = 450;
        if !self.stones.well_multi_bonus && !self.light_all_lit(LightBind::StonesBonus) {
            self.stones.well_multi_bonus = true;
            self.light_blink(
                LightBind::StonesWellMultiBonus,
                0,
                16,
                self.stones.light_phase_right,
            );
        }
        self.stones.timeout_multi_bonus = 570;
        self.stones.loop_combo = 1;
        self.stones.timeout_loop_combo = if self.hifps { 936 } else { 780 };
    }

    pub fn stones_ramp_left_to_vault(&mut self) {
        self.raise_physmap(PhysmapBind::StonesGateRampLeft0);
        if self.tilted {
            return;
        }
        self.mode_count_ramp();
        self.play_sfx_bind(SfxBind::RollTrigger);
        self.score_premult(Bcd::from_ascii(b"10020"), Bcd::from_ascii(b"1010"));
    }

    pub fn stones_incr_vault(&mut self) {
        self.stones.score_vault += Bcd::from_ascii(b"82150");
    }

    pub fn stones_incr_well(&mut self) {
        self.stones.score_well += Bcd::from_ascii(b"64190");
    }

    pub fn stones_incr_tower_bonus(&mut self) {
        self.stones.score_tower_bonus += Bcd::from_ascii(b"223470");
    }

    pub fn stones_load_fixup(&mut self) {
        if self.stones.kickback {
            self.light_blink(LightBind::StonesKickback, 0, 18, 0);
            self.drop_physmap(PhysmapBind::StonesGateKickback);
        }
        for i in 0..self.stones.cur_ghost {
            self.light_set(LightBind::StonesGhost, i, true);
        }
        if self.stones.ghost_active {
            self.light_blink(LightBind::StonesGhost, self.stones.cur_ghost, 32, 0);
            self.light_blink(LightBind::StonesVaultGhost, 0, 18, 0);
        }
    }
}
