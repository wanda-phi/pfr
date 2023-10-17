use std::collections::HashMap;

use enum_map::{enum_map, Enum, EnumMap};
use unnamed_entity::{entity_id, EntityMap, EntityVec};

use crate::{assets::mz::MzExe, bcd::Bcd, config::TableId};

use super::{
    dm::DmFont,
    sound::{extract_jingle, extract_sfx, Jingle, Sfx},
};

entity_id! {
    pub id ScriptPosId u16, delta;
    pub id DmAnimId u8;
    pub id DmAnimFrameId u8;
    pub id MsgId u16;
}

#[derive(Copy, Clone, Debug)]
pub enum Uop {
    End,

    Noop,
    Delay(u16),
    DelayIfMultiplayer(u16),
    Halt,
    Jump(ScriptPosId),
    JccScoreZero(ScriptScore, ScriptPosId),
    JccNoBonusMult(ScriptPosId),
    RepeatSetup(u16),
    RepeatLoop(u16, ScriptPosId),
    FinalScoreSetup,
    FinalScoreLoop(ScriptPosId),
    ConfirmQuit,

    WaitWhileGameStarting,
    ExtraBall,
    SetupPartyOn,
    SetupShootAgain,
    SetSpecialPlungerEvent,
    IssueBall,

    MultiplyBonus,
    AccBonusCyclones,
    AccBonusModeHit,
    AccBonusModeRamp,
    AccBonus,
    CheckTopScore,
    NextBallIfMatched,
    NextBall,

    Match,
    CheckMatch,
    RecordHighScores,
    GameOver,

    PlaySfx(Sfx, u8),
    PlayJingle(Jingle),
    SetMusic(u8),
    SetJingleTimeout(u16),
    WaitJingle,
    WaitJingleTimeout,

    ModeContinue(u8, ScriptScore),
    ModeStart(u8, ScriptScore),
    ModeStartOrContinue(u8, ScriptScore),

    DmBlink(u16),
    DmStopBlink,
    DmState(bool),
    DmClear,
    DmWipeDown,
    DmWipeRight,
    DmWipeDownStriped,
    DmAnim(DmAnimId),
    DmPuts(DmFont, DmCoord, MsgId),
    DmPrintScore(DmFont, bool, DmCoord, ScriptScore),
    DmMsgScrollUp(MsgId, i16),
    DmMsgScrollDown(MsgId, i16),
    DmLongMsg(MsgId),
    DmTowerHunt(u16),

    PartyArcadeReady,
    PartySecretDrop,

    SpeedStartTurbo,
    SpeedCheckTurboCont,
    SpeedClearFlagMode,

    ShowSpinWheelEnd,
    ShowBlinkMoneyMania,
    ShowEndMoneyMania,

    StonesTowerEject,
    StonesVaultEject,
    StonesWellEject,
    StonesTiltEject,
    StonesSetFlagMode,
    StonesSetFlagModeRamp,
    StonesSetFlagModeHit,
    StonesClearFlagMode,
    StonesClearFlagModeRamp,
    StonesClearFlagModeHit,
    StonesEndMode,
    StonesEndGrimReaper,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DmCoord {
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScriptScore {
    Bonus,
    ModeHit,
    ModeRamp,
    Jackpot,
    HighScore(usize),
    Const(Bcd),
    CycloneIncr,
    NumCyclone,
    CycloneBonus,
    PartyTunnelSkillShot,
    PartyCycloneSkillShot,
    ShowRaisingMillions,
    ShowSpinWheel,
    ShowCashpot,
    ShowCashpotX5,
    StonesSkillShot,
    StonesMillionPlus,
    StonesVault,
    StonesWell,
    StonesTowerBonus,
}

#[derive(Clone, Debug)]
pub struct DmAnim {
    pub repeats: u16,
    pub restart: usize,
    pub num_frames: usize,
    pub frames: Vec<(DmAnimFrameId, u16)>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum)]
pub enum ScriptBind {
    Init,
    Attract,
    GameStart,
    GameStartPlayers,
    PartyOn,
    ShootAgain,
    Enter,
    Main,
    GameIdle,
    Tilt,
    TopScoreInterball,
    TopScoreIngame,
    Match,
    CheckMatch,
    PostMatch,
    GameOver,
    ConfirmQuit,

    PartyJackpot,
    PartyJackpotModeHit,
    PartyJackpotModeRamp,

    SpeedModeHit,
    SpeedModeRampContinue,
    SpeedModeRamp,

    ShowHintLoopRight,
    ShowHintLoopLeft,
    ShowMbX2,
    ShowMbX3,
    ShowMbX4,
    ShowMbX6,
    ShowMbX8,
    ShowMbX10,
    ShowSpinWheelBlink,
    ShowSpinWheelClear,
    ShowSpinWheelClearHalt,
    ShowSpinWheelScore,

    StonesModeHitContinue,
    StonesModeRampContinue,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CheatEffect {
    None,
    Tilt,
    Slowdown,
    Balls,
    Reset,
}

#[derive(Debug, Clone)]
pub struct Cheat {
    pub keys: Box<[u8]>,
    pub script: ScriptPosId,
    pub effect: CheatEffect,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum)]
pub enum EffectBind {
    Drained,

