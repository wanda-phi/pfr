use rand::{thread_rng, Rng};

use crate::{
    assets::table::{
        lights::LightBind,
        physics::{Layer, PhysmapBind},
        script::{EffectBind, ScriptBind},
        sound::{JingleBind, SfxBind},
    },
    bcd::Bcd,
};

use super::{tasks::TaskKind, Table};

pub struct PartyState {
    pub flipper_lock_puke: bool,

    pub drop_zone_delay: u16,
    pub drop_zone_scroll_pos: u16,

    pub orbit_right_cycle: u8,
    pub orbit_right_blinking: bool,
    pub orbit_right_mb: bool,
    pub orbit_right_hb: bool,
    pub orbit_right_db: bool,
    pub mad_blinking: bool,
    pub cyclone_x5: bool,

    pub secret_drop_release: bool,
    pub arcade_button_just_hit: bool,
    pub arcade_open: bool,
    pub arcade_ready: bool,

    pub duck_hit: [bool; 3],
    pub cur_snack: u8,
    pub snack_lit: [bool; 3],
    pub in_snack: bool,
    pub popcorns: u8,

    pub in_demon: bool,
    pub demon_reward: u8,
    pub demon_5m: bool,
    pub demon_extra_ball: bool,
    pub demon_jackpot: bool,
    pub demon_jackpot_timed: bool,

    pub skyride: u8,

    pub score_cyclone_skill_shot: Bcd,
    pub score_tunnel_skill_shot: Bcd,

    pub light_phase_snack: u8,
    pub light_phase_orbit_special: u8,
    pub light_phase_puke: u8,
    pub light_phase_demon: u8,

    pub timeout_skill_shot: u16,
    pub timeout_party_t: u16,
    pub timeout_party_pr: u16,
    pub timeout_spring_loop: u16,
    pub timeout_tunnel: u16,
}

impl PartyState {
    pub fn new() -> Self {
        Self {
            flipper_lock_puke: false,

            drop_zone_delay: 0,
            drop_zone_scroll_pos: 0,

            orbit_right_cycle: 0,
            orbit_right_blinking: false,
            orbit_right_mb: false,
            orbit_right_hb: false,
            orbit_right_db: false,
            mad_blinking: false,
            cyclone_x5: false,

            secret_drop_release: false,
            arcade_button_just_hit: false,
            arcade_open: false,
            arcade_ready: false,

            duck_hit: [false; 3],
            cur_snack: 0,
            snack_lit: [false; 3],
            in_snack: false,
            popcorns: 0,

            in_demon: false,
            demon_reward: 0,
            demon_5m: false,
            demon_extra_ball: false,
            demon_jackpot: false,
            demon_jackpot_timed: false,

            skyride: 0,

            score_cyclone_skill_shot: Bcd::ZERO,
            score_tunnel_skill_shot: Bcd::ZERO,

            light_phase_snack: 0,
            light_phase_orbit_special: 0,
            light_phase_puke: 0,
            light_phase_demon: 0,

            timeout_skill_shot: 0,
            timeout_party_t: 0,
            timeout_party_pr: 0,
            timeout_spring_loop: 0,
            timeout_tunnel: 0,
        }
    }
}

impl Table {
    pub fn party_frame(&mut self) {
        if self.in_drain {
            return;
        }
        if self.party.timeout_skill_shot != 0 {
            self.party.timeout_skill_shot -= 1;
        }
        if self.party.timeout_party_t != 0 {
            self.party.timeout_party_t -= 1;
        }
        if self.party.timeout_party_pr != 0 {
            self.party.timeout_party_pr -= 1;
        }
        if self.party.timeout_spring_loop != 0 {
            self.party.timeout_spring_loop -= 1;
        }
        if self.party.timeout_tunnel != 0 {
            self.party.timeout_tunnel -= 1;
            if self.party.timeout_tunnel == 720 {
                self.light_set(LightBind::PartyTunnel, 2, false);
                self.light_set(LightBind::PartyTunnel, 1, false);
                self.light_blink(LightBind::PartyTunnel, 1, 8, 0);
            } else if self.party.timeout_tunnel == 0 {
                self.light_set(LightBind::PartyTunnel, 1, false);
                self.light_set(LightBind::PartyTunnel, 0, false);
                self.light_blink(LightBind::PartyTunnel, 0, 8, 0);
            }
        }
        self.party.light_phase_snack += 1;
        if self.party.light_phase_snack == 16 {
            self.party.light_phase_snack = 0;
        }
        self.party.light_phase_orbit_special += 1;
        if self.party.light_phase_orbit_special == 24 {
            self.party.light_phase_orbit_special = 0;
        }
        self.party.light_phase_puke += 1;
        if self.party.light_phase_puke == 4 {
            self.party.light_phase_puke = 0;
        }
        self.party.light_phase_demon += 1;
        if self.party.light_phase_demon == 28 {
            self.party.light_phase_demon = 0;
        }
    }

