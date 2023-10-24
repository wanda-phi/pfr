use rand::Rng;

use crate::{
    assets::table::{
        lights::LightBind,
        physics::{Layer, PhysmapBind},
        script::{EffectBind, ScriptBind},
        sound::SfxBind,
    },
    bcd::Bcd,
    config::TableId,
};

use super::{show::PrizeState, KbdState, Table};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaskKind {
    SetStartKeysActive,
    PartyOn,
    IssueBall,
    IssueBallFinish,
    IssueBallRelease,
    IssueBallSfx,
    IssueBallRaiseSfx,
    DrainSfx,
    GameOver,
    PartyDropZoneStart(u16),
    PartyDropZoneWait,
    PartyDropZoneRelease,
    PartyDropZoneScroll(u16),
    PartyResetArcadeButton,
    PartyOrbitRightUnblink,
    PartyMadUnblink(u8),
    PartyMadAllUnblink,
    PartySecretDrop,
    PartyCycloneX5Blink,
    PartyCycloneX5End,
    PartyTunnelFreeze,
    PartyArcadePickReward,
    PartyArcadeDropZoneStart(u16, u16, bool),
    PartyDoubleBonusBlink,
    PartyDoubleBonusEnd,
    PartySnacksRelease,
    PartySnacksFinish,
    PartyDemonBlink(u16),
    PartyDemonRelease,
    PartySideExtraBallFinish,
    PartySkyrideUnblink,
    PartyPukeUnblink(u8),
    PartyPukeUnblinkAll,
    PartyDuckDrop(u8),
    PartyDuckUnblink(u8),
    PartyDuckAllUnblink,
    PartyHappyHour,
    PartyMegaLaugh,
    SpeedUnblinkBur(u8),
    SpeedUnblinkBurAll,
    SpeedUnblinkNin(u8),
    SpeedUnblinkNinAll,
    SpeedUnblinkGear(u8),
    SpeedUnblinkGearAll,
    SpeedOffroad,
    SpeedTurbo,
    SpeedPitStop(u16),
    SpeedUnblinkCar,
    SpeedResetSuperJackpot,
    ShowResetDropCenter,
    ShowResetDropLeft,
    ShowUnblinkDollar(u8),
    ShowUnblinkDollarAll,
    ShowVaultEject,
    ShowBillionRelease,
    ShowSpinWheelEnd,
    ShowGivePrize,
    ShowCashpot,
    ShowCashpotEject(u16),
    StonesUnblinkStone(u8),
    StonesUnblinkBone(u8),
    StonesUnblinkStonesBones,
    StonesUnblinkKey(u8),
    StonesUnblinkKeyAll,
    StonesResetSuperJackpot,
    StonesTowerEject,
    StonesTowerEjectNow,
    StonesWellEject,
    StonesVaultEject,
    StonesUnblinkGhosts,
    StonesModeHit,
    StonesModeRamp,
    StonesRaiseKickback,
    StonesUnblinkRip(u8),
    StonesUnblinkRipAll,
    StonesScreamExtra,
}

pub struct Task {
    timer: u16,
    kind: TaskKind,
}

