#![allow(clippy::too_many_arguments)]

use enum_map::{enum_map, Enum, EnumMap};
use ndarray::prelude::*;
use unnamed_entity::{entity_id, EntityVec};

use crate::{assets::mz::MzExe, bcd::Bcd, config::TableId};

use super::sound::{extract_sfx, Sfx};

entity_id! {
    pub id BumperId u8, reserve 1;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Enum)]
pub enum Layer {
    Ground,
    Overhead,
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub xy_min: (u16, u16),
    pub xy_max: (u16, u16),
}

#[derive(Copy, Clone, Debug)]
pub struct Bumper {
    pub is_kicker: bool,
    pub rect: Rect,
    pub sfx: Sfx,
    pub score: Bcd,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HitTrigger {
    PartyArcadeButton,
    PartyDuck(u8),
    SpeedBur(u8),
    SpeedNin(u8),
    ShowDollar(u8),
    ShowCenter(u8),
    ShowLeft(u8),
    StonesBone(u8),
    StonesStone(u8),
}

#[derive(Copy, Clone, Debug)]
pub struct HitTriggerArea {
    pub rect: Rect,
    pub kind: HitTrigger,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RollTrigger {
    Dummy,
    PlungerBottom,
    PlungerGo,
    PartyLaneInner,
    PartyLaneOuter,
    PartyOrbitTopLeft,
    PartyOrbitTopRight,
    PartySecret,
    PartyTunnel,
    PartyArcade,
    PartyOrbitEntryRight,
    PartyEnter,
    PartyDemon,
    PartySkyrideTop,
    PartySkyrideRamp,
    PartySkyridePuke(u8),
    PartyRampCyclone,
    PartyRampSnack,
    PartySecretTilt,
    PartyTunnelTilt,
    SpeedLaneInner,
    SpeedLaneOuter,
    SpeedPitStop,
    SpeedEnter,
    SpeedPitLoopJump,
    SpeedRampOffroad,
    SpeedPitLoopPre,
    SpeedPit(u8),
    SpeedOffroadExit,
    SpeedRampMilesRight,
    SpeedRampMilesLeft,
    SpeedJumpPre,
    SpeedPlungerExit,
    ShowLaneInner,
    ShowLaneOuter,
    ShowEnter,
    ShowOrbitLeft,
    ShowOrbitRight,
    ShowCashpot,
    ShowVault,
    ShowVaultExit,
    ShowRampSkillEntry,
    ShowRampTopEntry,
    ShowRampLoopEntry,
    ShowRampTop,
    ShowRampSkillMark,
    ShowRampSkill,
    ShowRampRight,
    ShowRampLoop,
    ShowRampTopSecondary,
    StonesLaneInnerLeft,
    StonesLaneInnerRight,
    StonesLaneOuterLeft,
    StonesLaneOuterRight,
    StonesKeyEntry,
    StonesRampTower,
    StonesKey(u8),
    StonesWell,
    StonesVault,
    StonesKeyClose,
    StonesTower,
    StonesRampTop,
    StonesRip(u8),
    StonesRampTopExit,
    StonesRampScreams,
    StonesRampLeftToLane,
    StonesRampLeftToVault,
    StonesRampLeftFixup0,
    StonesRampLeftFixup1,
    StonesRampLeftFixup2,
    StonesRampLeftFixup3,
    StonesVaultExit,
    StonesEnter,
    StonesWellTilt,
    StonesTowerTilt,
}

#[derive(Copy, Clone, Debug)]
pub struct RollTriggerArea {
    pub rect: Rect,
    pub kind: RollTrigger,
}

impl Rect {
    pub fn contains(self, pos: (u16, u16)) -> bool {
        pos.0 >= self.xy_min.0
            && pos.0 <= self.xy_max.0
            && pos.1 >= self.xy_min.1
            && pos.1 <= self.xy_max.1
    }
}

#[derive(Clone, Debug)]
pub struct PhysmapPatch {
    pub layer: Layer,
    pub pos: (u16, u16),
    pub raised: Array2<u8>,
    pub dropped: Array2<u8>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum)]
pub enum PhysmapBind {
    PartyGateSkyride,
    PartyHitDuck0,
    PartyHitDuck1,
    PartyHitDuck2,
    ShowGateVaultEntry,
    ShowGatePlunger,
    ShowGateRampRight,
    ShowGateVaultExit,
    ShowHitLeft0,
    ShowHitLeft1,
    ShowHitCenter0,
    ShowHitCenter1,
    StonesGateRampLeft0,
    StonesGateRampLeft1,
    StonesGateRampLeft2,
    StonesGateTowerEntry,
    StonesGateRampTower,
    StonesGateKickback,
}

#[derive(Copy, Clone, Debug)]
pub struct Ramp {
    pub accel: (i16, i16),
    pub accel_hires: (i16, i16),
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub unk0: i16,
    pub unk2: i16,
    pub bounce_factor: i16,
    pub min_bounce_speed: i16,
    pub max_bounce_angle: i16,
}

pub const MATERIAL_FLIPPER: usize = 2;
pub const MATERIAL_KICKER: usize = 3;
pub const MATERIAL_BUMPER: usize = 7;

pub const MATERIALS: [Material; 8] = [
    // 0: dummy
    Material {
        unk0: 1792,
        unk2: 448,
        bounce_factor: 400,
        min_bounce_speed: 300,
        max_bounce_angle: 38,
    },
    // 1: dummy
    Material {
        unk0: 1792,
        unk2: 448,
        bounce_factor: 400,
        min_bounce_speed: 600,
        max_bounce_angle: 18,
    },
    // 2: flipper, patch
    Material {
        unk0: 1792,
        unk2: 448,
        bounce_factor: 400,
        min_bounce_speed: 600,
        max_bounce_angle: 18,
    },
    // 3: rubber [kickers]
    Material {
        unk0: 896,
        unk2: 224,
        bounce_factor: 875,
        min_bounce_speed: 200,
        max_bounce_angle: 38,
    },
    // 4: dummy
    Material {
        unk0: 1792,
        unk2: 448,
        bounce_factor: 400,
        min_bounce_speed: 300,
        max_bounce_angle: 38,
    },
    // 5: dummy
    Material {
        unk0: 30000,
        unk2: 7500,
        bounce_factor: 1000,
        min_bounce_speed: 400,
        max_bounce_angle: 38,
    },
    // 6: steel
    Material {
        unk0: 10000,
        unk2: 2500,
        bounce_factor: 450,
        min_bounce_speed: 700,
        max_bounce_angle: 38,
    },
    // 7: plastic [bumpers]
    Material {
        unk0: 10000,
        unk2: 2500,
        bounce_factor: 400,
        min_bounce_speed: 500,
        max_bounce_angle: 38,
    },
];

pub(super) fn extract_physmaps(exe: &MzExe, table: TableId) -> EnumMap<Layer, Array2<u8>> {
    enum_map! {
        layer => {
            let (s0, s1, s2) = match (table, layer) {
                (TableId::Table1, Layer::Ground) => (0x4114, 0x3b74, 0x7194),
                (TableId::Table1, Layer::Overhead) => (0x7734, 0x46b4, 0x7cd4),
                (TableId::Table2, Layer::Ground) => (0x3e58, 0x38b8, 0x6d5d),
                (TableId::Table2, Layer::Overhead) => (0x72fd, 0x43f8, 0x789d),
                (TableId::Table3, Layer::Ground) => (0x308f, 0x2aef, 0x6a1f),
                (TableId::Table3, Layer::Overhead) => (0x6fbf, 0x362f, 0x755f),
                (TableId::Table4, Layer::Ground) => (0x39fb, 0x345b, 0x6e67),
                (TableId::Table4, Layer::Overhead) => (0x7407, 0x3f9b, 0x79a7),
            };
            Array2::from_shape_fn((320, 576), |(x, y)| {
                let off = (x / 8 + y * 40) as u16;
                let b0 = exe.byte(s0, off);
                let b1 = exe.byte(s1, off);
                let b2 = exe.byte(s2, off);
                let bit0 = b0 >> (7 - x % 8) & 1;
                let bit1 = b1 >> (7 - x % 8) & 1;
                let bit2 = b2 >> (7 - x % 8) & 1;
                let mut val = bit2 << 2 | bit1 << 1 | bit0;
                if off != 0 && exe.byte(s0, off - 1) == 0 && exe.byte(s1, off - 1) == 0 {
                    val |= exe.byte(s2, off - 1) << 4;
                } else if exe.byte(s0, off) == 0 && exe.byte(s1, off) == 0 {
                    val |= exe.byte(s2, off) << 4;
                } else if exe.byte(s0, off + 1) == 0 && exe.byte(s1, off + 1) == 0 {
                    val |= exe.byte(s2, off + 1) << 4;
                } else {
                    val |= 0xf0;
                }
                val
            })
        }
    }
}

pub(super) fn extract_sine_table(exe: &MzExe, table: TableId) -> [i16; 0xa00] {
    let off = match table {
        TableId::Table1 => 0x4600,
        TableId::Table2 => 0x4690,
        TableId::Table3 => 0x3ee0,
        TableId::Table4 => 0x4bf0,
    };
    core::array::from_fn(|i| exe.data_word_s(off + (i as u16) * 2).swap_bytes())
}

fn xlat_physmap_addr(addr: u16) -> (u16, u16) {
    (addr % 0x28 * 8, addr / 0x28)
}

fn extract_physmap_rect(
    physmaps: &EnumMap<Layer, Array2<u8>>,
    layer: Layer,
    pos: (u16, u16),
    width: u16,
    height: u16,
) -> Array2<u8> {
    physmaps[layer]
        .slice(s![
            (pos.0 as usize)..((pos.0 + width * 8) as usize),
            (pos.1 as usize)..((pos.1 + height) as usize),
        ])
        .to_owned()
}

fn extract_physmap_rect_patched(
    exe: &MzExe,
    physmaps: &EnumMap<Layer, Array2<u8>>,
    layer: Layer,
    pos: (u16, u16),
    width: u16,
    height: u16,
    off: u16,
    skip3: bool,
) -> Array2<u8> {
    let mut res = extract_physmap_rect(physmaps, layer, pos, width, height);
    for y in 0..height {
        for bx in 0..width {
            let byte = exe.data_byte(off + if skip3 { y * 3 + 1 } else { y } * width + bx);
            for dx in 0..8 {
                let bit = byte >> (7 - dx) & 1;
                let xy = ((bx * 8 + dx) as usize, y as usize);
                res[xy] = (res[xy] & !2) | bit << 1;
            }
        }
    }
    res
}

pub(super) fn extract_physmap_rect_patched_or(
    exe: &MzExe,
    physmaps: &EnumMap<Layer, Array2<u8>>,
    layer: Layer,
    pos: (u16, u16),
    width: u16,
    height: u16,
    seg: u16,
    off: u16,
) -> Array2<u8> {
    let mut res = extract_physmap_rect(physmaps, layer, pos, width, height);
    for y in 0..height {
        for bx in 0..width {
            let byte = exe.byte(seg, off + y * width + bx);
            for dx in 0..8 {
                let bit = byte >> (7 - dx) & 1;
                let xy = ((bx * 8 + dx) as usize, y as usize);
                res[xy] |= bit << 1;
            }
        }
    }
    res
}

fn extract_physmap_patch_raw(
    exe: &MzExe,
    physmaps: &EnumMap<Layer, Array2<u8>>,
    layer: Layer,
    pos: (u16, u16),
    width: u16,
    height: u16,
    off_raised: u16,
    off_dropped: u16,
    skip3: bool,
) -> PhysmapPatch {
    PhysmapPatch {
        layer,
        pos,
        raised: extract_physmap_rect_patched(
            exe, physmaps, layer, pos, width, height, off_raised, skip3,
        ),
        dropped: extract_physmap_rect_patched(
            exe,
            physmaps,
            layer,
            pos,
            width,
            height,
            off_dropped,
            skip3,
        ),
    }
}

fn extract_physmap_patch_formatted(
    exe: &MzExe,
    physmaps: &EnumMap<Layer, Array2<u8>>,
    layer: Layer,
    off: u16,
) -> PhysmapPatch {
    extract_physmap_patch_raw(
        exe,
        physmaps,
        layer,
        xlat_physmap_addr(exe.data_word(off + 4)),
        exe.data_word(off + 6),
        exe.data_word(off + 8),
        exe.data_word(off),
        exe.data_word(off + 2),
        true,
    )
}

pub fn extract_physmap_patches(
    exe: &MzExe,
    table: TableId,
    physmaps: &EnumMap<Layer, Array2<u8>>,
) -> EnumMap<PhysmapBind, Option<PhysmapPatch>> {
    enum_map! {
        bind => match (table, bind) {
            (TableId::Table1, PhysmapBind::PartyGateSkyride) => {
                let pos = xlat_physmap_addr(0x266);
                let width = 2;
                let height = 18;
                Some(PhysmapPatch {
                    layer: Layer::Overhead,
                    pos,
                    raised: extract_physmap_rect(physmaps, Layer::Overhead, pos, width, height),
                    dropped: extract_physmap_rect_patched(
                        exe,
                        physmaps,
                        Layer::Overhead,
                        pos,
                        width,
                        height,
                        0x1332,
                        false,
                    ),
                })
            }
            (TableId::Table1, PhysmapBind::PartyHitDuck0) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x2b5a),
                2,
                15,
                0x68b0,
                0x68d0,
                false,
            )),
            (TableId::Table1, PhysmapBind::PartyHitDuck1) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x2e2b),
                2,
                15,
                0x68f0,
                0x6910,
                false,
            )),
            (TableId::Table1, PhysmapBind::PartyHitDuck2) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x30fc),
                1,
                15,
                0x6940,
                0x6930,
                false,
            )),
            (TableId::Table3, PhysmapBind::ShowGatePlunger) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x2fcb),
                2,
                34,
                0x6280,
                0x6230,
                false,
            )),
            (TableId::Table3, PhysmapBind::ShowGateRampRight) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Overhead,
                xlat_physmap_addr(0x198e),
                4,
                20,
                0x6480,
                0x6390,
                true,
            )),
            (TableId::Table3, PhysmapBind::ShowGateVaultEntry) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Overhead,
                xlat_physmap_addr(0x0f00),
                3,
                25,
                0x61e0,
                0x6190,
                false,
            )),
            (TableId::Table3, PhysmapBind::ShowGateVaultExit) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x4fd8),
                4,
                14,
                0x6620,
                0x6570,
                true,
            )),
            (TableId::Table3, PhysmapBind::ShowHitCenter0) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x2389),
                2,
                16,
                0x6370,
                0x62d0,
                false,
            )),
            (TableId::Table3, PhysmapBind::ShowHitCenter1) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x26a9),
                1,
                16,
                0x6360,
                0x62f0,
                false,
            )),
            (TableId::Table3, PhysmapBind::ShowHitLeft0) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x2994),
                1,
                16,
                0x6350,
                0x6300,
                false,
            )),
            (TableId::Table3, PhysmapBind::ShowHitLeft1) => Some(extract_physmap_patch_raw(
                exe,
                physmaps,
                Layer::Ground,
                xlat_physmap_addr(0x2cb3),
                2,
                16,
                0x6330,
                0x6310,
                false,
            )),
            (TableId::Table4, PhysmapBind::StonesGateKickback) => Some(
                extract_physmap_patch_formatted(exe, physmaps, Layer::Ground, 0x1265),
            ),
            (TableId::Table4, PhysmapBind::StonesGateTowerEntry) => Some(
                extract_physmap_patch_formatted(exe, physmaps, Layer::Ground, 0x123d),
            ),
            (TableId::Table4, PhysmapBind::StonesGateRampTower) => Some(
                extract_physmap_patch_formatted(exe, physmaps, Layer::Ground, 0x1233),
            ),
            (TableId::Table4, PhysmapBind::StonesGateRampLeft0) => Some(
                extract_physmap_patch_formatted(exe, physmaps, Layer::Overhead, 0x1247),
            ),
            (TableId::Table4, PhysmapBind::StonesGateRampLeft1) => Some(
                extract_physmap_patch_formatted(exe, physmaps, Layer::Overhead, 0x1251),
            ),
            (TableId::Table4, PhysmapBind::StonesGateRampLeft2) => Some(
                extract_physmap_patch_formatted(exe, physmaps, Layer::Overhead, 0x125b),
            ),
            _ => None,
        },
    }
}

