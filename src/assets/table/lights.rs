use enum_map::{enum_map, Enum, EnumMap};
use unnamed_entity::{entity_id, EntityId, EntityVec};

use crate::{assets::mz::MzExe, config::TableId};

entity_id! {
    pub id LightId u8;
    pub id AttractLightId u8;
}

#[derive(Clone, Debug)]
pub struct Light {
    pub base_index: u8,
    pub colors: Vec<(u8, u8, u8)>,
}

#[derive(Clone, Debug)]
pub struct AttractLight {
    pub ctr_reset: u16,
    pub ctr_off: u16,
    pub ctr_on: u16,
    pub light: LightId,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum)]
pub enum LightBind {
    PartyPuke,
    PartyDrop,
    PartyMad,
    PartyTunnel,
    PartyCycloneX5,
    PartySkyride,
    PartyDuck,
    PartyDuckDrop,
    PartySnack,
    PartyRightOrbitScore,
    PartyRightOrbitMultiBonus,
    PartyRightOrbitDoubleBonus,
    PartyRightOrbitHoldBonus,
    PartyDemonHead,
    PartyDemon5M,
    PartyDemonExtraBall,
    PartyDemonJackpot,
    PartyParty,
    PartyCrazy,
    PartyHappyHour,
    PartyMegaLaugh,
    PartySideExtraBall,
    PartyBonus,
    PartyExtraBall,
    PartyArcade,
    SpeedPitLoopExtraBall,
    SpeedPitStopHoldBonus,
    SpeedPitStopSuperJackpot,
    SpeedOffroadMultiBonus,
    SpeedMiniRampJump,
    SpeedPit,
    SpeedPitStopGoal,
    SpeedCarPartLit,
    SpeedMiniRampJackpot,
    SpeedBur,
    SpeedNin,
    SpeedGear,
    SpeedGearNum,
    SpeedPlace,
    SpeedBonus,
    SpeedCarPart,
    SpeedExtraBall,
    SpeedSpeed,
    ShowSkills,
    ShowDollar,
    ShowTopLoop,
    ShowSuperJackpot,
    ShowCashpot,
    ShowDropLeft,
    ShowDropCenter,
    ShowOrbitExtraBall,
    ShowCashpotX5,
    ShowPrize,
    ShowJackpot,
    ShowCollectPrize,
    ShowSpinWheel,
    ShowWheel,
    ShowBillion,
    ShowExtraBall,
    ShowMoneyMania,
    ShowBonus,
    StonesKey,
    StonesRip,
    StonesTower,
    StonesTowerExtraBall,
    StonesTowerJackpot,
    StonesTowerSuperJackpot,
    StonesTowerMillion,
    StonesTower5M,
    StonesTowerDoubleBonus,
    StonesTowerHoldBonus,
    StonesVaultLock,
    StonesVaultGhost,
    StonesScreamX2,
    StonesScreamDemon,
    StonesMillionPlus,
    StonesWellLock,
    StonesWellMultiBonus,
    StonesBone,
    StonesStone,
    StonesGhost,
    StonesBonus,
    StonesKickback,
}

pub(super) fn extract_attract_lights(
    exe: &MzExe,
    table: TableId,
) -> EntityVec<AttractLightId, AttractLight> {
    let mut res = EntityVec::new();
    let mut pos = match table {
        TableId::Table1 => 0xf41,
        TableId::Table2 => 0xcb9,
        TableId::Table3 => 0xb54,
        TableId::Table4 => 0xf1a,
    };
    while exe.data_word(pos) != 0xffff {
        assert_eq!(exe.data_word(pos), 0);
        let ctr_reset = exe.data_word(pos + 2);
        let ctr_dim = ctr_reset + exe.data_word(pos + 4);
        let ctr_lit = ctr_dim + exe.data_word(pos + 6);
        let light = LightId::from_idx((exe.data_word(pos + 8) - 1).into());
        res.push(AttractLight {
            ctr_reset,
            ctr_off: ctr_dim,
            ctr_on: ctr_lit,
            light,
        });
        pos += 10;
    }
    res
}