    pub fn party_flipper_pressed(&mut self) {
        if !self.party.flipper_lock_puke {
            self.light_rotate(LightBind::PartyPuke);
        }
    }

    pub fn party_mode_check(&mut self) {
        if self.mode_timeout_secs == 2
            && self.party.demon_jackpot_timed
            && !self.party.demon_jackpot
        {
            self.light_blink(LightBind::PartyDemonJackpot, 0, 2, 0);
        }
        if self.mode_timeout_secs == 0 {
            self.in_mode = false;
            self.party.demon_jackpot_timed = false;
            if !self.party.demon_jackpot {
                self.light_set(LightBind::PartyDemonJackpot, 0, false);
            }
            if self.in_mode_hit {
                self.in_mode_hit = false;
                self.light_set(LightBind::PartyHappyHour, 0, false);
                self.effect(EffectBind::PartyHappyHourEnd);
                if self.pending_mode_ramp {
                    self.pending_mode_ramp = false;
                    self.add_task(TaskKind::PartyMegaLaugh);
                }
            } else {
                self.in_mode_ramp = false;
                self.light_set(LightBind::PartyMegaLaugh, 0, false);
                self.effect(EffectBind::PartyMegaLaughEnd);
                if self.pending_mode_hit {
                    self.pending_mode_hit = false;
                    self.add_task(TaskKind::PartyHappyHour);
                }
            }
            self.sequencer.set_music(1);
        }
    }

    pub fn party_drained(&mut self) {
        if self.ball_scored_points {
            self.effect(EffectBind::Drained);
            self.set_music_silence();
            self.add_task(TaskKind::DrainSfx);
        } else {
            self.start_script(ScriptBind::PartyOn);
            self.play_jingle_plunger();
            self.party_on = true;
            self.add_task(TaskKind::PartyOn);
        }
    }

    pub fn party_start_drop_zone(&mut self) {
        self.ball.teleport(Layer::Ground, (15, 47), (0, 0));
        self.add_task(TaskKind::PartyDropZoneScroll);
        self.add_task(TaskKind::PartyDropZoneWait);
        self.party.drop_zone_scroll_pos = self.scroll.pos();
    }

    pub fn party_party(&mut self, which: u8) {
        if !self.light_state(LightBind::PartyParty, which) {
            self.light_set(LightBind::PartyParty, which, true);
            self.effect(match which {
                0 => EffectBind::PartyPartyP,
                1 => EffectBind::PartyPartyA,
                2 => EffectBind::PartyPartyR,
                3 => EffectBind::PartyPartyT,
                4 => EffectBind::PartyPartyY,
                _ => unreachable!(),
            });
            self.party_check_party_all();
        }
    }

    pub fn party_check_party_all(&mut self) {
        if self.light_all_lit(LightBind::PartyParty) {
            if self.in_mode {
                self.pending_mode_hit = true;
            } else {
                self.party_happy_hour();
            }
        }
    }

    pub fn party_happy_hour(&mut self) {
        self.effect(EffectBind::PartyHappyHour);
        self.sequencer.set_music(0x2b);
        self.light_set_all(LightBind::PartyParty, false);
        if !self.party.demon_jackpot && !self.party.demon_jackpot_timed {
            self.light_blink(
                LightBind::PartyDemonJackpot,
                0,
                14,
                self.party.light_phase_demon,
            );
        }
        self.party.demon_jackpot_timed = true;
        self.in_mode = true;
        self.in_mode_hit = true;
        self.light_blink(LightBind::PartyHappyHour, 0, 8, 0);
        self.pending_mode = true;
    }