    PartyArcadeSideExtraBall,
    PartyArcade5M,
    PartyArcade1M,
    PartyArcade500k,
    PartyArcadeNoScore,
    PartyArcade,
    PartyPartyP,
    PartyPartyA,
    PartyPartyR,
    PartyPartyT,
    PartyPartyY,
    PartyTunnel1M,
    PartyTunnel3M,
    PartyTunnel5M,
    PartyOrbit250k,
    PartyOrbit500k,
    PartyOrbit750k,
    PartyDemon250k,
    PartyDemon5M,
    PartyDemonExtraBall,
    PartyDuckAll,
    PartySnackNope,
    PartySnack0,
    PartySnack1,
    PartySnack2,
    PartyOrbitMb2,
    PartyOrbitMb4,
    PartyOrbitMb6,
    PartyOrbitMb8,
    PartyOrbitHoldBonus,
    PartyOrbitDoubleBonus,
    PartySideExtraBall,
    PartyOrbitCrazy,
    PartyArcadeCrazy,
    PartyOrbitMad0,
    PartyOrbitMad1,
    PartyOrbitMad2,
    PartySkyride0,
    PartySkyride1,
    PartySkyride2,
    PartySkyrideLitMb,
    PartyCyclone,
    PartyCycloneX5,
    PartySecret,
    PartyCycloneSkillShot,
    PartyTunnelSkillShot,
    PartyRollInner,
    PartyHappyHour,
    PartyHappyHourEnd,
    PartyMegaLaugh,
    PartyMegaLaughEnd,
    SpeedTurboRamp,
    SpeedMilesToJump,
    SpeedMilesToFirstOffroad,
    SpeedMilesToExtraBall,
    SpeedMilesToOffroad,
    SpeedSuperJackpot,
    SpeedJackpot,
    SpeedSuperJackpotGoal,
    SpeedHoldBonus,
    SpeedExtraGear,
    SpeedExtraBall,
    SpeedMilesExtraBall,
    SpeedJump,
    SpeedMilesJump,
    SpeedCar0,
    SpeedCar1,
    SpeedCar2,
    SpeedCar3,
    SpeedCar4,
    SpeedGear,
    SpeedPedalMetal,
    SpeedOvertake,
    SpeedOvertakeFinal,
    SpeedTurbo,
    SpeedLaneOuter,
    SpeedLaneInner,
    SpeedPit,
    SpeedPitAll,
    SpeedOffroadExit,
    SpeedRampOffroad,
    SpeedMillion,
    SpeedMiles0,
    SpeedMiles1,
    SpeedMiles2,
    SpeedMiles3,
    SpeedMiles4,
    SpeedMiles5,
    SpeedMiles6,
    SpeedMiles7,
    SpeedMiles8,
    SpeedMiles9,
    SpeedMiles10,
    SpeedMiles11,
    SpeedMb2,
    SpeedMb3,
    SpeedMb4,
    SpeedMb5,
    SpeedMb6,
    SpeedMb7,
    SpeedMb8,
    SpeedMb9,
    ShowCashpotLock,
    ShowBillion,
    ShowLaneOuter,
    ShowLaneInner,
    ShowRampRight,
    ShowRampTop,
    ShowRampLoop,
    ShowTopEntry,
    ShowSkillsEntry,
    ShowRampSkills,
    ShowOrbitLeft,
    ShowOrbitRight,
    ShowLoopEntry,
    ShowPrizeTv,
    ShowPrizeTrip,
    ShowPrizeCar,
    ShowPrizeBoat,
    ShowPrizeHouse,
    ShowPrizePlane,
    ShowModeHit,
    ShowModeRamp,
    ShowJackpot,
    ShowSuperJackpot,
    ShowExtraBall,
    ShowRaisingMillions,
    ShowSkillsToMoneyMania,
    ShowSkillsToExtraBall,
    ShowCashpot,
    ShowCashpotX5,
    ShowDropCenter,
    ShowDropLeft,
    ShowDollar,
    ShowDollarBoth,
    ShowRampTopTwice,
    ShowLitTv,
    ShowLitTrip,
    ShowLitCar,
    ShowLitBoat,
    ShowLitHouse,
    ShowLitPlane,
    StonesLock,
    StonesGhostDemon,
    StonesStonesBonesAllRedundant,
    StonesGhostLit0,
    StonesGhostLit1,
    StonesGhostLit2,
    StonesGhostLit3,
    StonesGhostLit4,
    StonesGhostLit5,
    StonesGhostLit6,
    StonesGhostLit7,
    StonesGhostExtraBall,
    StonesTowerHunt0,
    StonesTowerHunt1,
    StonesTowerHunt2,
    StonesGhost5M,
    StonesGhost10M,
    StonesGhost15M,
    StonesLoopCombo,
    StonesScreamsExtraBall,
    StonesKickback,
    StonesSkillShot,
    StonesTowerOpen,
    StonesGhostGhostHunter,
    StonesGhostGrimReaper,
    StonesGhostTowerHunt,
    StonesTopMillion,
    StonesTowerMillion,
    StonesDemon5M,
    StonesTower5M,
    StonesTowerExtraBall,
    StonesDemon10M,
    StonesDemon20M,
    StonesTowerHoldBonus,
    StonesTowerDoubleBonus,
    StonesTowerJackpot,
    StonesTowerSuperJackpot,
    StonesMillionPlus,
    StonesVault,
    StonesWell,
    StonesTowerBonus,
    StonesWellMb2,
    StonesWellMb4,
    StonesWellMb6,
    StonesWellMb8,
    StonesWellMb10,
    StonesScreamsToExtraBall,
    StonesScreamsTo5M,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectSound {
    Jingle(Jingle),
    Silent(u8),
}

#[derive(Debug, Clone, Copy)]
pub struct Effect {
    pub sound: EffectSound,
    pub score_main: Bcd,
    pub score_bonus: Bcd,
    pub script: Option<ScriptPosId>,
}

pub type DmAnimFrame = Box<[(DmCoord, bool)]>;

pub mod special_chars {
    pub const HIGH_SCORES: u8 = 0x80;
    pub const BONUS_MULT_L: u8 = 0x90;
    pub const BONUS_MULT_R: u8 = 0x92;
    pub const CUR_PLAYER: u8 = 0x94;
    pub const CUR_BALL: u8 = 0x95;
    pub const TOTAL_PLAYERS: u8 = 0x96;
    pub const NUM_CYCLONES: u8 = 0x98;
    pub const NUM_CYCLONES_TARGET: u8 = 0x9c;
    pub const NUM_CYCLONES_TARGET_L: u8 = 0xa0;
}

fn dm_addr_to_xy(addr: u16, plane: u8) -> DmCoord {
    let mut x = (addr % 0x54) as i16;
    let mut y = (addr / 0x54) as i16;
    if (y & 1) != 0 {
        y += 1;
        x -= 0x54;
    }
    let x = (x * 2) + plane as i16;
    let y = (y / 2) - 1;
    DmCoord { x, y }
}

fn extract_msg(
    exe: &MzExe,
    table: TableId,
    off: u16,
    is_long: bool,
    msgs: &mut EntityMap<MsgId, u16, Box<[u8]>>,
) -> MsgId {
    if let Some((msg, _)) = msgs.get(&off) {
        return msg;
    }
    let mut msg = vec![];
    let mut pos = off;
    let high_score_off = match table {
        TableId::Table1 => 0x16,
        TableId::Table2 => 0x16,
        TableId::Table3 => 0x16,
        TableId::Table4 => 0xa6,
    };
    loop {
        let byte = exe.data_byte(pos);
        if (byte == 0 && !is_long) || (byte == 0xff && is_long) {
            break;
        }
        let mut chr = if is_long {
            match byte {
                1 => b'_',
                b'^' => b'-',
                _ => byte,
            }
        } else {
            match byte {
                0x2a => b'_',
                0x37..=0x40 => byte - 7,
                0x20 | 0x21 | 0x2d | 0x41..=0x5a => byte,
                0x5b => b'?',
                0x5c => b'(',
                0x5d => b')',
                0x5e => b'-',
                _ => panic!("unknown char {byte:02x} at {pos:04x}"),
            }
        };
        if pos >= high_score_off && pos < high_score_off + 0x40 {
            let idx = (pos - high_score_off) / 0x10;
            let cidx = (pos - high_score_off) % 0x10;
            assert!((12..15).contains(&cidx));
            chr = special_chars::HIGH_SCORES + (idx * 3 + (cidx - 12)) as u8;
        }
        match (table, pos) {
            (TableId::Table1, 0x1cf9)
            | (TableId::Table2, 0x1d92)
            | (TableId::Table3, 0x1a7c)
            | (TableId::Table1, 0x1d91)
            | (TableId::Table2, 0x1da9)
            | (TableId::Table3, 0x1a93)
            | (TableId::Table4, 0x2408)
            | (TableId::Table3, 0x1aa8)
            | (TableId::Table4, 0x1716)
            | (TableId::Table1, 0x2249)
            | (TableId::Table2, 0x2058)
            | (TableId::Table3, 0x1d09)
            | (TableId::Table4, 0x26b4)
            | (TableId::Table1, 0x2288)
            | (TableId::Table2, 0x2097)
            | (TableId::Table3, 0x1d48)
            | (TableId::Table4, 0x26f3) => chr = special_chars::CUR_PLAYER,

            (TableId::Table1, 0x2253)
            | (TableId::Table2, 0x2062)
            | (TableId::Table3, 0x1d13)
            | (TableId::Table4, 0x26be)
            | (TableId::Table1, 0x228f)
            | (TableId::Table2, 0x209e)
            | (TableId::Table3, 0x1d4f)
            | (TableId::Table4, 0x26fa) => chr = special_chars::CUR_BALL,

            (TableId::Table1, 0x1d9b)
            | (TableId::Table2, 0x1db3)
            | (TableId::Table3, 0x1ab2)
            | (TableId::Table4, 0x2412) => chr = special_chars::TOTAL_PLAYERS,

            (TableId::Table1, 0x1db9)
            | (TableId::Table2, 0x1dd1)
            | (TableId::Table3, 0x1ad0)
            | (TableId::Table4, 0x2424) => chr = special_chars::BONUS_MULT_L,
            (TableId::Table3, 0x1ad1) | (TableId::Table4, 0x2425) => {
                chr = special_chars::BONUS_MULT_L + 1
            }

            (TableId::Table1, 0x2237)
            | (TableId::Table2, 0x2046)
            | (TableId::Table3, 0x1cf7)
            | (TableId::Table4, 0x26a2) => chr = special_chars::BONUS_MULT_R,
            (TableId::Table1, 0x2238)
            | (TableId::Table2, 0x2047)
            | (TableId::Table3, 0x1cf8)
            | (TableId::Table4, 0x26a3) => chr = special_chars::BONUS_MULT_R + 1,

            (TableId::Table2, 0x19b5) | (TableId::Table4, 0x1e40) => {
                chr = special_chars::NUM_CYCLONES
            }
            (TableId::Table2, 0x19b6) | (TableId::Table4, 0x1e41) => {
                chr = special_chars::NUM_CYCLONES + 1
            }
            (TableId::Table2, 0x19b7) | (TableId::Table4, 0x1e42) => {
                chr = special_chars::NUM_CYCLONES + 2
            }
            (TableId::Table2, 0x1924)
            | (TableId::Table2, 0x1937)
            | (TableId::Table4, 0x1eb0)
            | (TableId::Table4, 0x1ec5) => chr = special_chars::NUM_CYCLONES_TARGET,
            (TableId::Table2, 0x1925)
            | (TableId::Table2, 0x1938)
            | (TableId::Table4, 0x1eb1)
            | (TableId::Table4, 0x1ec6) => chr = special_chars::NUM_CYCLONES_TARGET + 1,
            (TableId::Table2, 0x1926)
            | (TableId::Table2, 0x1939)
            | (TableId::Table4, 0x1eb2)
            | (TableId::Table4, 0x1ec7) => chr = special_chars::NUM_CYCLONES_TARGET + 2,

            _ => (),
        }
        msg.push(chr);
        pos += 1;
    }
    if table == TableId::Table3 && off == 0x128f {
        msg.pop();
        msg.pop();
        msg.push(special_chars::NUM_CYCLONES_TARGET_L);
        msg.push(special_chars::NUM_CYCLONES_TARGET_L + 1);
        msg.push(special_chars::NUM_CYCLONES_TARGET_L + 2);
    }
    msgs.insert(off, msg.into()).0
}

fn extract_dm_anim(
    exe: &MzExe,
    table: TableId,
    off: u16,
    anims: &mut EntityMap<DmAnimId, u16, DmAnim>,
    frames: &mut EntityMap<DmAnimFrameId, u16, DmAnimFrame>,
) -> DmAnimId {
    if let Some((anim, _)) = anims.get(&off) {
        return anim;
    }
    let seg = match table {
        TableId::Table1 => 0x2056,
        TableId::Table2 => 0x1f6f,
        TableId::Table3 => 0x418c,
        TableId::Table4 => 0x1d8f,
    };
    let repeats = exe.word(seg, off - 4);
    let num_frames = exe.word(seg, off - 2) / 4;
    let restart = if repeats == 1 {
        0
    } else {
        (exe.word(seg, off - 6) / 4) as usize
    };
    let real_num_frames = if repeats == 1 {
        num_frames
    } else {
        num_frames + 1
    };
    let frames = (0..real_num_frames)
        .map(|i| {
            let foff = exe.word(seg, off + i * 4);
            let num = exe.word(seg, off + i * 4 + 2);
            let frame = if let Some((frame, _)) = frames.get(&foff) {
                frame
            } else {
                let mut frame = vec![];
                let mut fpos = foff;
                for plane in [0, 1] {
                    let mut dpos = 0xa7;
                    let cnt = exe.word(seg, fpos);
                    fpos += 2;
                    for _ in 0..cnt {
                        let byte = exe.byte(seg, fpos);
                        fpos += 1;
                        dpos += (byte >> 1) as u16;
                        if byte != 0xfe {
                            let xy = dm_addr_to_xy(dpos, plane);
                            frame.push((xy, (byte & 1) != 0));
                        }
                    }
                }
                frames.insert(foff, frame.into()).0
            };
            (frame, num)
        })
        .collect();
    let anim = DmAnim {
        repeats,
        restart,
        num_frames: num_frames as usize,
        frames,
    };
    anims.insert(off, anim).0
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum UopKind {
    End,

    Noop,
    Delay,
    DelayIfMultiplayer,
    Halt,
    Jump,
    JccScoreZero,
    JccNoBonusMult,
    RepeatSetup,
    RepeatLoop,
    FinalScoreSetup,
    FinalScoreLoop,
    ConfirmQuit,

    WaitWhileGameStarting,
    ExtraBall,
    SetupPartyOn,
    SetupShootAgain,
    SetSpecialPlungerEvent,
    IssueBall,

    MultiplyBonus,
    AccBonusCyclones,
    AccBonusModeHit,
    AccBonusModeRamp,
    AccBonus,
    CheckTopScore,
    NextBallIfMatched,
    NextBall,

    Match,
    CheckMatch,
    RecordHighScores,
    GameOver,

    PlaySfx,
    PlayJingle,
    SetMusic,
    SetJingleTimeout,
    WaitJingle,
    WaitJingleTimeout,

    ModeContinue,
    ModeStart,
    ModeStartOrContinue,

    DmBlink,
    DmStopBlink,
    DmState,
    DmClear,
    DmWipeDown,
    DmWipeRight,
    DmWipeDownStriped,
    DmAnim,
    DmPuts(DmFont),
    DmPrintScore(DmFont, bool),
    DmBigScore,
    DmMsgScrollUp,
    DmMsgScrollDown,
    DmLongMsg,
    DmTowerHunt,

    PartyArcadeReady,
    PartySecretDrop,

    SpeedStartTurbo,
    SpeedCheckTurboCont,
    SpeedClearFlagMode,

    ShowSpinWheelEnd,
    ShowBlinkMoneyMania,
    ShowEndMoneyMania,

    StonesTowerEject,
    StonesVaultEject,
    StonesWellEject,
    StonesTiltEject,
    StonesSetFlagMode,
    StonesSetFlagModeRamp,
    StonesSetFlagModeHit,
    StonesClearFlagMode,
    StonesClearFlagModeRamp,
    StonesClearFlagModeHit,
    StonesEndMode,
    StonesEndGrimReaper,
}

fn xlat_uop_kind(table: TableId, ptr: u16) -> UopKind {
    if ptr == 0 {
        return UopKind::End;
    }
    match table {
        TableId::Table1 => match ptr {
            0x0317 => UopKind::AccBonus,
            0x047f => UopKind::RecordHighScores,
            0x0705 => UopKind::RepeatLoop,
            0x071c => UopKind::RepeatSetup,
            0x0735 => UopKind::FinalScoreSetup,
            0x0762 => UopKind::FinalScoreLoop,
            0x07e5 => UopKind::Match,
            0x0a19 => UopKind::CheckMatch,
            0x0a6b => UopKind::NextBallIfMatched,
            0x0ab2 => UopKind::NextBall,
            0x0b43 => UopKind::IssueBall,
            0x0b5b => UopKind::GameOver,
            0x2bf4 => UopKind::SetupPartyOn,
            0x2c14 => UopKind::SetupShootAgain,
            0x2c2f => UopKind::SetSpecialPlungerEvent,
            0x2c43 => UopKind::Noop,
            0x2c52 => UopKind::PartyArcadeReady,
            0x2c66 => UopKind::WaitWhileGameStarting,
            0x2c88 => UopKind::JccScoreZero,
            0x2ccd => UopKind::JccNoBonusMult,
            0x2cf1 => UopKind::Jump,
            0x2cfd => UopKind::MultiplyBonus,
            0x2d3f => UopKind::AccBonusModeRamp,
            0x2d69 => UopKind::AccBonusModeHit,
            0x2d93 => UopKind::AccBonusCyclones,
            0x2f6a => UopKind::PartySecretDrop,
            0x2f7f => UopKind::ExtraBall,
            0x44dd => UopKind::DmAnim,
            0x4519 => UopKind::DmLongMsg,
            0x453a => UopKind::DmBigScore,
            0x454a => UopKind::DmBlink,
            0x45f1 => UopKind::DmPrintScore(DmFont::H11, false),
            0x461a => UopKind::DmPrintScore(DmFont::H13, false),
            0x4643 => UopKind::DmPrintScore(DmFont::H8, false),
            0x466c => UopKind::DmPrintScore(DmFont::H8, true),
            0x469b => UopKind::DmPrintScore(DmFont::H13, true),
            0x46ca => UopKind::DmPrintScore(DmFont::H5, false),
            0x46f3 => UopKind::PlaySfx,
            0x471e => UopKind::SetMusic,
            0x4736 => UopKind::PlayJingle,
            0x4757 => UopKind::ModeContinue,
            0x4780 => UopKind::ModeStart,
            0x47bc => UopKind::ModeStartOrContinue,
            0x4892 => UopKind::DmMsgScrollUp,
            0x48af => UopKind::DmMsgScrollDown,
            0x4b53 => UopKind::DmPuts(DmFont::H13),
            0x4b94 => UopKind::DmPuts(DmFont::H11),
            0x4bd5 => UopKind::DmPuts(DmFont::H8),
            0x4c16 => UopKind::DmPuts(DmFont::H5),
            0x4ceb => UopKind::DmStopBlink,
            0x4cfd => UopKind::DmState,
            0x4d1e => UopKind::SetJingleTimeout,
            0x4d52 => UopKind::Delay,
            0x4d62 => UopKind::DelayIfMultiplayer,
            0x4d82 => UopKind::Halt,
            0x4d96 => UopKind::ConfirmQuit,
            0x4dbd => UopKind::DmClear,
            0x4df0 => UopKind::DmWipeDown,
            0x4e2d => UopKind::DmWipeRight,
            0x4e6d => UopKind::DmWipeDownStriped,
            0x5273 => UopKind::WaitJingle,
            0x527c => UopKind::WaitJingleTimeout,
            0x5456 => UopKind::CheckTopScore,
            _ => panic!("unknown uop {ptr:04x}"),
        },
        TableId::Table2 => match ptr {
            0x022f => UopKind::AccBonus,
            0x0349 => UopKind::RecordHighScores,
            0x05d5 => UopKind::RepeatLoop,
            0x05ec => UopKind::RepeatSetup,
            0x0602 => UopKind::SpeedStartTurbo,
            0x0618 => UopKind::SpeedCheckTurboCont,
            0x063d => UopKind::FinalScoreSetup,
            0x066f => UopKind::FinalScoreLoop,
            0x06f6 => UopKind::SpeedClearFlagMode,
            0x0711 => UopKind::Match,
            0x093a => UopKind::CheckMatch,
            0x0987 => UopKind::NextBallIfMatched,
            0x09cd => UopKind::NextBall,
            0x0a7a => UopKind::IssueBall,
            0x0a92 => UopKind::GameOver,
            0x2535 => UopKind::SetupPartyOn,
            0x2550 => UopKind::SetupShootAgain,
            0x2578 => UopKind::SetSpecialPlungerEvent,
            0x258c => UopKind::Noop,
            0x259b => UopKind::WaitWhileGameStarting,
            0x25bd => UopKind::JccScoreZero,
            0x2602 => UopKind::MultiplyBonus,
            0x2644 => UopKind::JccNoBonusMult,
            0x2668 => UopKind::Jump,
            0x2674 => UopKind::AccBonusModeHit,
            0x269e => UopKind::AccBonusModeRamp,
            0x26c8 => UopKind::AccBonusCyclones,
            0x277e => UopKind::ExtraBall,
            0x3cdc => UopKind::DmAnim,
            0x3d18 => UopKind::DmLongMsg,
            0x3d39 => UopKind::DmBigScore,
            0x3d49 => UopKind::DmBlink,
            0x3df0 => UopKind::DmPrintScore(DmFont::H11, false),
            0x3e19 => UopKind::DmPrintScore(DmFont::H13, false),
            0x3e42 => UopKind::DmPrintScore(DmFont::H8, false),
            0x3e6b => UopKind::DmPrintScore(DmFont::H8, true),
            0x3e9a => UopKind::DmPrintScore(DmFont::H13, true),
            0x3ec9 => UopKind::DmPrintScore(DmFont::H5, false),
            0x3ef2 => UopKind::PlaySfx,
            0x3f1d => UopKind::SetMusic,
            0x3f35 => UopKind::PlayJingle,
            0x3f56 => UopKind::ModeContinue,
            0x3f7f => UopKind::ModeStart,
            0x3fbb => UopKind::ModeStartOrContinue,
            0x4091 => UopKind::DmMsgScrollUp,
            0x40ae => UopKind::DmMsgScrollDown,
            0x4352 => UopKind::DmPuts(DmFont::H13),
            0x4393 => UopKind::DmPuts(DmFont::H11),
            0x43d4 => UopKind::DmPuts(DmFont::H8),
            0x4415 => UopKind::DmPuts(DmFont::H5),
            0x44ea => UopKind::DmStopBlink,
            0x44fc => UopKind::DmState,
            0x451d => UopKind::SetJingleTimeout,
            0x4551 => UopKind::Delay,
            0x4561 => UopKind::DelayIfMultiplayer,
            0x4581 => UopKind::Halt,
            0x4595 => UopKind::ConfirmQuit,
            0x45bc => UopKind::DmClear,
            0x45ef => UopKind::DmWipeDown,
            0x462c => UopKind::DmWipeRight,
            0x466c => UopKind::DmWipeDownStriped,
            0x4a72 => UopKind::WaitJingle,
            0x4a7b => UopKind::WaitJingleTimeout,
            0x4c50 => UopKind::CheckTopScore,
            _ => panic!("unknown uop {ptr:04x}"),
        },
        TableId::Table3 => match ptr {
            0x01f8 => UopKind::AccBonus,
            0x0312 => UopKind::RecordHighScores,
            0x0598 => UopKind::RepeatLoop,
            0x05af => UopKind::RepeatSetup,
            0x05c8 => UopKind::FinalScoreSetup,
            0x05f5 => UopKind::FinalScoreLoop,
            0x0678 => UopKind::ShowSpinWheelEnd,
            0x068f => UopKind::ShowEndMoneyMania,
            0x06b6 => UopKind::ShowBlinkMoneyMania,
            0x06d0 => UopKind::Match,
            0x08c5 => UopKind::CheckMatch,
            0x0912 => UopKind::NextBallIfMatched,
            0x0959 => UopKind::NextBall,
            0x09fd => UopKind::IssueBall,
            0x0a15 => UopKind::GameOver,
            0x1fe1 => UopKind::SetupPartyOn,
            0x1fff => UopKind::SetupShootAgain,
            0x201a => UopKind::SetSpecialPlungerEvent,
            0x202e => UopKind::Noop,
            0x203d => UopKind::WaitWhileGameStarting,
            0x205f => UopKind::JccScoreZero,
            0x20a4 => UopKind::MultiplyBonus,
            0x20fc => UopKind::JccNoBonusMult,
            0x2120 => UopKind::AccBonusModeHit,
            0x214a => UopKind::Jump,
            0x215b => UopKind::AccBonusCyclones,
            0x2211 => UopKind::ExtraBall,
            0x376b => UopKind::DmAnim,
            0x37a7 => UopKind::DmLongMsg,
            0x37c8 => UopKind::DmBigScore,
            0x37d8 => UopKind::DmBlink,
            0x387f => UopKind::DmPrintScore(DmFont::H11, false),
            0x38a8 => UopKind::DmPrintScore(DmFont::H13, false),
            0x38d1 => UopKind::DmPrintScore(DmFont::H8, false),
            0x38fa => UopKind::DmPrintScore(DmFont::H8, true),
            0x3929 => UopKind::DmPrintScore(DmFont::H13, true),
            0x3958 => UopKind::DmPrintScore(DmFont::H5, false),
            0x3981 => UopKind::PlaySfx,
            0x39ac => UopKind::SetMusic,
            0x39c4 => UopKind::PlayJingle,
            0x39e5 => UopKind::ModeContinue,
            0x3a0e => UopKind::ModeStart,
            0x3a4a => UopKind::ModeStartOrContinue,
            0x3b20 => UopKind::DmMsgScrollUp,
            0x3b3d => UopKind::DmMsgScrollDown,
            0x3de1 => UopKind::DmPuts(DmFont::H13),
            0x3e22 => UopKind::DmPuts(DmFont::H11),
            0x3e63 => UopKind::DmPuts(DmFont::H8),
            0x3ea4 => UopKind::DmPuts(DmFont::H5),
            0x3f79 => UopKind::DmStopBlink,
            0x3f8b => UopKind::DmState,
            0x3fac => UopKind::SetJingleTimeout,
            0x3fe0 => UopKind::Delay,
            0x3ff0 => UopKind::DelayIfMultiplayer,
            0x4010 => UopKind::Halt,
            0x4024 => UopKind::ConfirmQuit,
            0x404b => UopKind::DmClear,
            0x407e => UopKind::DmWipeDown,
            0x40bb => UopKind::DmWipeRight,
            0x40fb => UopKind::DmWipeDownStriped,
            0x4501 => UopKind::WaitJingle,
            0x450a => UopKind::WaitJingleTimeout,
            0x46df => UopKind::CheckTopScore,
            _ => panic!("unknown uop {ptr:04x}"),
        },
        TableId::Table4 => match ptr {
            0x02d0 => UopKind::AccBonus,
            0x03ea => UopKind::RecordHighScores,
            0x0676 => UopKind::RepeatLoop,
            0x068d => UopKind::RepeatSetup,
            0x06a6 => UopKind::FinalScoreSetup,
            0x06d8 => UopKind::FinalScoreLoop,
            0x075f => UopKind::StonesClearFlagMode,
            0x077a => UopKind::StonesSetFlagMode,
            0x078f => UopKind::StonesSetFlagModeRamp,
            0x07a3 => UopKind::StonesClearFlagModeRamp,
            0x07b7 => UopKind::StonesSetFlagModeHit,
            0x07cb => UopKind::StonesClearFlagModeHit,
            0x07df => UopKind::Match,
            0x0a04 => UopKind::CheckMatch,
            0x0a51 => UopKind::NextBallIfMatched,
            0x0a97 => UopKind::NextBall,
            0x0b44 => UopKind::IssueBall,
            0x0b5c => UopKind::GameOver,
            0x1fd6 => UopKind::StonesEndMode,
            0x2106 => UopKind::StonesTowerEject,
            0x233b => UopKind::StonesWellEject,
            0x28d8 => UopKind::StonesEndGrimReaper,
            0x28f3 => UopKind::StonesVaultEject,
            0x29b7 => UopKind::StonesTiltEject,
            0x3359 => UopKind::WaitWhileGameStarting,
            0x337b => UopKind::JccScoreZero,
            0x33bf => UopKind::JccNoBonusMult,
            0x33e3 => UopKind::Jump,
            0x33ee => UopKind::AccBonusModeHit,
            0x3418 => UopKind::AccBonusCyclones,
            0x355c => UopKind::SetupPartyOn,
            0x3577 => UopKind::SetupShootAgain,
            0x3592 => UopKind::SetSpecialPlungerEvent,
            0x35a6 => UopKind::Noop,
            0x35ff => UopKind::MultiplyBonus,
            0x3657 => UopKind::AccBonusModeRamp,
            0x3681 => UopKind::DmTowerHunt,
            0x3732 => UopKind::ExtraBall,
            0x4c89 => UopKind::DmAnim,
            0x4cc5 => UopKind::DmLongMsg,
            0x4cf6 => UopKind::DmBlink,
            0x4d9d => UopKind::DmPrintScore(DmFont::H11, false),
            0x4dc6 => UopKind::DmPrintScore(DmFont::H13, false),
            0x4def => UopKind::DmPrintScore(DmFont::H8, false),
            0x4e18 => UopKind::DmPrintScore(DmFont::H8, true),
            0x4e47 => UopKind::DmPrintScore(DmFont::H13, true),
            0x4e76 => UopKind::DmPrintScore(DmFont::H5, false),
            0x4e9f => UopKind::PlaySfx,
            0x4eca => UopKind::SetMusic,
            0x4ee2 => UopKind::PlayJingle,
            0x4f03 => UopKind::ModeContinue,
            0x4f2c => UopKind::ModeStart,
            0x4f68 => UopKind::ModeStartOrContinue,
            0x503e => UopKind::DmMsgScrollUp,
            0x505b => UopKind::DmMsgScrollDown,
            0x52ff => UopKind::DmPuts(DmFont::H13),
            0x5340 => UopKind::DmPuts(DmFont::H11),
            0x5381 => UopKind::DmPuts(DmFont::H8),
            0x53c2 => UopKind::DmPuts(DmFont::H5),
            0x5497 => UopKind::DmStopBlink,
            0x54a9 => UopKind::DmState,
            0x54ca => UopKind::SetJingleTimeout,
            0x54fe => UopKind::Delay,
            0x550e => UopKind::Delay,
            0x552e => UopKind::Halt,
            0x5542 => UopKind::ConfirmQuit,
            0x5569 => UopKind::DmClear,
            0x559c => UopKind::DmWipeDown,
            0x55d9 => UopKind::DmWipeRight,
            0x5619 => UopKind::DmWipeDownStriped,
            0x5a1f => UopKind::WaitJingle,
            0x5a28 => UopKind::WaitJingleTimeout,
            0x5bfd => UopKind::CheckTopScore,
            _ => panic!("unknown uop {ptr:04x}"),
        },
    }
}

fn xlat_score(exe: &MzExe, table: TableId, ptr: u16) -> ScriptScore {
    match table {
        TableId::Table1 => match ptr {
            0x16 => ScriptScore::HighScore(0),
            0x26 => ScriptScore::HighScore(1),
            0x36 => ScriptScore::HighScore(2),
            0x46 => ScriptScore::HighScore(3),
            0xb0 => ScriptScore::NumCyclone,
            0xbc => ScriptScore::CycloneBonus,
            0xdc => ScriptScore::PartyTunnelSkillShot,
            0xe8 => ScriptScore::PartyCycloneSkillShot,
            0xf4 => ScriptScore::ModeHit,
            0x100 => ScriptScore::ModeRamp,
            0x10c => ScriptScore::Jackpot,
            0x130 | 0x13c | 0x148 | 0x154 | 0x1d8 | 0x1e4 | 0x1f0 => {
                ScriptScore::Const(exe.data_bcd(ptr))
            }
            0x160 => ScriptScore::CycloneIncr,
            0x3399 => ScriptScore::Bonus,
            _ => panic!("unknown score {ptr:04x}"),
        },
        TableId::Table2 => match ptr {
            0x16 => ScriptScore::HighScore(0),
            0x26 => ScriptScore::HighScore(1),
            0x36 => ScriptScore::HighScore(2),
            0x46 => ScriptScore::HighScore(3),
            0x58 => ScriptScore::NumCyclone,
            0x64 => ScriptScore::CycloneBonus,
            0x96a => ScriptScore::Jackpot,
            0x99a => ScriptScore::CycloneIncr,
            0x1168 => ScriptScore::Const(exe.data_bcd(ptr)),
            0x1373 => ScriptScore::ModeHit,
            0x1424 => ScriptScore::ModeRamp,
            0x3361 => ScriptScore::Bonus,
            _ => panic!("unknown score {ptr:04x}"),
        },
        TableId::Table3 => match ptr {
            0x16 => ScriptScore::HighScore(0),
            0x26 => ScriptScore::HighScore(1),
            0x36 => ScriptScore::HighScore(2),
            0x46 => ScriptScore::HighScore(3),
            0x125 => ScriptScore::ShowRaisingMillions,
            0x13d => ScriptScore::ShowSpinWheel,
            0x1df => ScriptScore::CycloneIncr,
            0x3a5 => ScriptScore::NumCyclone,
            0x3b1 => ScriptScore::CycloneBonus,
            0x617 => ScriptScore::Jackpot,
            0x6cb => ScriptScore::ShowCashpot,
            0x6e7 => ScriptScore::ShowCashpotX5,
            0x15fc => ScriptScore::ModeHit, // and also mode ramp
            0x2c2b => ScriptScore::Bonus,
            _ => panic!("unknown score {ptr:04x}"),
        },
        TableId::Table4 => match ptr {
            0xa6 => ScriptScore::HighScore(0),
            0xb6 => ScriptScore::HighScore(1),
            0xc6 => ScriptScore::HighScore(2),
            0xd6 => ScriptScore::HighScore(3),
            0x1de => ScriptScore::Const(exe.data_bcd(ptr)),
            0x1ee => ScriptScore::NumCyclone,
            0x1fa => ScriptScore::CycloneBonus,
            0x212 => ScriptScore::StonesSkillShot,
            0x22f => ScriptScore::StonesMillionPlus,
            0x275 => ScriptScore::Jackpot,
            0x299 => ScriptScore::StonesTowerBonus,
            0x2bd => ScriptScore::StonesVault,
            0x2e1 => ScriptScore::StonesWell,
            0x8a8 => ScriptScore::CycloneIncr,
            0x1d3b => ScriptScore::ModeHit,
            0x1e30 => ScriptScore::ModeRamp,
            0x38b1 => ScriptScore::Bonus,
            _ => panic!("unknown score {ptr:04x}"),
        },
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn extract_scripts(
    exe: &MzExe,
    table: TableId,
) -> (
    EntityVec<ScriptPosId, Uop>,
    HashMap<u16, ScriptPosId>,
    EntityVec<MsgId, Box<[u8]>>,
    EntityVec<DmAnimId, DmAnim>,
    EntityVec<DmAnimFrameId, DmAnimFrame>,
) {
    let ranges: &[(u16, u16)] = match table {
        TableId::Table1 => &[
            (0x13cd, 0x177b),
            (0x1790, 0x1812),
            (0x1822, 0x1c02),
            (0x439c, 0x4432),
        ],
        TableId::Table2 => &[
            (0x1102, 0x1168),
            (0x117a, 0x1312),
            (0x137f, 0x13eb),
            (0x1461, 0x1569),
            (0x157e, 0x1608),
            (0x1618, 0x1924),
            (0x1b65, 0x1c03),
            (0x1d1b, 0x1d2d),
            (0x4433, 0x44c9),
        ],
        TableId::Table3 => &[
            (0xe30, 0xede),
            (0xf16, 0xf20),
            (0xf66, 0xf70),
            (0xfb4, 0xfbe),
            (0x1001, 0x100b),
            (0x104c, 0x1056),
            (0x10ad, 0x128f),
            (0x12a3, 0x12d3),
            (0x12e7, 0x1397),
            (0x1738, 0x17ce),
            (0x1629, 0x1723),
            (0x17de, 0x1a1a),
            (0x3c58, 0x3d0c),
        ],
        TableId::Table4 => &[
            (0x1771, 0x1cc5),
            (0x1d47, 0x1dfb),
            (0x1e4c, 0x1eb0),
            (0x1efe, 0x2016),
            (0x203e, 0x20de),
            (0x20ee, 0x233e),
            (0x2368, 0x2382),
            (0x496c, 0x4a20),
        ],
    };
    let mut msgs = EntityMap::new();
    let mut anims = EntityMap::new();
    let mut anim_frames = EntityMap::new();
    let mut uops = EntityVec::new();
    let mut uops_by_addr = HashMap::new();
    let mut relocs = vec![];
    for &(mut pos, end) in ranges {
        let mut was_end = true;
        while pos != end {
            let cur = uops.next_id();
            uops_by_addr.insert(pos, cur);
            let kind = exe.data_word(pos);
            let kind = xlat_uop_kind(table, kind);
            pos += 2;
            was_end = false;
            let uop = match kind {
                UopKind::End => {
                    was_end = true;
                    Uop::End
                }
                UopKind::Noop => {
                    assert_eq!(exe.data_word(pos), 1);
                    pos += 2;
                    Uop::Noop
                }
                UopKind::Delay => {
                    let time = exe.data_word(pos);
                    pos += 2;
                    Uop::Delay(time)
                }
                UopKind::DelayIfMultiplayer => {
                    let time = exe.data_word(pos);
                    pos += 2;
                    Uop::DelayIfMultiplayer(time)
                }
                UopKind::Jump => {
                    let target = exe.data_word(pos);
                    pos += 2;
                    relocs.push((cur, target));
                    Uop::Jump(cur)
                }
                UopKind::JccScoreZero => {
                    let score = exe.data_word(pos);
                    let score = xlat_score(exe, table, score);
                    let target = exe.data_word(pos + 2);
                    pos += 4;
                    relocs.push((cur, target));
                    Uop::JccScoreZero(score, cur)
                }
                UopKind::JccNoBonusMult => {
                    let target = exe.data_word(pos);
                    pos += 2;
                    relocs.push((cur, target));
                    Uop::JccNoBonusMult(cur)
                }
                UopKind::RepeatSetup => {
                    let num = exe.data_word(pos);
                    assert_eq!(exe.data_word(pos + 2), 0);
                    pos += 4;
                    Uop::RepeatSetup(num)
                }
                UopKind::RepeatLoop => {
                    let num = exe.data_word(pos);
                    let target = exe.data_word(pos + 2);
                    pos += 4;
                    relocs.push((cur, target));
                    Uop::RepeatLoop(num, cur)
                }
                UopKind::FinalScoreSetup => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::FinalScoreSetup
                }
                UopKind::FinalScoreLoop => {
                    let target = exe.data_word(pos);
                    pos += 2;
                    relocs.push((cur, target));
                    Uop::FinalScoreLoop(cur)
                }

                UopKind::DmBlink => {
                    let arg = exe.data_word(pos);
                    pos += 2;
                    Uop::DmBlink(arg)
                }
                UopKind::DmStopBlink => {
                    pos += 2;
                    Uop::DmStopBlink
                }
                UopKind::DmState => {
                    let state = exe.data_word(pos);
                    pos += 2;
                    assert!(state < 2);
                    Uop::DmState(state != 0)
                }
                UopKind::DmClear => Uop::DmClear,
                UopKind::DmWipeDown => Uop::DmWipeDown,
                UopKind::DmWipeRight => Uop::DmWipeRight,
                UopKind::DmWipeDownStriped => Uop::DmWipeDownStriped,
                UopKind::DmAnim => {
                    let anim = exe.data_word(pos);
                    let anim = extract_dm_anim(exe, table, anim, &mut anims, &mut anim_frames);
                    pos += 2;
                    Uop::DmAnim(anim)
                }
                UopKind::DmPuts(font) => {
                    let mut msg = exe.data_word(pos);
                    let mut dpos = exe.data_word(pos + 2);
                    if dpos == 0x14e {
                        assert_eq!(exe.data_byte(msg), b' ');
                        assert_eq!(exe.data_byte(msg + 1), b' ');
                        msg += 2;
                        dpos += 8;
                    }
                    let msg = extract_msg(exe, table, msg, false, &mut msgs);
                    pos += 4;
                    Uop::DmPuts(font, dm_addr_to_xy(dpos, 0), msg)
                }
                UopKind::DmPrintScore(font, is_centered) => {
                    let score = exe.data_word(pos);
                    let score = xlat_score(exe, table, score);
                    let dpos = exe.data_word(pos + 2);
                    pos += 4;
                    Uop::DmPrintScore(font, is_centered, dm_addr_to_xy(dpos, 0), score)
                }
                UopKind::DmBigScore => {
                    let score = exe.data_word(pos);
                    let score = xlat_score(exe, table, score);
                    pos += 2;
                    Uop::DmPrintScore(DmFont::H13, false, DmCoord { x: -16, y: 1 }, score)
                }
                UopKind::DmMsgScrollUp => {
                    let msg = exe.data_word(pos);
                    let msg = extract_msg(exe, table, msg, false, &mut msgs);
                    let end = exe.data_word_s(pos + 2);
                    pos += 4;
                    Uop::DmMsgScrollUp(msg, end)
                }
                UopKind::DmMsgScrollDown => {
                    let msg = exe.data_word(pos);
                    let msg = extract_msg(exe, table, msg, false, &mut msgs);
                    let end = exe.data_word_s(pos + 2);
                    pos += 4;
                    Uop::DmMsgScrollDown(msg, end)
                }
                UopKind::DmLongMsg => {
                    let msg = exe.data_word(pos);
                    let msg = extract_msg(exe, table, msg, true, &mut msgs);
                    pos += 2;
                    Uop::DmLongMsg(msg)
                }
                UopKind::DmTowerHunt => {
                    let arg = exe.data_word(pos);
                    pos += 2;
                    Uop::DmTowerHunt(arg)
                }
                UopKind::PlaySfx => {
                    let ptr = exe.data_word(pos);
                    let sfx = extract_sfx(exe, ptr);
                    let mut volume = exe.data_word(pos + 2);
                    if volume == 0 {
                        volume = 0x40;
                    }
                    pos += 4;
                    Uop::PlaySfx(sfx, volume.try_into().unwrap())
                }
                UopKind::PlayJingle => {
                    let ptr = exe.data_word(pos);
                    let jingle = extract_jingle(exe, ptr);
                    pos += 2;
                    Uop::PlayJingle(jingle)
                }
                UopKind::SetMusic => {
                    let music = exe.data_word(pos);
                    pos += 2;
                    Uop::SetMusic(music.try_into().unwrap())
                }
                UopKind::SetJingleTimeout => {
                    let timeout = exe.data_word(pos);
                    pos += 2;
                    Uop::SetJingleTimeout(timeout)
                }
                UopKind::WaitJingle => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::WaitJingle
                }
                UopKind::WaitJingleTimeout => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::WaitJingleTimeout
                }
                UopKind::ModeContinue => {
                    let dig1 = exe.data_word(pos);
                    let dig0 = exe.data_word(pos + 2);
                    assert!(dig1 < 10);
                    assert!(dig0 < 10);
                    let time = (dig1 * 10 + dig0) as u8;
                    let score = exe.data_word(pos + 4);
                    let score = xlat_score(exe, table, score);
                    pos += 6;
                    Uop::ModeContinue(time, score)
                }
                UopKind::ModeStart => {
                    let dig1 = exe.data_word(pos);
                    let dig0 = exe.data_word(pos + 2);
                    assert!(dig1 < 10);
                    assert!(dig0 < 10);
                    let time = (dig1 * 10 + dig0) as u8;
                    let score = exe.data_word(pos + 4);
                    let score = xlat_score(exe, table, score);
                    pos += 6;
                    Uop::ModeStart(time, score)
                }
                UopKind::ModeStartOrContinue => {
                    let dig1 = exe.data_word(pos);
                    let dig0 = exe.data_word(pos + 2);
                    assert!(dig1 < 10);
                    assert!(dig0 < 10);
                    let time = (dig1 * 10 + dig0) as u8;
                    let score = exe.data_word(pos + 4);
                    let score = xlat_score(exe, table, score);
                    pos += 6;
                    Uop::ModeStartOrContinue(time, score)
                }
                UopKind::Halt => {
                    assert_eq!(exe.data_word(pos), 1);
                    pos += 2;
                    Uop::Halt
                }
                UopKind::ConfirmQuit => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::ConfirmQuit
                }
                UopKind::WaitWhileGameStarting => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::WaitWhileGameStarting
                }
                UopKind::ExtraBall => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::ExtraBall
                }
                UopKind::SetupPartyOn => {
                    assert!(exe.data_word(pos) <= 1);
                    pos += 2;
                    Uop::SetupPartyOn
                }
                UopKind::SetupShootAgain => {
                    assert!(exe.data_word(pos) == 1);
                    pos += 2;
                    Uop::SetupShootAgain
                }
                UopKind::SetSpecialPlungerEvent => {
                    assert!(exe.data_word(pos) <= 1);
                    pos += 2;
                    Uop::SetSpecialPlungerEvent
                }
                UopKind::IssueBall => Uop::IssueBall,

                UopKind::MultiplyBonus => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::MultiplyBonus
                }
                UopKind::AccBonusCyclones => Uop::AccBonusCyclones,
                UopKind::AccBonusModeHit => Uop::AccBonusModeHit,
                UopKind::AccBonusModeRamp => Uop::AccBonusModeRamp,
                UopKind::AccBonus => Uop::AccBonus,
                UopKind::CheckTopScore => Uop::CheckTopScore,
                UopKind::NextBallIfMatched => Uop::NextBallIfMatched,
                UopKind::NextBall => Uop::NextBall,