impl Task {
    pub fn run(&mut self, table: &mut Table) -> bool {
        if self.timer != self.kind.delay(table) {
            self.timer += 1;
            return true;
        }
        match self.kind {
            TaskKind::SetStartKeysActive => table.start_keys_active = true,
            TaskKind::PartyOn => {
                table.party_on = true;
                table.issue_ball();
            }
            TaskKind::IssueBall => table.issue_ball(),
            TaskKind::IssueBallFinish => table.issue_ball_finish(),
            TaskKind::IssueBallRelease => table.issue_ball_release(),
            TaskKind::IssueBallSfx => table.play_sfx_bind(SfxBind::IssueBall),
            TaskKind::IssueBallRaiseSfx => table.play_sfx_bind(SfxBind::RaiseHitTargets),
            TaskKind::DrainSfx => table.play_sfx_bind(SfxBind::BallDrained),
            TaskKind::GameOver => {
                table.kbd_state = KbdState::Main;
                table.in_attract = true;
                table.lights.reset();
                table.start_keys_active = true;
                table.score_main = Bcd::ZERO;
                if table.assets.table == TableId::Table1 {
                    table.light_set_all(LightBind::PartyDuckDrop, true);
                }
            }
            TaskKind::PartyDropZoneStart(_) => table.party_start_drop_zone(),
            TaskKind::PartyDropZoneWait => {
                table.light_blink(LightBind::PartyDrop, 0, 7, 0);
                table.light_blink(LightBind::PartyDrop, 1, 7, 0);
                table.add_task(TaskKind::PartyDropZoneRelease);
            }
            TaskKind::PartyDropZoneRelease => {
                table.scroll.reset_special_target();
                table.ball.teleport(
                    Layer::Overhead,
                    (15, 47),
                    (0, rand::thread_rng().gen_range(0..0x80)),
                );
                table.play_sfx_bind(SfxBind::IssueBall);
                table.light_set_all(LightBind::PartyDrop, false);
            }
            TaskKind::PartyDropZoneScroll(ref mut pos) => {
                if *pos >= 5 {
                    *pos -= 5;
                    table.scroll.set_special_target_now(*pos);
                    return true;
                } else {
                    table.scroll.set_special_target_now(0);
                }
            }
            TaskKind::PartyOrbitRightUnblink => {
                table.light_set_all(LightBind::PartyRightOrbitScore, false);
                table.light_blink(LightBind::PartyRightOrbitScore, 0, 9, 0);
                table.party.orbit_right_blinking = false;
            }
            TaskKind::PartyMadUnblink(which) => table.light_set(LightBind::PartyMad, which, true),
            TaskKind::PartyMadAllUnblink => {
                table.light_set_all(LightBind::PartyMad, false);
                table.party.mad_blinking = false;
            }
            TaskKind::PartySecretDrop => {
                if !table.party.secret_drop_release {
                    return true;
                }
                table.add_task(TaskKind::PartyCycloneX5Blink);
                table.party_start_drop_zone();
            }
            TaskKind::PartyCycloneX5Blink => {
                if table.party.cyclone_x5 {
                    table.light_blink(LightBind::PartyCycloneX5, 0, 2, 0);

                    table.add_task(TaskKind::PartyCycloneX5End);
                } else {
                    table.light_set(LightBind::PartyCycloneX5, 0, false);
                }
            }
            TaskKind::PartyCycloneX5End => {
                table.party.cyclone_x5 = false;
                table.light_set(LightBind::PartyCycloneX5, 0, false);
            }
            TaskKind::PartyTunnelFreeze => table.ball.teleport(Layer::Ground, (15, 47), (0, 0)),
            TaskKind::PartyArcadePickReward => {
                if table.in_mode || table.party.arcade_ready {
                    if table.tilted {
                        table.party_start_drop_zone();
                    } else {
                        table.set_music_main();
                        table.sequencer.force_end_loop();
                        table.party_arcade_pick_reward();
                    }
                } else {
                    return true;
                }
            }
            TaskKind::PartyArcadeDropZoneStart(
                ref mut timeout_soft,
                ref mut timeout_hard,
                ref mut jingle_done,
            ) => {
                *timeout_hard -= 1;
                if table.in_mode || *timeout_hard == 0 {
                    table.party_start_drop_zone();
                } else {
                    if *timeout_soft != 0 {
                        *timeout_soft -= 1;
                    }
                    if table.sequencer.jingle_playing() || *jingle_done {
                        *jingle_done = true;
                        if *timeout_soft == 0 {
                            table.party_start_drop_zone();
                        } else {
                            return true;
                        }
                    } else {
                        return true;
                    }
                }
            }
            TaskKind::PartyDoubleBonusBlink => {
                if table.party.orbit_right_db {
                    table.light_blink(LightBind::PartyRightOrbitDoubleBonus, 0, 2, 0);
                    table.add_task(TaskKind::PartyDoubleBonusEnd);
                }
            }
            TaskKind::PartyDoubleBonusEnd => {
                if table.party.orbit_right_db {
                    table.light_set(LightBind::PartyRightOrbitDoubleBonus, 0, false);
                    table.party.orbit_right_db = false;
                }
            }
            TaskKind::PartySnacksRelease => {
                table.play_sfx_bind(SfxBind::PartySnacksRelease);
                table.ball.teleport(Layer::Overhead, (3, 253), (0, -2500));
                table.add_task(TaskKind::PartySnacksFinish);
            }
            TaskKind::PartySnacksFinish => table.party.in_snack = false,
            TaskKind::PartyDemonBlink(_) => {
                table.light_blink(LightBind::PartyDemonHead, 0, 7, 0);
                table.add_task(TaskKind::PartyDemonRelease);
            }
            TaskKind::PartyDemonRelease => {
                table.light_set(LightBind::PartyDemonHead, 0, false);
                table.play_sfx_bind(SfxBind::IssueBall);
                table.ball.teleport(Layer::Ground, (257, 310), (-575, 1575));
                table.party.in_demon = false;
            }
            TaskKind::PartySideExtraBallFinish => table.block_drain = false,
            TaskKind::PartySkyrideUnblink => {
                table.light_set_all(LightBind::PartySkyride, false);
            }
            TaskKind::PartyPukeUnblink(which) => {
                table.light_set(LightBind::PartyPuke, which, true);
                table.party.flipper_lock_puke = false;
            }
            TaskKind::PartyPukeUnblinkAll => table.light_set_all(LightBind::PartyPuke, false),
            TaskKind::PartyResetArcadeButton => table.party.arcade_button_just_hit = false,
            TaskKind::PartyDuckDrop(which) => {
                let bind = match which {
                    0 => PhysmapBind::PartyHitDuck0,
                    1 => PhysmapBind::PartyHitDuck1,
                    2 => PhysmapBind::PartyHitDuck2,
                    _ => unreachable!(),
                };
                table.drop_physmap(bind);
            }
            TaskKind::PartyDuckUnblink(which) => table.light_set(LightBind::PartyDuck, which, true),
            TaskKind::PartyDuckAllUnblink => {
                table.light_set_all(LightBind::PartyDuck, false);
                table.light_set_all(LightBind::PartyDuckDrop, true);
                for bind in [
                    PhysmapBind::PartyHitDuck0,
                    PhysmapBind::PartyHitDuck1,
                    PhysmapBind::PartyHitDuck2,
                ] {
                    table.raise_physmap(bind);
                }
                table.play_sfx_bind(SfxBind::RaiseHitTargets);
                table.party.duck_hit = [false; 3];
            }
            TaskKind::PartyHappyHour => table.party_happy_hour(),
            TaskKind::PartyMegaLaugh => table.party_mega_laugh(),
            TaskKind::SpeedUnblinkBur(which) => {
                if table.speed.blink_bur[which as usize] {
                    table.speed.blink_bur[which as usize] = false;
                    table.light_set(LightBind::SpeedBur, which, true);
                }
            }
            TaskKind::SpeedUnblinkBurAll => {
                table.light_set(LightBind::SpeedGear, 2, true);
                table.light_set_all(LightBind::SpeedBur, false);
                table.speed.blink_bur = [false; 3];
            }
            TaskKind::SpeedUnblinkNin(which) => {
                if table.speed.blink_nin[which as usize] {
                    table.speed.blink_nin[which as usize] = false;
                    table.light_set(LightBind::SpeedNin, which, true);
                }
            }
            TaskKind::SpeedUnblinkNinAll => {
                table.light_set(LightBind::SpeedGear, 3, true);
                table.light_set_all(LightBind::SpeedNin, false);
                table.speed.blink_nin = [false; 3];
            }
            TaskKind::SpeedUnblinkGear(which) => {
                table.light_set(LightBind::SpeedGear, which, true);
            }
            TaskKind::SpeedUnblinkGearAll => table.light_set_all(LightBind::SpeedGear, false),
            TaskKind::SpeedOffroad => {
                if !table.in_drain {
                    if table.in_mode {
                        return true;
                    } else {
                        table.speed_do_offroad();
                    }
                }
            }
            TaskKind::SpeedTurbo => {
                if !table.in_drain {
                    if table.in_mode {
                        return true;
                    } else {
                        table.speed_do_turbo();
                    }
                }
            }
            TaskKind::SpeedPitStop(_) => {
                table.play_sfx_bind(SfxBind::SpeedEjectPit);
                table.ball.teleport(Layer::Ground, (256, 41), (-2100, 800));
            }
            TaskKind::SpeedUnblinkCar => {
                table.light_set_all(LightBind::SpeedCarPart, false);
            }
            TaskKind::SpeedResetSuperJackpot => {
                table.light_set(LightBind::SpeedPitStopSuperJackpot, 0, false)
            }
            TaskKind::ShowResetDropCenter => {
                table.play_sfx_bind(SfxBind::RaiseHitTargets);
                table.light_set_all(LightBind::ShowDropCenter, true);
                table.raise_physmap(PhysmapBind::ShowHitCenter0);
                table.raise_physmap(PhysmapBind::ShowHitCenter1);
            }
            TaskKind::ShowResetDropLeft => {
                table.play_sfx_bind(SfxBind::RaiseHitTargets);
                table.light_set_all(LightBind::ShowDropLeft, true);
                table.raise_physmap(PhysmapBind::ShowHitLeft0);
                table.raise_physmap(PhysmapBind::ShowHitLeft1);
            }
            TaskKind::ShowUnblinkDollar(which) => {
                table.light_set(LightBind::ShowDollar, which, true)
            }
            TaskKind::ShowUnblinkDollarAll => table.light_set_all(LightBind::ShowDollar, false),
            TaskKind::ShowVaultEject => {
                table.play_sfx_bind(SfxBind::IssueBall);
                table.ball.frozen = false;
                table.ball.speed.1 = -3500;
            }
            TaskKind::ShowBillionRelease => {
                table.light_set(LightBind::ShowBillion, 0, false);
                table.light_set_all(LightBind::ShowPrize, false);
                table.show.prizes = [PrizeState::None; 6];
                table.show.prize_sets = 0;
                table.play_sfx_bind(SfxBind::IssueBall);
                table.ball.frozen = false;
                table.ball.speed.1 = -3500;
            }
            TaskKind::ShowSpinWheelEnd => {
                table.scroll.reset_special_target();
                table.score_main += table.show_wheel_score();
                table.start_script(ScriptBind::ShowSpinWheelClear);
                table.ball.frozen = false;
                table.ball.speed.1 = -2916;
                table.light_set(LightBind::ShowSpinWheel, 0, false);
            }
            TaskKind::ShowGivePrize => {
                table.show_give_prize();
            }
            TaskKind::ShowCashpot => {
                table.add_task(TaskKind::ShowCashpotEject(40));
                table.light_set(LightBind::ShowCashpot, 0, true);
            }
            TaskKind::ShowCashpotEject(_) => table.show_cashpot_eject(),
            TaskKind::StonesUnblinkStone(which) => {
                if !table.stones.stones_bones_blinking {
                    table.light_set(LightBind::StonesStone, which, true);
                }
                table.stones.stone_blinking[which as usize] = false;
            }
            TaskKind::StonesUnblinkBone(which) => {
                if !table.stones.stones_bones_blinking {
                    table.light_set(LightBind::StonesBone, which, true);
                }
                table.stones.bone_blinking[which as usize] = false;
            }
            TaskKind::StonesUnblinkStonesBones => {
                table.light_set_all(LightBind::StonesStone, false);
                table.light_set_all(LightBind::StonesBone, false);
                table.stones.stones_bones_blinking = false;
            }
            TaskKind::StonesUnblinkKey(which) => {
                if !table.stones.key_blinking {
                    table.light_set(LightBind::StonesKey, which, true);
                    table.stones.flipper_lock_key = false;
                }
            }
            TaskKind::StonesUnblinkKeyAll => {
                table.light_set_all(LightBind::StonesKey, false);
                table.stones.key_blinking = false;
                table.stones.flipper_lock_key = false;
            }
            TaskKind::StonesResetSuperJackpot => {
                if table.stones.tower_super_jackpot {
                    table.stones.tower_super_jackpot = false;
                    table.light_set(LightBind::StonesTowerSuperJackpot, 0, false);
                    table.stones_tower_check_close();
                }
            }
            TaskKind::StonesTowerEject => {
                table.timer_stop = false;
                if table.stones.tower_resume_mode {
                    if table.stones.tower_resume_mode_ramp {
                        table.start_script(ScriptBind::StonesModeRampContinue);
                    } else {
                        table.start_script(ScriptBind::StonesModeHitContinue);
                    }
                }
                table.stones_tower_eject();
            }
            TaskKind::StonesTowerEjectNow => table.stones_tower_eject(),
            TaskKind::StonesWellEject => {
                table.play_sfx_bind(SfxBind::StonesEject);
                table.ball.teleport(Layer::Ground, (275, 245), (-666, 1666));
                table.stones.in_well = false;
            }
            TaskKind::StonesVaultEject => {
                table.play_sfx_bind(SfxBind::StonesEject);
                table.drop_physmap(PhysmapBind::StonesGateKickback);
                table.ball.teleport(Layer::Ground, (2, 532), (0, -2880));
                table.stones.in_vault = false;
                table.add_task(TaskKind::StonesRaiseKickback);
            }
            TaskKind::StonesUnblinkGhosts => {
                table.light_set_all(LightBind::StonesGhost, false);
                table.stones.ghosts_blinking = false;
            }
            TaskKind::StonesModeHit => {
                if table.in_mode {
                    return true;
                }
                if !table.in_drain {
                    if table.stones.in_vault {
                        table.stones.vault_hold = true;
                    }
                    table.effect(EffectBind::StonesGhostGhostHunter);
                }
            }
            TaskKind::StonesModeRamp => {
                if table.in_mode {
                    return true;
                }
                if !table.in_drain {
                    table.stones.vault_hold = true;
                    table.effect(EffectBind::StonesGhostGrimReaper);
                }
            }
            TaskKind::StonesRaiseKickback => {
                if !table.stones.kickback {
                    table.raise_physmap(PhysmapBind::StonesGateKickback);
                }
                table.stones.vault_from_ramp = false;
            }
            TaskKind::StonesUnblinkRip(which) => {
                if !table.stones.rip_blinking {
                    table.light_set(LightBind::StonesRip, which, true);
                    table.stones.flipper_lock_rip = false;
                }
            }
            TaskKind::StonesUnblinkRipAll => {
                table.light_set_all(LightBind::StonesRip, false);
                table.stones.rip_blinking = false;
                table.stones.flipper_lock_rip = false;
            }
            TaskKind::StonesScreamExtra => table.stones_ramp_screams(),
        }
        false
    }
}