    pub fn party_crazy_letter(&mut self, effect: EffectBind) -> bool {
        self.incr_jackpot();
        let res = self.effect(effect);
        self.light_sequence(LightBind::PartyCrazy);
        if self.light_all_lit(LightBind::PartyCrazy) {
            self.light_set_all(LightBind::PartyCrazy, false);
            if self.in_mode {
                self.pending_mode_ramp = true;
            } else {
                self.party_mega_laugh();
            }
        }
        res
    }

    pub fn party_mega_laugh(&mut self) {
        self.effect(EffectBind::PartyMegaLaugh);
        self.sequencer.set_music(0x19);
        if !self.party.demon_jackpot && !self.party.demon_jackpot_timed {
            self.light_blink(
                LightBind::PartyDemonJackpot,
                0,
                14,
                self.party.light_phase_demon,
            );
        }
        self.party.demon_jackpot_timed = true;
        self.in_mode = true;
        self.in_mode_ramp = true;
        self.light_blink(LightBind::PartyMegaLaugh, 0, 8, 0);
        self.pending_mode = true;
    }

    pub fn party_arcade_button(&mut self) {
        if self.party.arcade_button_just_hit {
            return;
        }
        self.play_sfx_bind(SfxBind::PartyArcadeButton);
        self.party.arcade_button_just_hit = true;
        self.add_task(TaskKind::PartyResetArcadeButton);
        if !self.party.arcade_open {
            self.party.arcade_open = true;
            self.light_blink(LightBind::PartyArcade, 0, 12, 0);
            self.light_blink(LightBind::PartyArcade, 1, 12, 0);
        }
    }

    pub fn party_hit_duck(&mut self, which: u8) {
        if !self.light_state(LightBind::PartyDuckDrop, which) {
            return;
        }
        if self.party.duck_hit[which as usize] {
            return;
        }
        self.party.duck_hit[which as usize] = true;
        self.light_set(LightBind::PartyDuckDrop, which, false);
        self.add_task(TaskKind::PartyDuckDrop(which));
        self.play_sfx_bind(SfxBind::PartyHitDuck);
        self.score_premult(Bcd::from_ascii(b"7510"), Bcd::from_ascii(b"750"));
        self.mode_count_hit();
        if self.light_all_unlit(LightBind::PartyDuckDrop) {
            self.effect(EffectBind::PartyDuckAll);
            for i in 0..3 {
                self.light_blink(LightBind::PartyDuck, i, 2, 0);
            }
            self.add_task(TaskKind::PartyDuckAllUnblink);
            if !self.party.snack_lit[self.party.cur_snack as usize] {
                let phase = match self.party.cur_snack {
                    0 | 2 => self.party.light_phase_snack,
                    1 => (self.party.light_phase_snack + 8) % 16,
                    _ => unreachable!(),
                };
                self.light_blink(LightBind::PartySnack, self.party.cur_snack, 8, phase);
                self.party.snack_lit[self.party.cur_snack as usize] = true;
            }
            self.party.cur_snack += 1;
            if self.party.cur_snack == 3 {
                self.party.cur_snack = 0;
            }
        } else {
            self.add_task(TaskKind::PartyDuckUnblink(which));
            self.light_blink(LightBind::PartyDuck, which, 3, 0);
        }
    }