                UopKind::Match => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::Match
                }
                UopKind::CheckMatch => Uop::CheckMatch,
                UopKind::RecordHighScores => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::RecordHighScores
                }
                UopKind::GameOver => Uop::GameOver,

                UopKind::PartyArcadeReady => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::PartyArcadeReady
                }
                UopKind::PartySecretDrop => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::PartySecretDrop
                }

                UopKind::SpeedStartTurbo => Uop::SpeedStartTurbo,
                UopKind::SpeedCheckTurboCont => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::SpeedCheckTurboCont
                }
                UopKind::SpeedClearFlagMode => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::SpeedClearFlagMode
                }

                UopKind::ShowSpinWheelEnd => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::ShowSpinWheelEnd
                }
                UopKind::ShowBlinkMoneyMania => {
                    assert_eq!(exe.data_word(pos), 32);
                    pos += 2;
                    Uop::ShowBlinkMoneyMania
                }
                UopKind::ShowEndMoneyMania => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::ShowEndMoneyMania
                }

                UopKind::StonesTowerEject => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesTowerEject
                }
                UopKind::StonesVaultEject => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesVaultEject
                }
                UopKind::StonesWellEject => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesWellEject
                }
                UopKind::StonesTiltEject => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesTiltEject
                }
                UopKind::StonesSetFlagMode => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesSetFlagMode
                }
                UopKind::StonesSetFlagModeRamp => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesSetFlagModeRamp
                }
                UopKind::StonesSetFlagModeHit => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesSetFlagModeHit
                }
                UopKind::StonesClearFlagMode => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesClearFlagMode
                }
                UopKind::StonesClearFlagModeRamp => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesClearFlagModeRamp
                }
                UopKind::StonesClearFlagModeHit => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesClearFlagModeHit
                }
                UopKind::StonesEndMode => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesEndMode
                }
                UopKind::StonesEndGrimReaper => {
                    assert_eq!(exe.data_word(pos), 0);
                    pos += 2;
                    Uop::StonesEndGrimReaper
                }
            };
            uops.push(uop);
        }
        assert!(was_end);
    }
    for (pos, tgt) in relocs {
        let tgt = uops_by_addr[&tgt];
        match uops[pos] {
            Uop::Jump(ref mut fix) => *fix = tgt,
            Uop::JccScoreZero(_, ref mut fix) => *fix = tgt,
            Uop::JccNoBonusMult(ref mut fix) => *fix = tgt,
            Uop::RepeatLoop(_, ref mut fix) => *fix = tgt,
            Uop::FinalScoreLoop(ref mut fix) => *fix = tgt,
            _ => panic!("relocation to weird uop"),
        }
    }
    let msgs = msgs.into_values().collect();
    let anims = anims.into_values().collect();
    let anim_frames = anim_frames.into_values().collect();
    (uops, uops_by_addr, msgs, anims, anim_frames)
}

