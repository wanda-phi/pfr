use enum_map::{enum_map, Enum, EnumMap};

use crate::{assets::mz::MzExe, config::TableId};

#[derive(Copy, Clone, Debug)]
pub struct Jingle {
    pub position: u8,
    pub repeat: u8,
    pub priority: u8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Enum)]
pub enum JingleBind {
    // General
    Silence,
    GameStart,
    Plunger,
    Main,
    Attract,
    WarnTilt,
    Tilt,
    GameOverSad,
    GameOverHighScore,
    Drained,
    MatchStart,
    MatchWin,
    // General, not always present
    ModeEndHit,
    ModeEndRamp,
    // Partyland
    PartyJackpot,
    // Speed Devils
    SpeedModeHit,
    // Billion Dollar Game Show
    ShowSpinWheel,
    ShowMultiBonus,
    ShowJackpot,
    ShowExtraBallLit,
    ShowPrizeIncoming,
    // Stones and Bones
    StonesTowerHuntEnd,
}

#[derive(Copy, Clone, Debug)]
pub struct Sfx {
    pub sample: u8,
    pub period: u8,
    pub channel: u8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Enum)]
pub enum SfxBind {
    // General
    FlipperPress,
    BallDrained,
    IssueBall,
    SpringUp,
    RollInner,
    TickBonus,
    GameStart,
    // General, not always present
    RollTrigger,
    RaiseHitTargets,
    // Partyland
    PartySnacksRelease,
    PartyHitDuck,
    PartyArcadeButton,
    // Speed Devils
    SpeedEjectPit,
    SpeedHitTarget,
    // Billion Dollar Game Show
    ShowEjectCashpot,
    ShowHitTrigger,
    // Stones and Bones
    StonesEject,
    StonesHitStone,
    StonesHitBone,
}

pub(super) fn extract_jingle(exe: &MzExe, off: u16) -> Jingle {
    Jingle {
        position: exe.data_byte(off),
        repeat: exe.data_byte(off + 1),
        priority: exe.data_byte(off + 2),
    }
}

pub(super) fn extract_sfx(exe: &MzExe, off: u16) -> Sfx {
    assert_eq!(exe.data_byte(off + 2), 0);
    Sfx {
        sample: exe.data_byte(off),
        period: exe.data_byte(off + 1),
        channel: exe.data_byte(off + 3),
    }
}

pub(super) fn extract_jingle_binds(
    exe: &MzExe,
    table: TableId,
) -> EnumMap<JingleBind, Option<Jingle>> {
    enum_map! {
        bind => {
            let off = match table {
                TableId::Table1 => match bind {
                    JingleBind::Silence => Some(0xc6c),
                    JingleBind::Plunger => Some(0xc6f),
                    JingleBind::Main => Some(0xc72),
                    JingleBind::Attract => Some(0xc75),
                    JingleBind::WarnTilt => Some(0xc78),
                    JingleBind::Tilt => Some(0xc7b),
                    JingleBind::GameOverSad => Some(0xc7e),
                    JingleBind::GameOverHighScore => Some(0xc81),
                    JingleBind::Drained => Some(0xc84),
                    JingleBind::GameStart => Some(0xc87),
                    JingleBind::MatchStart => Some(0xc90),
                    JingleBind::MatchWin => Some(0xc93),
                    JingleBind::PartyJackpot => Some(0xca2),
                    _ => None,
                },
                TableId::Table2 => match bind {
                    JingleBind::Silence => Some(0xa3e),
                    JingleBind::Plunger => Some(0xa41),
                    JingleBind::GameStart => Some(0xa41),
                    JingleBind::Main => Some(0xa44),
                    JingleBind::Attract => Some(0xa47),
                    JingleBind::WarnTilt => Some(0xa4a),
                    JingleBind::Tilt => Some(0xa4d),
                    JingleBind::GameOverSad => Some(0xa50),
                    JingleBind::GameOverHighScore => Some(0xa53),
                    JingleBind::Drained => Some(0xa56),
                    JingleBind::MatchStart => Some(0xa62),
                    JingleBind::MatchWin => Some(0xa65),
                    JingleBind::ModeEndRamp => Some(0xaa1),
                    JingleBind::SpeedModeHit => Some(0xaa7),
                    JingleBind::ModeEndHit => Some(0xaaa),
                    _ => None,
                },
                TableId::Table3 => match bind {
                    JingleBind::Silence => Some(0x8ad),
                    JingleBind::GameStart => Some(0x8de),
                    JingleBind::Plunger => Some(0x8de),
                    JingleBind::Main => Some(0x8d2),
                    JingleBind::Attract => Some(0x8e1),
                    JingleBind::WarnTilt => Some(0x8b0),
                    JingleBind::Tilt => Some(0x8b3),
                    JingleBind::GameOverSad => Some(0x8d5),
                    JingleBind::GameOverHighScore => Some(0x8d8),
                    JingleBind::Drained => Some(0x8b6),
                    JingleBind::MatchStart => Some(0x8c3),
                    JingleBind::MatchWin => Some(0x8c6),
                    JingleBind::ModeEndHit => Some(0x8fd),
                    JingleBind::ModeEndRamp => Some(0x8fd),
                    JingleBind::ShowSpinWheel => Some(0x8c0),
                    JingleBind::ShowMultiBonus => Some(0x8e8),
                    JingleBind::ShowJackpot => Some(0x8eb),
                    JingleBind::ShowExtraBallLit => Some(0x918),
                    JingleBind::ShowPrizeIncoming => Some(0x91e),
                    _ => None,
                },
                TableId::Table4 => match bind {
                    JingleBind::Silence => Some(0x8ec),
                    JingleBind::GameStart => Some(0x8ef),
                    JingleBind::Plunger => Some(0x8f2),
                    JingleBind::Main => Some(0x8f5),
                    JingleBind::Attract => Some(0x8f8),
                    JingleBind::WarnTilt => Some(0x8fb),
                    JingleBind::Tilt => Some(0x8fe),
                    JingleBind::GameOverSad => Some(0x901),
                    JingleBind::GameOverHighScore => Some(0x904),
                    JingleBind::Drained => Some(0x907),
                    JingleBind::MatchWin => Some(0x90d),
                    JingleBind::MatchStart => Some(0x910),
                    JingleBind::ModeEndHit => Some(0x95b),
                    JingleBind::ModeEndRamp => Some(0x95b),
                    JingleBind::StonesTowerHuntEnd => Some(0x94f),
                    _ => None,
                },
            };
            off.map(|off| extract_jingle(exe, off))
        }
    }
}