pub(super) fn extract_light_binds(table: TableId) -> EnumMap<LightBind, Vec<LightId>> {
    let light_sets: &[(_, &[_])] = match table {
        TableId::Table1 => &[
            (LightBind::PartyPuke, &[0x01, 0x04, 0x05, 0x02]),
            (LightBind::PartyDrop, &[0x03, 0x38]),
            (LightBind::PartyMad, &[0x06, 0x08, 0x09]),
            (LightBind::PartyArcade, &[0x07, 0x37]),
            (LightBind::PartyTunnel, &[0x0e, 0x0c, 0x0a]),
            (LightBind::PartyCycloneX5, &[0x0b]),
            (LightBind::PartySkyride, &[0x16, 0x0d, 0x0f]),
            (LightBind::PartyDuck, &[0x10, 0x12, 0x18]),
            (LightBind::PartyDuckDrop, &[0x34, 0x35, 0x36]),
            (LightBind::PartyRightOrbitScore, &[0x1a, 0x19, 0x13]),
            (LightBind::PartySnack, &[0x17, 0x11, 0x14]),
            (LightBind::PartyDemonHead, &[0x15]),
            (LightBind::PartyDemon5M, &[0x1b]),
            (LightBind::PartyDemonExtraBall, &[0x1e]),
            (LightBind::PartyDemonJackpot, &[0x23]),
            (LightBind::PartyCrazy, &[0x29, 0x26, 0x22, 0x1f, 0x1c]),
            (LightBind::PartyRightOrbitMultiBonus, &[0x1d]),
            (LightBind::PartyRightOrbitHoldBonus, &[0x21]),
            (LightBind::PartyRightOrbitDoubleBonus, &[0x24]),
            (LightBind::PartyHappyHour, &[0x20]),
            (LightBind::PartyMegaLaugh, &[0x25]),
            (LightBind::PartySideExtraBall, &[0x27]),
            (LightBind::PartyParty, &[0x2a, 0x2b, 0x2c, 0x2d, 0x2e]),
            (LightBind::PartyBonus, &[0x2f, 0x31, 0x32, 0x30]),
            (LightBind::PartyExtraBall, &[0x33]),
        ],
        TableId::Table2 => &[
            (LightBind::SpeedPitLoopExtraBall, &[0x01]),
            (LightBind::SpeedPitStopHoldBonus, &[0x02]),
            (LightBind::SpeedPitStopSuperJackpot, &[0x03]),
            (LightBind::SpeedOffroadMultiBonus, &[0x04]),
            (LightBind::SpeedMiniRampJump, &[0x05]),
            (LightBind::SpeedPit, &[0x06, 0x07, 0x08]),
            (LightBind::SpeedPitStopGoal, &[0x09]),
            (LightBind::SpeedCarPartLit, &[0x0b, 0x0d, 0x0e, 0x0c, 0x0a]),
            (LightBind::SpeedMiniRampJackpot, &[0x0f]),
            (LightBind::SpeedBur, &[0x10, 0x11, 0x12]),
            (LightBind::SpeedNin, &[0x13, 0x14, 0x15]),
            (LightBind::SpeedGear, &[0x16, 0x17, 0x18, 0x19]),
            (
                LightBind::SpeedGearNum,
                &[0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],
            ),
            (
                LightBind::SpeedPlace,
                &[0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29],
            ),
            (
                LightBind::SpeedBonus,
                &[0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31],
            ),
            (LightBind::SpeedCarPart, &[0x32, 0x35, 0x34, 0x33, 0x36]),
            (LightBind::SpeedExtraBall, &[0x37]),
            (
                LightBind::SpeedSpeed,
                &[
                    0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43,
                ],
            ),
        ],
        TableId::Table3 => &[
            (LightBind::ShowSkills, &[0x01]),
            (LightBind::ShowDollar, &[0x02, 0x03]),
            (LightBind::ShowTopLoop, &[0x04]),
            (LightBind::ShowSuperJackpot, &[0x05]),
            (LightBind::ShowCashpot, &[0x06]),
            (LightBind::ShowDropCenter, &[0x07, 0x08]),
            (LightBind::ShowDropLeft, &[0x09, 0x0a]),
            (LightBind::ShowOrbitExtraBall, &[0x0b]),
            (LightBind::ShowCashpotX5, &[0x0c]),
            (LightBind::ShowPrize, &[0x0d, 0x0e, 0x0f, 0x1c, 0x1d, 0x1e]),
            (LightBind::ShowJackpot, &[0x10]),
            (LightBind::ShowCollectPrize, &[0x11]),
            (LightBind::ShowSpinWheel, &[0x12]),
            (
                LightBind::ShowWheel,
                &[0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a],
            ),
            (LightBind::ShowBillion, &[0x1b]),
            (LightBind::ShowExtraBall, &[0x1f]),
            (LightBind::ShowMoneyMania, &[0x20]),
            (LightBind::ShowBonus, &[0x21, 0x22, 0x23, 0x24, 0x25, 0x26]),
        ],
        TableId::Table4 => &[
            (LightBind::StonesKey, &[0x01, 0x02, 0x03]),
            (LightBind::StonesRip, &[0x04, 0x05, 0x06]),
            (LightBind::StonesTower, &[0x07]),
            (LightBind::StonesTowerExtraBall, &[0x08]),
            (LightBind::StonesTowerJackpot, &[0x09]),
            (LightBind::StonesTowerSuperJackpot, &[0x0a]),
            (LightBind::StonesTowerMillion, &[0x0b]),
            (LightBind::StonesTower5M, &[0x0c]),
            (LightBind::StonesTowerHoldBonus, &[0x0d]),
            (LightBind::StonesTowerDoubleBonus, &[0x0e]),
            (LightBind::StonesVaultLock, &[0x0f]),
            (LightBind::StonesVaultGhost, &[0x10]),
            (LightBind::StonesScreamX2, &[0x11]),
            (LightBind::StonesScreamDemon, &[0x12]),
            (LightBind::StonesMillionPlus, &[0x13]),
            (LightBind::StonesBone, &[0x14, 0x15, 0x16, 0x17]),
            (LightBind::StonesWellMultiBonus, &[0x18]),
            (LightBind::StonesWellLock, &[0x19]),
            (LightBind::StonesStone, &[0x1a, 0x1b, 0x1c, 0x1d, 0x1e]),
            (
                LightBind::StonesGhost,
                &[0x20, 0x22, 0x23, 0x25, 0x24, 0x1f, 0x21, 0x26],
            ),
            (LightBind::StonesBonus, &[0x27, 0x28, 0x29, 0x2a, 0x2b]),
            (LightBind::StonesKickback, &[0x2c]),
        ],
    };
    let mut res = enum_map! { _ => vec![] };
    for &(bind, lights) in light_sets {
        res[bind] = lights.iter().map(|&x| LightId::from_idx(x - 1)).collect()
    }
    res
}