pub fn extract_script_binds(
    table: TableId,
    uops_by_addr: &HashMap<u16, ScriptPosId>,
) -> EnumMap<ScriptBind, Option<ScriptPosId>> {
    let offsets = match table {
        TableId::Table1 => enum_map! {
            ScriptBind::TopScoreInterball => Some(0x13cd),
            ScriptBind::TopScoreIngame => Some(0x13f7),
            ScriptBind::PartyOn => Some(0x1477),
            ScriptBind::Enter => Some(0x148d),
            ScriptBind::Init => Some(0x14b9),
            ScriptBind::PartyJackpot => Some(0x1647),
            ScriptBind::PartyJackpotModeHit => Some(0x1651),
            ScriptBind::PartyJackpotModeRamp => Some(0x166f),
            ScriptBind::ShootAgain => Some(0x1790),
            ScriptBind::Match => Some(0x17a8),
            ScriptBind::CheckMatch => Some(0x17b6),
            ScriptBind::PostMatch => Some(0x17b8),
            ScriptBind::GameOver => Some(0x17c4),
            ScriptBind::GameIdle => Some(0x18ea),
            ScriptBind::Attract => Some(0x19d4),
            ScriptBind::Main => Some(0x1acc),
            ScriptBind::GameStart => Some(0x1adc),
            ScriptBind::GameStartPlayers => Some(0x1aea),
            ScriptBind::Tilt => Some(0x1bf0),
            ScriptBind::ConfirmQuit => Some(0x4414),
            _ => None,
        },
        TableId::Table2 => enum_map! {
            ScriptBind::TopScoreInterball => Some(0x1102),
            ScriptBind::TopScoreIngame => Some(0x1138),
            ScriptBind::PartyOn => Some(0x128e),
            ScriptBind::Enter => Some(0x12a4),
            ScriptBind::Init => Some(0x12b0),
            ScriptBind::SpeedModeHit => Some(0x12bc),
            ScriptBind::SpeedModeRampContinue => Some(0x137f),
            ScriptBind::SpeedModeRamp => Some(0x138f),
            ScriptBind::ShootAgain => Some(0x157e),
            ScriptBind::Match => Some(0x1596),
            ScriptBind::CheckMatch => Some(0x15a8),
            ScriptBind::PostMatch => Some(0x15aa),
            ScriptBind::GameOver => Some(0x15ba),
            ScriptBind::GameIdle => Some(0x1618),
            ScriptBind::Attract => Some(0x1702),
            ScriptBind::Main => Some(0x17fa),
            ScriptBind::GameStart => Some(0x180a),
            ScriptBind::GameStartPlayers => Some(0x1818),
            ScriptBind::Tilt => Some(0x1d1b),
            ScriptBind::ConfirmQuit => Some(0x44ab),
            _ => None,
        },
        TableId::Table3 => enum_map! {
            ScriptBind::TopScoreInterball => Some(0x0e30),
            ScriptBind::TopScoreIngame => Some(0x0e5e),
            ScriptBind::PartyOn => Some(0x0e86),
            ScriptBind::Enter => Some(0x0e9e),
            ScriptBind::Init => Some(0x0eac),
            ScriptBind::ShowHintLoopRight => Some(0x10ef),
            ScriptBind::ShowHintLoopLeft => Some(0x10fd),
            ScriptBind::ShowMbX2 => Some(0x110b),
            ScriptBind::ShowMbX3 => Some(0x111d),
            ScriptBind::ShowMbX4 => Some(0x112f),
            ScriptBind::ShowMbX6 => Some(0x1141),
            ScriptBind::ShowMbX8 => Some(0x1153),
            ScriptBind::ShowMbX10 => Some(0x1165),
            ScriptBind::ShowSpinWheelBlink => Some(0x12e7),
            ScriptBind::ShowSpinWheelClear => Some(0x12f9),
            ScriptBind::ShowSpinWheelClearHalt => Some(0x12ff),
            ScriptBind::ShowSpinWheelScore => Some(0x1309),
            ScriptBind::ShootAgain => Some(0x1738),
            ScriptBind::Match => Some(0x1752),
            ScriptBind::CheckMatch => Some(0x1766),
            ScriptBind::PostMatch => Some(0x1768),
            ScriptBind::GameOver => Some(0x177a),
            ScriptBind::GameIdle => Some(0x17de),
            ScriptBind::Attract => Some(0x18de),
            ScriptBind::Main => Some(0x19dc),
            ScriptBind::GameStart => Some(0x19ee),
            ScriptBind::GameStartPlayers => Some(0x19fe),
            ScriptBind::Tilt => Some(0x1a0a),
            ScriptBind::ConfirmQuit => Some(0x3cec),
            _ => None,
        },
        TableId::Table4 => enum_map! {
            ScriptBind::TopScoreInterball => Some(0x1771),
            ScriptBind::TopScoreIngame => Some(0x17a9),
            ScriptBind::PartyOn => Some(0x1bf7),
            ScriptBind::Enter => Some(0x1c0f),
            ScriptBind::Init => Some(0x1c1d),
            ScriptBind::StonesModeHitContinue => Some(0x1ca3),
            ScriptBind::StonesModeRampContinue => Some(0x1dd9),
            ScriptBind::ShootAgain => Some(0x203e),
            ScriptBind::Match => Some(0x205c),
            ScriptBind::CheckMatch => Some(0x2074),
            ScriptBind::PostMatch => Some(0x2076),
            ScriptBind::GameOver => Some(0x208a),
            ScriptBind::GameIdle => Some(0x20ee),
            ScriptBind::Attract => Some(0x21ee),
            ScriptBind::Main => Some(0x22fc),
            ScriptBind::GameStart => Some(0x230e),
            ScriptBind::GameStartPlayers => Some(0x231e),
            ScriptBind::Tilt => Some(0x2368),
            ScriptBind::ConfirmQuit => Some(0x4a00),
            _ => None,
        },
    };
    enum_map! {
        key => offsets[key].map(|off| uops_by_addr[&off])
    }
}