impl TaskKind {
    pub fn delay(self, table: &Table) -> u16 {
        match self {
            TaskKind::SetStartKeysActive => 15,
            TaskKind::PartyOn => 30,
            TaskKind::IssueBall => 30,
            TaskKind::IssueBallFinish => 30,
            TaskKind::IssueBallRelease => 80,
            TaskKind::IssueBallSfx => 45,
            TaskKind::IssueBallRaiseSfx => 5,
            TaskKind::DrainSfx => 5,
            TaskKind::GameOver => 0,
            TaskKind::PartyDropZoneStart(delay) => delay,
            TaskKind::PartyDropZoneWait => 30,
            TaskKind::PartyDropZoneRelease => 27,
            TaskKind::PartyDropZoneScroll(_) => 0,
            TaskKind::PartyOrbitRightUnblink => 120,
            TaskKind::PartyMadUnblink(_) => 14,
            TaskKind::PartyMadAllUnblink => 120,
            TaskKind::PartySecretDrop => 0,
            TaskKind::PartyCycloneX5Blink => 480,
            TaskKind::PartyCycloneX5End => 120,
            TaskKind::PartyTunnelFreeze => 2,
            TaskKind::PartyArcadePickReward => 0,
            TaskKind::PartyArcadeDropZoneStart(_, _, _) => 0,
            TaskKind::PartyDoubleBonusBlink => 480,
            TaskKind::PartyDoubleBonusEnd => 120,
            TaskKind::PartySnacksRelease => {
                if table.in_mode {
                    40
                } else {
                    130
                }
            }
            TaskKind::PartySnacksFinish => 60,
            TaskKind::PartyDemonBlink(delay) => delay,
            TaskKind::PartyDemonRelease => 27,
            TaskKind::PartySideExtraBallFinish => 600,
            TaskKind::PartySkyrideUnblink => 120,
            TaskKind::PartyPukeUnblink(_) => 13,
            TaskKind::PartyPukeUnblinkAll => 100,
            TaskKind::PartyResetArcadeButton => 20,
            TaskKind::PartyDuckDrop(_) => 20,
            TaskKind::PartyDuckUnblink(_) => 13,
            TaskKind::PartyDuckAllUnblink => 71,
            TaskKind::PartyHappyHour => 400,
            TaskKind::PartyMegaLaugh => 400,
            TaskKind::SpeedUnblinkBur(_) => 10,
            TaskKind::SpeedUnblinkBurAll => 40,
            TaskKind::SpeedUnblinkNin(_) => 10,
            TaskKind::SpeedUnblinkNinAll => 40,
            TaskKind::SpeedUnblinkGear(_) => 10,
            TaskKind::SpeedUnblinkGearAll => 45,
            TaskKind::SpeedTurbo => 0,
            TaskKind::SpeedOffroad => 0,
            TaskKind::SpeedPitStop(delay) => delay,
            TaskKind::SpeedUnblinkCar => 120,
            TaskKind::SpeedResetSuperJackpot => 1200,
            TaskKind::ShowResetDropCenter => 60,
            TaskKind::ShowResetDropLeft => 60,
            TaskKind::ShowUnblinkDollar(_) => 25,
            TaskKind::ShowUnblinkDollarAll => 60,
            TaskKind::ShowVaultEject => 30,
            TaskKind::ShowBillionRelease => 250,
            TaskKind::ShowSpinWheelEnd => 100,
            TaskKind::ShowGivePrize => 0,
            TaskKind::ShowCashpot => 160,
            TaskKind::ShowCashpotEject(delay) => delay,
            TaskKind::StonesUnblinkStone(_) => 10,
            TaskKind::StonesUnblinkBone(_) => 10,
            TaskKind::StonesUnblinkStonesBones => 70,
            TaskKind::StonesUnblinkKey(_) => 10,
            TaskKind::StonesUnblinkKeyAll => 70,
            TaskKind::StonesResetSuperJackpot => 780,
            TaskKind::StonesTowerEject => 10,
            TaskKind::StonesTowerEjectNow => 0,
            TaskKind::StonesWellEject => 10,
            TaskKind::StonesVaultEject => 10,
            TaskKind::StonesUnblinkGhosts => 240,
            TaskKind::StonesModeHit => 0,
            TaskKind::StonesModeRamp => 0,
            TaskKind::StonesRaiseKickback => 30,
            TaskKind::StonesUnblinkRip(_) => 20,
            TaskKind::StonesUnblinkRipAll => 70,
            TaskKind::StonesScreamExtra => 2,
        }
    }
}

impl Table {
    pub fn add_task(&mut self, kind: TaskKind) {
        self.tasks.push(Task { timer: 0, kind });
    }

    pub fn tasks_frame(&mut self) {
        let mut tasks = core::mem::take(&mut self.tasks);
        tasks.retain_mut(|task| task.run(self));
        self.tasks.extend(tasks);
    }
}