pub(super) fn extract_ramps(exe: &MzExe, table: TableId) -> Vec<Ramp> {
    let (off, off_hires, num) = match table {
        TableId::Table1 => (0x5e, 0x72, 4),
        TableId::Table2 => (0x7b, 0x97, 6),
        TableId::Table3 => (0x233, 0x24b, 5),
        TableId::Table4 => (0xca2, 0xcd2, 11),
    };
    (0..num)
        .map(|i| Ramp {
            accel: (
                exe.data_word_s(off + i * 4),
                exe.data_word_s(off + i * 4 + 2),
            ),
            accel_hires: (
                exe.data_word_s(off_hires + i * 4),
                exe.data_word_s(off_hires + i * 4 + 2),
            ),
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
pub struct BallOutlinePixel {
    pub x: u16,
    pub y: u16,
    pub angle: u16,
    pub quad: u8,
    pub idx: u8,
    pub is_bot: bool,
    pub is_right: bool,
}

pub(super) fn extract_ball_outline(exe: &MzExe, table: TableId) -> Vec<BallOutlinePixel> {
    let (mut pos, end) = match table {
        TableId::Table1 => (0x8866, 0x8c34),
        TableId::Table2 => (0x8056, 0x8424),
        TableId::Table3 => (0x7ae6, 0x7eb4),
        TableId::Table4 => (0x8ff6, 0x93c4),
    };
    let mut res = vec![];
    let mut byte_off = 0;
    while pos != end {
        if exe.code_byte(pos) == 0x26 {
            assert_eq!(exe.code_byte(pos + 1), 0x8b);
            byte_off = match exe.code_byte(pos + 2) {
                0x04 => {
                    pos += 3;
                    0
                }
                0x44 => {
                    pos += 4;
                    let byte = exe.code_byte(pos - 1);
                    assert_eq!(byte & 0x80, 0);
                    byte as u16
                }
                0x84 => {
                    pos += 5;
                    exe.code_word(pos - 2)
                }
                _ => unreachable!(),
            } + 1;
            assert_eq!(exe.code_byte(pos), 0xd3);
            assert_eq!(exe.code_byte(pos + 1), 0xc0);
            pos += 2;
        }
        assert_eq!(exe.code_byte(pos), 0xa8);
        let bit = exe.code_byte(pos + 1);
        let y = byte_off / 0x28;
        let x = (byte_off % 0x28) * 8
            + match bit {
                0x80 => 0,
                0x40 => 1,
                0x20 => 2,
                0x10 => 3,
                0x08 => 4,
                0x04 => 5,
                0x02 => 6,
                0x01 => 7,
                _ => unreachable!(),
            }
            - 7;
        pos += 2;
        assert_eq!(exe.code_byte(pos), 0x74);
        let cur_end = pos + 2 + exe.code_byte(pos + 1) as u16;
        pos += 2;
        assert_eq!(exe.code_byte(pos + 1), 0xc5);
        let angle = match exe.code_byte(pos) {
            0x81 => {
                pos += 4;
                exe.code_word(pos - 2)
            }
            0x83 => {
                let byte = exe.code_byte(pos + 2);
                pos += 3;
                assert_eq!(byte & 0x80, 0);
                byte as u16
            }
            _ => unreachable!(),
        };
        assert_eq!(exe.code_byte(pos), 0x83);
        assert_eq!(exe.code_byte(pos + 1), 0xcf);
        let quad = exe.code_byte(pos + 2);
        pos += 3;
        assert_eq!(exe.code_byte(pos), 0xb5);
        let idx = exe.code_byte(pos + 1);
        pos += 2;
        assert_eq!(exe.code_byte(pos), 0xfe);
        let is_bot = match exe.code_byte(pos + 1) {
            0xc2 => false,
            0xc6 => true,
            _ => unreachable!(),
        };
        pos += 2;
        assert_eq!(exe.code_byte(pos), 0xfe);
        let is_right = match exe.code_byte(pos + 1) {
            0xc3 => false,
            0xc7 => true,
            _ => unreachable!(),
        };
        pos += 2;
        assert_eq!(cur_end, pos);

        res.push(BallOutlinePixel {
            x,
            y,
            angle,
            quad,
            idx,
            is_bot,
            is_right,
        });
    }
    res
}

fn extract_rect(exe: &MzExe, off: u16) -> Rect {
    Rect {
        xy_min: (exe.data_word(off), exe.data_word(off + 2)),
        xy_max: (exe.data_word(off + 4), exe.data_word(off + 6)),
    }
}

fn extract_transition_list(exe: &MzExe, mut off: u16) -> Vec<Rect> {
    let mut res = vec![];
    while exe.data_word(off) != 0xffff {
        res.push(extract_rect(exe, off));
        off += 8;
    }
    res
}

pub(super) fn extract_transitions(exe: &MzExe, table: TableId) -> (Vec<Rect>, Vec<Rect>) {
    let (off_d, off_u) = match table {
        TableId::Table1 => (0xec5, 0xf17),
        TableId::Table2 => (0xc55, 0xc97),
        TableId::Table3 => (0xad8, 0xb2a),
        TableId::Table4 => (0xc1e, 0xc70),
    };
    (
        extract_transition_list(exe, off_d),
        extract_transition_list(exe, off_u),
    )
}

pub(super) fn extract_bumpers(exe: &MzExe, table: TableId) -> EntityVec<BumperId, Bumper> {
    let (off_b, off_k) = match table {
        TableId::Table1 => (0xcf3, 0xd1b),
        TableId::Table2 => (0xaad, 0xadf),
        TableId::Table3 => (0x924, 0x94c),
        TableId::Table4 => (0x96a, 0x99c),
    };
    let mut res = EntityVec::new();
    for (mut pos, is_kicker) in [(off_b, false), (off_k, true)] {
        while exe.data_word(pos) != 0xffff {
            let rect = extract_rect(exe, pos);
            let ptr = exe.data_word(pos + 8);
            let sfx = extract_sfx(exe, exe.data_word(ptr));
            let score = exe.data_bcd(ptr + 2);
            pos += 10;
            res.push(Bumper {
                is_kicker,
                rect,
                sfx,
                score,
            });
        }
    }
    res
}

fn extract_roll_trigger_list(exe: &MzExe, table: TableId, mut pos: u16) -> Vec<RollTriggerArea> {
    let mut res = vec![];
    while exe.data_word(pos) != 0 {
        let rect = extract_rect(exe, pos);
        let ptr = exe.data_word(pos + 8);
        pos += 10;
        let kind = match table {
            TableId::Table1 => match ptr {
                0x16b6 => RollTrigger::PartyOrbitTopLeft,
                0x18af => RollTrigger::PartyOrbitTopRight,
                0x1a0e => RollTrigger::PartySecret,
                0x1a6c => RollTrigger::PartySecretTilt,
                0x1b29 => RollTrigger::PartyTunnel,
                0x1c9c => RollTrigger::PartyTunnelTilt,
                0x1cda => RollTrigger::PartyArcade,
                0x1f92 => RollTrigger::PartyRampSnack,
                0x225d => RollTrigger::PartyOrbitEntryRight,
                0x2264 => RollTrigger::PartyEnter,
                0x2299 => RollTrigger::PartyDemon,
                0x24bc | 0x24d7 => RollTrigger::PartyLaneInner,
                0x24f2 | 0x2577 => RollTrigger::PartyLaneOuter,
                0x25c2 => RollTrigger::PlungerGo,
                0x25d5 => RollTrigger::PlungerBottom,
                0x25dc => RollTrigger::PartySkyrideTop,
                0x26ca => RollTrigger::PartySkyrideRamp,
                0x26ce => RollTrigger::PartySkyridePuke(0),
                0x272b => RollTrigger::PartySkyridePuke(3),
                0x2788 => RollTrigger::PartySkyridePuke(1),
                0x27e5 => RollTrigger::PartySkyridePuke(2),
                0x29e7 | 0x29e8 => RollTrigger::Dummy,
                0x29e9 => RollTrigger::PartyRampCyclone,
                _ => panic!("unk roll trigger {ptr:04x}"),
            },
            TableId::Table2 => match ptr {
                0x11d1 => RollTrigger::SpeedPlungerExit,
                0x193e => RollTrigger::SpeedPitStop,
                0x1c04 => RollTrigger::SpeedRampOffroad,
                0x1d1f => RollTrigger::SpeedPitLoopJump,
                0x1ec5 => RollTrigger::SpeedPitLoopPre,
                0x1ec6 => RollTrigger::SpeedJumpPre,
                0x1ee3 => RollTrigger::SpeedPit(2),
                0x1f4f => RollTrigger::SpeedPit(1),
                0x1fc0 => RollTrigger::SpeedPit(0),
                0x212f => RollTrigger::SpeedLaneOuter,
                0x2136 => RollTrigger::SpeedLaneInner,
                0x2151 => RollTrigger::SpeedOffroadExit,
                0x2158 => RollTrigger::SpeedEnter,
                0x218d => RollTrigger::PlungerGo,
                0x2194 => RollTrigger::PlungerBottom,
                0x2231 => RollTrigger::SpeedRampMilesRight,
                0x224e => RollTrigger::SpeedRampMilesLeft,
                _ => panic!("unk roll trigger {ptr:04x}"),
            },
            TableId::Table3 => match ptr {
                0x1077 => RollTrigger::ShowEnter,
                0x10a9 => RollTrigger::ShowVaultExit,
                0x10ad => RollTrigger::PlungerGo,
                0x10b7 => RollTrigger::PlungerBottom,
                0x1219 => RollTrigger::ShowVault,
                0x1587 => RollTrigger::ShowCashpot,
                0x1713 => RollTrigger::ShowRampRight,
                0x1768 => RollTrigger::ShowRampLoop,
                0x18d6 => RollTrigger::ShowOrbitLeft,
                0x19bd => RollTrigger::ShowOrbitRight,
                0x1a66 => RollTrigger::ShowRampSkillMark,
                0x1a67 => RollTrigger::ShowRampSkill,
                0x1cd9 | 0x1cf4 => RollTrigger::ShowLaneOuter,
                0x1d0f | 0x1d2a => RollTrigger::ShowLaneInner,
                0x1d45 => RollTrigger::ShowRampTopEntry,
                0x1d4c => RollTrigger::ShowRampSkillEntry,
                0x1d53 => RollTrigger::ShowRampLoopEntry,
                0x1d5a => RollTrigger::ShowRampTopSecondary,
                0x1d5e => RollTrigger::ShowRampTop,
                _ => panic!("unk roll trigger {ptr:04x}"),
            },
            TableId::Table4 => match ptr {
                0x1738 => RollTrigger::StonesEnter,
                0x1768 => RollTrigger::StonesKeyEntry,
                0x1783 => RollTrigger::Dummy,
                0x1784 => RollTrigger::StonesRampTower,
                0x178e => RollTrigger::StonesKeyClose,
                0x17cd => RollTrigger::StonesKey(0),
                0x18da => RollTrigger::StonesKey(1),
                0x19e7 => RollTrigger::StonesKey(2),
                0x1c68 => RollTrigger::StonesTower,
                0x20bf => RollTrigger::StonesTowerTilt,
                0x21c2 => RollTrigger::StonesWell,
                0x23bd => RollTrigger::StonesWellTilt,
                0x23c9 => RollTrigger::Dummy,
                0x23ca => RollTrigger::StonesLaneInnerLeft,
                0x2425 => RollTrigger::StonesLaneInnerRight,
                0x2480 => RollTrigger::StonesLaneOuterLeft,
                0x24c3 => RollTrigger::StonesLaneOuterRight,
                0x2506 => RollTrigger::StonesVault,
                0x29f6 => RollTrigger::Dummy,
                0x29f7 => RollTrigger::PlungerBottom,
                0x29fe => RollTrigger::PlungerGo,
                0x2a05 => RollTrigger::StonesRampTop,
                0x2b5a => RollTrigger::StonesRip(0),
                0x2c19 => RollTrigger::StonesRip(1),
                0x2cd8 => RollTrigger::StonesRip(2),
                0x2d97 => RollTrigger::StonesRampTopExit,
                0x2d9e => RollTrigger::StonesRampScreams,
                0x2ff7 => RollTrigger::StonesRampLeftToLane,
                0x30f5 => RollTrigger::StonesRampLeftToVault,
                0x3164 => RollTrigger::StonesRampLeftFixup0,
                0x316b => RollTrigger::StonesRampLeftFixup1,
                0x3172 => RollTrigger::StonesRampLeftFixup2,
                0x3179 => RollTrigger::StonesRampLeftFixup3,
                0x3180 => RollTrigger::StonesVaultExit,

                _ => panic!("unk roll trigger {ptr:04x}"),
            },
        };
        res.push(RollTriggerArea { rect, kind });
    }
    res
}

pub(super) fn extract_roll_triggers(
    exe: &MzExe,
    table: TableId,
) -> (
    EnumMap<Layer, Vec<RollTriggerArea>>,
    EnumMap<Layer, Vec<RollTriggerArea>>,
) {
    let (off_g, off_o, off_gt, off_ot) = match table {
        TableId::Table1 => (0xd9b, 0xe29, 0xea3, 0xec3),
        TableId::Table2 => (0xb91, 0xc29, 0xc53, 0xc53),
        TableId::Table3 => (0x9e0, 0xa78, 0xaca, 0xad6),
        TableId::Table4 => (0xa5e, 0xb14, 0xbac, 0xbcc),
    };
    (
        enum_map! {
            Layer::Ground => extract_roll_trigger_list(exe, table, off_g),
            Layer::Overhead => extract_roll_trigger_list(exe, table, off_o),
        },
        enum_map! {
            Layer::Ground => extract_roll_trigger_list(exe, table, off_gt),
            Layer::Overhead => extract_roll_trigger_list(exe, table, off_ot),
        },
    )
}

pub(super) fn extract_hit_triggers(exe: &MzExe, table: TableId) -> Vec<HitTriggerArea> {
    let mut pos = match table {
        TableId::Table1 => 0xd71,
        TableId::Table2 => 0xb51,
        TableId::Table3 => 0x9a2,
        TableId::Table4 => 0xa00,
    };
    let mut res = vec![];
    while exe.data_word(pos) != 0 {
        let rect = extract_rect(exe, pos);
        let ptr = exe.data_word(pos + 8);
        let kind = match table {
            TableId::Table1 => match ptr {
                0x134c => HitTrigger::PartyArcadeButton,
                0x13ae => HitTrigger::PartyDuck(0),
                0x142e => HitTrigger::PartyDuck(1),
                0x14ae => HitTrigger::PartyDuck(2),
                _ => panic!("unk hit trigger {ptr:04x}"),
            },
            TableId::Table2 => match ptr {
                0x11de => HitTrigger::SpeedBur(0),
                0x1236 => HitTrigger::SpeedBur(1),
                0x128e => HitTrigger::SpeedBur(2),
                0x12e6 => HitTrigger::SpeedNin(0),
                0x133e => HitTrigger::SpeedNin(1),
                0x1396 => HitTrigger::SpeedNin(2),
                _ => panic!("unk hit trigger {ptr:04x}"),
            },
            TableId::Table3 => match ptr {
                0xf57 => HitTrigger::ShowDollar(0),
                0xf95 => HitTrigger::ShowDollar(1),
                0xdeb => HitTrigger::ShowCenter(0),
                0xe2b => HitTrigger::ShowCenter(1),
                0xea1 => HitTrigger::ShowLeft(0),
                0xee1 => HitTrigger::ShowLeft(1),
                _ => panic!("unk hit trigger {ptr:04x}"),
            },
            TableId::Table4 => match ptr {
                0x103e => HitTrigger::StonesBone(0),
                0x10ba => HitTrigger::StonesBone(1),
                0x1136 => HitTrigger::StonesBone(2),
                0x11b2 => HitTrigger::StonesBone(3),
                0x122e => HitTrigger::StonesStone(0),
                0x12aa => HitTrigger::StonesStone(1),
                0x1326 => HitTrigger::StonesStone(2),
                0x13a2 => HitTrigger::StonesStone(3),
                0x141e => HitTrigger::StonesStone(4),
                _ => panic!("unk hit trigger {ptr:04x}"),
            },
        };
        pos += 10;
        res.push(HitTriggerArea { rect, kind });
    }
    res
}