pub fn extract_cheats(table: TableId, uops_by_addr: &HashMap<u16, ScriptPosId>) -> Vec<Cheat> {
    let (
        tech,
        cheat,
        robban,
        stein,
        greet,
        daniel,
        gabriel,
        johan,
        tsp,
        earthquake,
        snail,
        extra_balls,
        fair_play,
    ) = match table {
        TableId::Table1 => (
            0x439c, 0x43a4, 0x43ac, 0x43b4, 0x43bc, 0x43c4, 0x43cc, 0x43d4, 0x43dc, 0x43e4, 0x43ec,
            0x43f4, 0x43fc,
        ),
        TableId::Table2 => (
            0x4433, 0x443b, 0x4443, 0x444b, 0x4453, 0x445b, 0x4463, 0x446b, 0x4473, 0x447b, 0x4483,
            0x448b, 0x4493,
        ),
        TableId::Table3 => (
            0x3c58, 0x3c62, 0x3c6c, 0x3c76, 0x3c80, 0x3c8a, 0x3c94, 0x3c9e, 0x3ca8, 0x3cb2, 0x3cbc,
            0x3cc6, 0x3cd0,
        ),
        TableId::Table4 => (
            0x496c, 0x4976, 0x4980, 0x498a, 0x4994, 0x499e, 0x49a8, 0x49b2, 0x49bc, 0x49c6, 0x49d0,
            0x49da, 0x49e4,
        ),
    };
    [
        (&b"JOHAN"[..], johan, CheatEffect::None),
        (b"TECH", tech, CheatEffect::None),
        (b"TSP", tsp, CheatEffect::None),
        (b"DANIEL", daniel, CheatEffect::None),
        (b"GABRIEL", gabriel, CheatEffect::None),
        (b"CHEAT", cheat, CheatEffect::None),
        (b"EARTHQUAKE", earthquake, CheatEffect::Tilt),
        (b"EXTRA BALLS", extra_balls, CheatEffect::Balls),
        (b"SNAIL", snail, CheatEffect::Slowdown),
        (b"FAIR PLAY", fair_play, CheatEffect::Reset),
        (b"ROBBAN", robban, CheatEffect::None),
        (b"STEIN", stein, CheatEffect::None),
        (b"GREET", greet, CheatEffect::None),
    ]
    .into_iter()
    .map(|(keys, script, effect)| Cheat {
        keys: keys.into(),
        script: uops_by_addr[&script],
        effect,
    })
    .collect()
}