    pub fn party_orbit_right(&mut self) {
        if self.party.orbit_right_blinking {
            return;
        }
        self.incr_jackpot();
        self.mode_count_ramp();
        if self.party.timeout_party_t != 0 {
            self.party_party(3);
        }
        self.party.timeout_party_t = 600;
        self.party.timeout_party_pr = 300;
        if self.party.orbit_right_cycle < 2 {
            self.light_set(
                LightBind::PartyRightOrbitScore,
                self.party.orbit_right_cycle,
                true,
            );
            self.light_blink(
                LightBind::PartyRightOrbitScore,
                self.party.orbit_right_cycle + 1,
                9,
                0,
            );
            self.effect(if self.party.orbit_right_cycle == 0 {
                EffectBind::PartyOrbit250k
            } else {
                EffectBind::PartyOrbit500k
            });
            self.party.orbit_right_cycle += 1;
        } else {
            for i in 0..3 {
                self.light_blink(LightBind::PartyRightOrbitScore, i, 2, 0);
            }
            self.effect(EffectBind::PartyOrbit750k);
            self.add_task(TaskKind::PartyOrbitRightUnblink);
            self.party.orbit_right_blinking = true;
            self.party.orbit_right_cycle = 0;
        }
        if self.party.orbit_right_mb {
            self.party.orbit_right_mb = false;
            self.light_set(LightBind::PartyRightOrbitMultiBonus, 0, false);
            'mult_bonus: {
                let (bonus_mult, effect) = match self.light_sequence(LightBind::PartyBonus) {
                    0 => (2, EffectBind::PartyOrbitMb2),
                    1 => (4, EffectBind::PartyOrbitMb4),
                    2 => (6, EffectBind::PartyOrbitMb6),
                    3 => (8, EffectBind::PartyOrbitMb8),
                    _ => break 'mult_bonus,
                };
                self.effect(effect);
                self.bonus_mult_early = bonus_mult;
                self.bonus_mult_late = bonus_mult;
            }
        }
        if self.party.orbit_right_hb {
            self.party.orbit_right_hb = false;
            self.light_set(LightBind::PartyRightOrbitHoldBonus, 0, false);
            self.effect(EffectBind::PartyOrbitHoldBonus);
            self.hold_bonus = true;
        }
        if self.party.orbit_right_db {
            self.party.orbit_right_db = false;
            self.light_set(LightBind::PartyRightOrbitDoubleBonus, 0, false);
            self.effect(EffectBind::PartyOrbitDoubleBonus);
            self.score_bonus += self.score_bonus;
        }
    }

    pub fn party_orbit_left(&mut self) {
        if self.party.mad_blinking {
            return;
        }
        self.incr_jackpot();
        self.mode_count_ramp();
        if self.party.timeout_party_t != 0 {
            self.party_party(3);
        }
        self.party.timeout_party_t = 600;
        let which = self.light_sequence(LightBind::PartyMad);
        self.effect(match which {
            0 => EffectBind::PartyOrbitMad0,
            1 => EffectBind::PartyOrbitMad1,
            2 => EffectBind::PartyOrbitMad2,
            _ => unreachable!(),
        });
        if which < 2 {
            self.light_blink(LightBind::PartyMad, which, 2, 0);
            self.add_task(TaskKind::PartyMadUnblink(which));
        } else {
            for i in 0..3 {
                self.light_blink(LightBind::PartyMad, i, 2, 0);
            }
            self.add_task(TaskKind::PartyMadAllUnblink);
            self.party.mad_blinking = true;
            self.party_crazy_letter(EffectBind::PartyOrbitCrazy);
        }
    }

    pub fn party_secret(&mut self) {
        self.party.secret_drop_release = false;
        self.effect(EffectBind::PartySecret);
        self.incr_jackpot();
        self.light_blink(LightBind::PartyCycloneX5, 0, 6, 0);
        self.party.cyclone_x5 = true;
        self.add_task(TaskKind::PartySecretDrop);
        self.ball.teleport(Layer::Ground, (15, 47), (0, 0));
    }

    pub fn party_secret_tilt(&mut self) {
        self.ball.teleport(Layer::Ground, (15, 47), (0, 0));
        self.party_start_drop_zone();
    }

    pub fn party_tunnel(&mut self) {
        self.incr_jackpot();
        self.mode_count_ramp();
        if self.party.timeout_skill_shot != 0 {
            self.incr_jackpot();
            self.party.timeout_skill_shot = 0;
            self.party.score_tunnel_skill_shot += Bcd::from_ascii(b"1000000");
            self.score_main += self.party.score_tunnel_skill_shot;
            self.effect(EffectBind::PartyTunnelSkillShot);
            self.silence_effect = true;
            self.party_party(0);
        } else if self.party.timeout_party_pr != 0 {
            self.party_party(0);
        }
        if !self.light_state(LightBind::PartyTunnel, 0) {
            self.effect(EffectBind::PartyTunnel1M);
            self.party.timeout_tunnel = 720;
            self.light_set(LightBind::PartyTunnel, 0, true);
            self.light_blink(LightBind::PartyTunnel, 1, 8, 0);
        } else if !self.light_state(LightBind::PartyTunnel, 1) {
            self.effect(EffectBind::PartyTunnel3M);
            self.party.timeout_tunnel = 1440;
            self.light_set(LightBind::PartyTunnel, 1, true);
            self.light_blink(LightBind::PartyTunnel, 2, 8, 0);
        } else {
            self.effect(EffectBind::PartyTunnel5M);
            self.party.timeout_tunnel = 1440;
        }
        self.add_task(TaskKind::PartyDropZoneScroll);
        self.add_task(TaskKind::PartyDropZoneStart(if self.in_mode {
            0
        } else {
            130
        }));
        if !self.in_mode {
            self.add_task(TaskKind::PartyTunnelFreeze);
        }
        self.silence_effect = false;
    }

    pub fn party_tunnel_tilt(&mut self) {
        self.ball.teleport(Layer::Ground, (15, 47), (0, 0));
        self.party_start_drop_zone();
    }

    pub fn party_arcade(&mut self) {
        self.mode_count_ramp();
        if self.tilted || !self.party.arcade_open {
            self.party_start_drop_zone();
            return;
        }
        self.party.arcade_open = false;
        self.light_set_all(LightBind::PartyArcade, false);
        if self.effect(EffectBind::PartyArcade) {
            self.party.arcade_ready = false;
            self.set_music_silence();
            self.add_task(TaskKind::PartyArcadePickReward);
        } else {
            self.party.arcade_ready = true;
            self.party_arcade_pick_reward();
        }
        self.add_task(TaskKind::PartyDropZoneScroll);
        self.ball.teleport(Layer::Ground, (15, 47), (0, 0));
    }

    pub fn party_arcade_pick_reward(&mut self) {
        let delay = match thread_rng().gen_range(0..6) {
            0 => {
                // side extra ball
                self.light_set(LightBind::PartySideExtraBall, 0, true);
                if self.effect(EffectBind::PartyArcadeSideExtraBall) {
                    160
                } else {
                    10
                }
            }
            1 => {
                // crazy letter
                if self.party_crazy_letter(EffectBind::PartyArcadeCrazy) {
                    self.add_task(TaskKind::PartyArcadeDropZoneStart(140, 180, false));
                    return;
                } else {
                    10
                }
            }
            2 => {
                if self.effect(EffectBind::PartyArcade1M) {
                    self.add_task(TaskKind::PartyArcadeDropZoneStart(120, 150, false));
                    return;
                } else {
                    10
                }
            }
            3 => {
                if self.effect(EffectBind::PartyArcade5M) {
                    self.add_task(TaskKind::PartyArcadeDropZoneStart(110, 140, false));
                    return;
                } else {
                    10
                }
            }
            4 => {
                if self.effect(EffectBind::PartyArcade500k) {
                    self.add_task(TaskKind::PartyArcadeDropZoneStart(45, 70, false));
                    return;
                } else {
                    10
                }
            }
            5 => {
                self.effect(EffectBind::PartyArcadeNoScore);
                45
            }
            _ => unreachable!(),
        };
        self.add_task(TaskKind::PartyDropZoneStart(delay));
    }

    pub fn party_ramp_snack(&mut self) {
        if self.party.in_snack {
            return;
        }
        self.party.in_snack = true;
        self.score_premult(Bcd::from_ascii(b"50000"), Bcd::from_ascii(b"5000"));
        self.mode_count_ramp();
        if self.party.snack_lit[2] {
            self.effect(EffectBind::PartySnack2);
            self.incr_jackpot();
            if self.party.popcorns < 2 {
                self.party.popcorns += 1;
            }
        } else if self.party.snack_lit[1] {
            self.effect(EffectBind::PartySnack1);
            self.incr_jackpot();
        } else if self.party.snack_lit[0] {
            self.effect(EffectBind::PartySnack0);
            self.incr_jackpot();
        } else {
            self.effect(EffectBind::PartySnackNope);
        }
        self.light_set_all(LightBind::PartySnack, false);
        if self.party.snack_lit[2] {
            if !self.light_state(LightBind::PartyParty, 1) {
                self.light_set(LightBind::PartyParty, 1, true);
                self.effect(EffectBind::PartyPartyA);
                self.party_check_party_all();
            }
            if self.party.popcorns == 1 {
                if !self.party.orbit_right_hb {
                    self.party.orbit_right_hb = true;
                    self.light_blink(
                        LightBind::PartyRightOrbitHoldBonus,
                        0,
                        12,
                        (self.party.light_phase_orbit_special + 12) % 24,
                    );
                }
            } else if !self.party.orbit_right_db {
                self.party.orbit_right_db = true;
                self.light_blink(
                    LightBind::PartyRightOrbitDoubleBonus,
                    0,
                    12,
                    self.party.light_phase_orbit_special,
                );
                self.add_task(TaskKind::PartyDoubleBonusBlink);
            }
        }

        self.party.snack_lit = [false; 3];
        self.ball.teleport_freeze(Layer::Overhead, (3, 253));
        self.add_task(TaskKind::PartySnacksRelease);
    }

    pub fn party_demon(&mut self) {
        if self.party.in_demon {
            return;
        }
        self.party.in_demon = true;
        self.ball.teleport_freeze(Layer::Ground, (257, 310));
        let mut got_something = false;
        let mut timeout = 85;
        if self.party.demon_5m {
            self.party.demon_5m = false;
            self.effect(EffectBind::PartyDemon5M);
            self.light_set(LightBind::PartyDemon5M, 0, false);
            got_something = true;
            timeout = if self.in_mode { 15 } else { 160 };
        }
        if self.party.demon_extra_ball {
            self.party.demon_extra_ball = false;
            self.effect(EffectBind::PartyDemonExtraBall);
            self.extra_ball();
            self.light_set(LightBind::PartyDemonExtraBall, 0, false);
            got_something = true;
            timeout = if self.in_mode { 15 } else { 320 };
        }
        if self.party.demon_jackpot || self.party.demon_jackpot_timed {
            self.party.demon_jackpot = false;
            self.party.demon_jackpot_timed = false;
            self.light_set(LightBind::PartyDemonJackpot, 0, false);
            self.play_jingle_bind(JingleBind::PartyJackpot);
            if !self.in_mode {
                self.start_script(ScriptBind::PartyJackpot);
            } else if self.in_mode_hit {
                self.start_script(ScriptBind::PartyJackpotModeHit);
            } else {
                self.start_script(ScriptBind::PartyJackpotModeRamp);
            }
            self.score_main += self.score_jackpot;
            self.score_jackpot = self.assets.score_jackpot_init;
            got_something = true;
            timeout = 410;
        }
        if !got_something {
            self.effect(EffectBind::PartyDemon250k);
        }
        self.add_task(TaskKind::PartyDemonBlink(timeout));
    }

    pub fn party_lane_outer(&mut self) {
        if self.light_state(LightBind::PartySideExtraBall, 0) {
            self.light_set(LightBind::PartySideExtraBall, 0, false);
            self.effect(EffectBind::PartySideExtraBall);
            self.extra_ball();
            self.block_drain = true;
            self.add_task(TaskKind::PartySideExtraBallFinish);
        } else {
            self.play_sfx_bind(SfxBind::RollTrigger);
            self.score(Bcd::from_ascii(b"50030"), Bcd::ZERO);
        }
    }

    pub fn party_skyride_top(&mut self) {
        self.mode_count_ramp();
        self.raise_physmap(PhysmapBind::PartyGateSkyride);
        if self.party.timeout_party_t != 0 {
            self.party.timeout_party_t = 0;
            self.party_party(3);
        }
        self.party.timeout_party_t = 600;
        match self.party.skyride {
            0 => {
                self.effect(EffectBind::PartySkyride0);
                self.light_set(LightBind::PartySkyride, 0, true);
                self.party.skyride = 1;
            }
            1 => {
                self.effect(EffectBind::PartySkyride1);
                self.light_set(LightBind::PartySkyride, 1, true);
                self.party.skyride = 2;
            }
            2 => {
                self.effect(EffectBind::PartySkyride2);
                for i in 0..3 {
                    self.light_blink(LightBind::PartySkyride, i, 2, 0);
                }
                self.party.skyride = 0;
                self.add_task(TaskKind::PartySkyrideUnblink);
                if !self.party.orbit_right_mb && !self.light_state(LightBind::PartyBonus, 3) {
                    self.party.orbit_right_mb = true;
                    self.light_blink(
                        LightBind::PartyRightOrbitMultiBonus,
                        0,
                        12,
                        self.party.light_phase_orbit_special,
                    );
                    self.effect(EffectBind::PartySkyrideLitMb);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn party_puke(&mut self, which: u8) {
        self.play_sfx_bind(SfxBind::RollTrigger);
        if self.light_state(LightBind::PartyPuke, which) {
            return;
        }
        self.light_set(LightBind::PartyPuke, which, true);
        self.score_premult(Bcd::from_ascii(b"20070"), Bcd::from_ascii(b"1000"));
        self.incr_jackpot();
        if self.light_all_lit(LightBind::PartyPuke) {
            self.light_blink(LightBind::PartyPuke, 0, 2, self.party.light_phase_puke);
            self.light_blink(LightBind::PartyPuke, 2, 2, self.party.light_phase_puke);
            self.light_blink(
                LightBind::PartyPuke,
                1,
                2,
                (self.party.light_phase_puke + 2) % 4,
            );
            self.light_blink(
                LightBind::PartyPuke,
                3,
                2,
                (self.party.light_phase_puke + 2) % 4,
            );
            self.add_task(TaskKind::PartyPukeUnblinkAll);
            match self.party.demon_reward {
                0 => {
                    self.party.demon_5m = true;
                    self.light_blink(LightBind::PartyDemon5M, 0, 14, self.party.light_phase_demon);
                    self.party.demon_reward = 1;
                }
                1 => {
                    self.party.demon_extra_ball = true;
                    self.light_blink(
                        LightBind::PartyDemonExtraBall,
                        0,
                        14,
                        (self.party.light_phase_demon + 14) % 28,
                    );
                    self.party.demon_reward = 2;
                }
                2 => {
                    self.party.demon_jackpot = true;
                    self.light_blink(
                        LightBind::PartyDemonJackpot,
                        0,
                        14,
                        self.party.light_phase_demon,
                    );
                    self.party.demon_reward = 3;
                }
                _ => (),
            }
            self.party_party(4);
        } else {
            self.party.flipper_lock_puke = true;
            self.light_blink(LightBind::PartyPuke, which, 2, 0);
            self.add_task(TaskKind::PartyPukeUnblink(which));
        }
    }

    pub fn party_ramp_cyclone(&mut self) {
        self.mode_count_ramp();
        if self.party.timeout_skill_shot != 0 {
            self.incr_jackpot();
            self.incr_jackpot();
            self.party.score_cyclone_skill_shot += Bcd::from_ascii(b"1000000");
            self.score_main += self.party.score_cyclone_skill_shot;
            self.effect(EffectBind::PartyCycloneSkillShot);
            self.silence_effect = true;
            self.party_party(2);
        } else if self.party.timeout_party_pr != 0 {
            self.party_party(2);
        }
        if self.party.cyclone_x5 {
            self.add_cyclone(5);
            self.effect(EffectBind::PartyCycloneX5);
            self.light_set(LightBind::PartyCycloneX5, 0, false);
            self.party.cyclone_x5 = false;
        } else {
            self.add_cyclone(1);
            self.effect(EffectBind::PartyCyclone);
        }
        self.silence_effect = false;
    }
}