pub(super) fn extract_sfx_binds(exe: &MzExe, table: TableId) -> EnumMap<SfxBind, Option<Sfx>> {
    enum_map! {
        bind => {
            let off = match table {
                TableId::Table1 => match bind {
                    SfxBind::FlipperPress => Some(0xc2d),
                    SfxBind::BallDrained => Some(0xc31),
                    SfxBind::IssueBall => Some(0xc35),
                    SfxBind::SpringUp => Some(0xc3d),
                    SfxBind::PartySnacksRelease => Some(0xc41),
                    SfxBind::RollTrigger => Some(0xc45),
                    SfxBind::RollInner => Some(0xc49),
                    SfxBind::TickBonus => Some(0xc61),
                    SfxBind::GameStart => Some(0xc65),
                    SfxBind::RaiseHitTargets => Some(0xc1d),
                    SfxBind::PartyHitDuck => Some(0xc19),
                    SfxBind::PartyArcadeButton => Some(0xc5d),
                    _ => None,
                },
                TableId::Table2 => match bind {
                    SfxBind::FlipperPress => Some(0x9fa),
                    SfxBind::BallDrained => Some(0x9fe),
                    SfxBind::IssueBall => Some(0xa02),
                    SfxBind::SpringUp => Some(0xa0a),
                    SfxBind::RollInner => Some(0xa1a),
                    SfxBind::TickBonus => Some(0xa36),
                    SfxBind::GameStart => Some(0xa12),
                    SfxBind::RaiseHitTargets => Some(0x9f2),
                    SfxBind::SpeedEjectPit => Some(0xa0e),
                    SfxBind::SpeedHitTarget => Some(0xa2a),
                    _ => None,
                },
                TableId::Table3 => match bind {
                    SfxBind::FlipperPress => Some(0x86d),
                    SfxBind::BallDrained => Some(0x871),
                    SfxBind::IssueBall => Some(0x875),
                    SfxBind::SpringUp => Some(0x87d),
                    SfxBind::RollTrigger => Some(0x885),
                    SfxBind::RollInner => Some(0x889),
                    SfxBind::TickBonus => Some(0x8a5),
                    SfxBind::GameStart => Some(0x8b9),
                    SfxBind::RaiseHitTargets => Some(0x865),
                    SfxBind::ShowEjectCashpot => Some(0x881),
                    SfxBind::ShowHitTrigger => Some(0x89d),
                    _ => None,
                },
                TableId::Table4 => match bind {
                    SfxBind::FlipperPress => Some(0x8b8),
                    SfxBind::BallDrained => Some(0x8bc),
                    SfxBind::IssueBall => Some(0x8c0),
                    SfxBind::SpringUp => Some(0x8c8),
                    SfxBind::RollTrigger => Some(0x8d0),
                    SfxBind::RollInner => Some(0x8d4),
                    SfxBind::TickBonus => Some(0x8e4),
                    SfxBind::GameStart => Some(0x8e8),
                    SfxBind::StonesEject => Some(0x8cc),
                    SfxBind::StonesHitStone => Some(0x8d8),
                    SfxBind::StonesHitBone => Some(0x8dc),
                    _ => None,
                },
            };
            off.map(|off| extract_sfx(exe, off))
        }
    }
}