fn extract_effect(exe: &MzExe, mut off: u16, uops_by_addr: &HashMap<u16, ScriptPosId>) -> Effect {
    let jingle = exe.data_word(off);
    off += 2;
    let sound = if jingle == 0 {
        let priority = exe.data_byte(off);
        off += 1;
        EffectSound::Silent(priority)
    } else {
        EffectSound::Jingle(extract_jingle(exe, jingle))
    };
    let script = exe.data_word(off + 24);
    let script = if script == 0 {
        None
    } else {
        Some(uops_by_addr[&script])
    };
    Effect {
        sound,
        score_main: exe.data_bcd(off),
        score_bonus: exe.data_bcd(off + 12),
        script,
    }
}

pub fn extract_effects(
    exe: &MzExe,
    table: TableId,
    uops_by_addr: &HashMap<u16, ScriptPosId>,
) -> EnumMap<EffectBind, Option<Effect>> {
    let mut res = enum_map! { _ => None };
    let effects: &[_] = match table {
        TableId::Table1 => &[
            (0x640, EffectBind::PartyArcadeSideExtraBall),
            (0x65d, EffectBind::PartyArcade5M),
            (0x679, EffectBind::PartyArcade1M),
            (0x695, EffectBind::PartyArcade500k),
            (0x6b1, EffectBind::PartyArcadeNoScore),
            (0x6d5, EffectBind::Drained),
            (0x6f1, EffectBind::PartyArcade),
            (0x70d, EffectBind::PartyPartyP),
            (0x729, EffectBind::PartyPartyA),
            (0x745, EffectBind::PartyPartyR),
            (0x761, EffectBind::PartyPartyT),
            (0x77d, EffectBind::PartyPartyY),
            (0x799, EffectBind::PartyTunnel1M),
            (0x7b5, EffectBind::PartyTunnel3M),
            (0x7d1, EffectBind::PartyTunnel5M),
            (0x7ed, EffectBind::PartyDemon250k),
            (0x809, EffectBind::PartySnackNope),
            (0x825, EffectBind::PartyOrbit250k),
            (0x841, EffectBind::PartyOrbit500k),
            (0x85d, EffectBind::PartyOrbit750k),
            (0x879, EffectBind::PartyDemon5M),
            (0x895, EffectBind::PartyDuckAll),
            (0x8b1, EffectBind::PartySnack0),
            (0x8cd, EffectBind::PartySnack1),
            (0x8e9, EffectBind::PartySnack2),
            // unused 1M at 0x905
            (0x921, EffectBind::PartySkyrideLitMb), // seems to be a nop
            (0x93e, EffectBind::PartyOrbitMb2),
            (0x95a, EffectBind::PartyOrbitMb4),
            (0x976, EffectBind::PartyOrbitMb6),
            (0x992, EffectBind::PartyOrbitMb8),
            (0x9ae, EffectBind::PartyOrbitHoldBonus),
            (0x9ca, EffectBind::PartyOrbitDoubleBonus),
            (0x9e6, EffectBind::PartyDemonExtraBall),
            (0xa02, EffectBind::PartySideExtraBall),
            (0xa1e, EffectBind::PartyOrbitCrazy),
            (0xa3a, EffectBind::PartyArcadeCrazy),
            (0xa56, EffectBind::PartyOrbitMad0),
            (0xa72, EffectBind::PartyOrbitMad1),
            (0xa8e, EffectBind::PartyOrbitMad2),
            (0xaab, EffectBind::PartySkyride0),
            (0xac7, EffectBind::PartySkyride1),
            (0xae3, EffectBind::PartySkyride2),
            (0xaff, EffectBind::PartyCyclone),
            (0xb1b, EffectBind::PartyCycloneX5),
            (0xb37, EffectBind::PartySecret),
            (0xb54, EffectBind::PartyCycloneSkillShot),
            (0xb70, EffectBind::PartyTunnelSkillShot),
            (0xb8c, EffectBind::PartyRollInner),
            (0xba9, EffectBind::PartyHappyHour),
            (0xbc5, EffectBind::PartyHappyHourEnd),
            (0xbe1, EffectBind::PartyMegaLaugh),
            (0xbfd, EffectBind::PartyMegaLaughEnd),
        ],
        TableId::Table2 => &[
            (0x393, EffectBind::SpeedTurboRamp),
            (0x3af, EffectBind::SpeedMilesToJump),
            (0x3cc, EffectBind::SpeedMilesToFirstOffroad),
            (0x3e9, EffectBind::SpeedMilesToExtraBall),
            (0x406, EffectBind::SpeedMilesToOffroad),
            (0x423, EffectBind::SpeedSuperJackpot),
            (0x43f, EffectBind::SpeedJackpot),
            (0x45b, EffectBind::SpeedSuperJackpotGoal),
            (0x477, EffectBind::SpeedHoldBonus),
            (0x493, EffectBind::SpeedExtraGear),
            (0x4af, EffectBind::SpeedExtraBall),
            (0x4cb, EffectBind::SpeedMilesExtraBall),
            (0x4ea, EffectBind::SpeedJump),
            (0x506, EffectBind::SpeedMilesJump),
            (0x522, EffectBind::SpeedCar0),
            (0x53e, EffectBind::SpeedCar1),
            (0x55a, EffectBind::SpeedCar2),
            (0x576, EffectBind::SpeedCar3),
            (0x592, EffectBind::SpeedCar4),
            (0x5ae, EffectBind::SpeedGear),
            (0x5ca, EffectBind::SpeedPedalMetal),
            (0x5e6, EffectBind::SpeedOvertake),
            (0x602, EffectBind::SpeedOvertakeFinal),
            (0x61e, EffectBind::SpeedTurbo),
            (0x63a, EffectBind::Drained),
            (0x657, EffectBind::SpeedLaneOuter),
            (0x673, EffectBind::SpeedLaneInner),
            (0x690, EffectBind::SpeedPit),
            (0x6ad, EffectBind::SpeedPitAll),
            (0x6c9, EffectBind::SpeedOffroadExit),
            (0x6e6, EffectBind::SpeedRampOffroad),
            (0x704, EffectBind::SpeedMillion),
            (0x721, EffectBind::SpeedMiles0),
            (0x73e, EffectBind::SpeedMiles1),
            (0x75b, EffectBind::SpeedMiles2),
            (0x778, EffectBind::SpeedMiles3),
            (0x795, EffectBind::SpeedMiles4),
            (0x7b2, EffectBind::SpeedMiles5),
            (0x7cf, EffectBind::SpeedMiles6),
            (0x7ec, EffectBind::SpeedMiles7),
            (0x809, EffectBind::SpeedMiles8),
            (0x826, EffectBind::SpeedMiles9),
            (0x843, EffectBind::SpeedMiles10),
            (0x860, EffectBind::SpeedMiles11),
            (0x87d, EffectBind::SpeedMb2),
            (0x899, EffectBind::SpeedMb3),
            (0x8b5, EffectBind::SpeedMb4),
            (0x8d1, EffectBind::SpeedMb5),
            (0x8ed, EffectBind::SpeedMb6),
            (0x909, EffectBind::SpeedMb7),
            (0x925, EffectBind::SpeedMb8),
            (0x941, EffectBind::SpeedMb9),
        ],
        TableId::Table3 => &[
            (0x3be, EffectBind::ShowCashpotLock),
            (0x3da, EffectBind::ShowBillion),
            (0x3f6, EffectBind::ShowLaneOuter),
            (0x413, EffectBind::ShowLaneInner),
            (0x430, EffectBind::ShowRampRight),
            (0x44d, EffectBind::ShowRampTop),
            (0x46a, EffectBind::ShowRampLoop),
            (0x487, EffectBind::ShowTopEntry),
            (0x4a4, EffectBind::ShowSkillsEntry),
            (0x4c1, EffectBind::ShowRampSkills),
            (0x4de, EffectBind::ShowOrbitLeft),
            (0x4fb, EffectBind::ShowOrbitRight),
            (0x518, EffectBind::ShowLoopEntry),
            (0x535, EffectBind::ShowPrizeTv),
            (0x551, EffectBind::ShowPrizeTrip),
            (0x56d, EffectBind::ShowPrizeCar),
            (0x589, EffectBind::ShowPrizeBoat),
            (0x5a5, EffectBind::ShowPrizeHouse),
            (0x5c1, EffectBind::ShowPrizePlane),
            (0x5dd, EffectBind::ShowModeHit),
            (0x5f9, EffectBind::ShowModeRamp),
            (0x615, EffectBind::ShowJackpot),
            (0x63d, EffectBind::ShowSuperJackpot),
            (0x659, EffectBind::ShowExtraBall),
            (0x675, EffectBind::ShowRaisingMillions),
            (0x691, EffectBind::ShowSkillsToMoneyMania),
            (0x6ad, EffectBind::ShowSkillsToExtraBall),
            (0x6c9, EffectBind::ShowCashpot),
            (0x6e5, EffectBind::ShowCashpotX5),
            (0x701, EffectBind::ShowDropCenter),
            (0x71e, EffectBind::ShowDropLeft),
            (0x73b, EffectBind::ShowDollar),
            (0x758, EffectBind::ShowDollarBoth),
            (0x774, EffectBind::ShowRampTopTwice),
            (0x790, EffectBind::ShowLitTv),
            (0x7ac, EffectBind::ShowLitTrip),
            (0x7c8, EffectBind::ShowLitCar),
            (0x7e4, EffectBind::ShowLitBoat),
            (0x800, EffectBind::ShowLitHouse),
            (0x81c, EffectBind::ShowLitPlane),
            (0x838, EffectBind::Drained),
        ],
        TableId::Table4 => &[
            (0x33d, EffectBind::StonesLock),
            (0x359, EffectBind::StonesGhostDemon),
            (0x376, EffectBind::StonesStonesBonesAllRedundant),
            (0x393, EffectBind::StonesGhostLit0),
            (0x3af, EffectBind::StonesGhostLit1),
            (0x3cb, EffectBind::StonesGhostLit2),
            (0x3e7, EffectBind::StonesGhostLit3),
            (0x403, EffectBind::StonesGhostLit4),
            (0x41f, EffectBind::StonesGhostLit5),
            (0x43b, EffectBind::StonesGhostLit6),
            (0x457, EffectBind::StonesGhostLit7),
            (0x473, EffectBind::StonesGhostExtraBall),
            (0x48f, EffectBind::StonesTowerHunt0),
            (0x4ab, EffectBind::StonesTowerHunt1),
            (0x4c7, EffectBind::StonesTowerHunt2),
            (0x4e3, EffectBind::StonesGhost5M),
            (0x4ff, EffectBind::StonesGhost10M),
            (0x51b, EffectBind::StonesGhost15M),
            (0x537, EffectBind::StonesLoopCombo),
            (0x553, EffectBind::StonesScreamsExtraBall),
            (0x56f, EffectBind::StonesKickback),
            (0x58c, EffectBind::StonesSkillShot),
            (0x5a8, EffectBind::StonesTowerOpen),
            (0x5c4, EffectBind::StonesGhostGhostHunter),
            (0x5e0, EffectBind::StonesGhostGrimReaper),
            (0x5fc, EffectBind::StonesGhostTowerHunt),
            (0x618, EffectBind::StonesTopMillion),
            (0x634, EffectBind::StonesTowerMillion),
            (0x650, EffectBind::StonesDemon5M),
            (0x66c, EffectBind::StonesTower5M),
            (0x688, EffectBind::StonesTowerExtraBall),
            (0x6a4, EffectBind::StonesDemon10M),
            (0x6c0, EffectBind::StonesDemon20M),
            (0x6dc, EffectBind::StonesTowerHoldBonus),
            (0x6f8, EffectBind::StonesTowerDoubleBonus),
            (0x714, EffectBind::StonesTowerJackpot),
            (0x730, EffectBind::StonesTowerSuperJackpot),
            (0x74c, EffectBind::StonesMillionPlus),
            (0x768, EffectBind::StonesVault),
            (0x784, EffectBind::StonesWell),
            (0x7a0, EffectBind::StonesTowerBonus),
            (0x7bc, EffectBind::StonesWellMb2),
            (0x7d8, EffectBind::StonesWellMb4),
            (0x7f4, EffectBind::StonesWellMb6),
            (0x810, EffectBind::StonesWellMb8),
            (0x82c, EffectBind::StonesWellMb10),
            (0x848, EffectBind::StonesScreamsToExtraBall),
            (0x864, EffectBind::StonesScreamsTo5M),
            (0x880, EffectBind::Drained),
        ],
    };
    for &(off, bind) in effects {
        res[bind] = Some(extract_effect(exe, off, uops_by_addr));
    }
    res
}
