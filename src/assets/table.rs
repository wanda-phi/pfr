use std::{collections::HashMap, path::Path};

use arrayvec::ArrayVec;
use enum_map::EnumMap;
use ndarray::prelude::*;
use unnamed_entity::EntityVec;

use crate::{
    assets::table::{
        dm::{extract_dm_fonts, extract_dm_tower, DmFont},
        flippers::extract_flippers,
        gfx::{extract_ball, extract_lights, extract_main_board, extract_spring},
        lights::{extract_attract_lights, extract_light_binds},
        physics::{
            extract_ball_outline, extract_bumpers, extract_hit_triggers, extract_physmap_patches,
            extract_physmaps, extract_ramps, extract_roll_triggers, extract_sine_table,
            extract_transitions,
        },
        script::{extract_cheats, extract_effects, extract_script_binds, extract_scripts},
        sound::{extract_jingle_binds, extract_sfx_binds},
    },
    bcd::Bcd,
    config::TableId,
};

use sound::{Jingle, JingleBind, Sfx, SfxBind};

use self::{
    dm::DmPalette,
    flippers::{Flipper, FlipperId},
    lights::{AttractLight, AttractLightId, Light, LightBind, LightId},
    physics::{
        BallOutlinePixel, Bumper, BumperId, HitTriggerArea, Layer, PhysmapBind, PhysmapPatch, Ramp,
        Rect, RollTriggerArea,
    },
    script::{
        Cheat, DmAnim, DmAnimFrame, DmAnimFrameId, DmAnimId, Effect, EffectBind, MsgId, ScriptBind,
        ScriptPosId, Uop,
    },
};

use super::{iff::Image, mz::MzExe};

pub mod dm;
pub mod flippers;
mod gfx;
pub mod lights;
pub mod physics;
pub mod script;
pub mod sound;

#[derive(Clone, Debug)]
pub struct Assets {
    pub table: TableId,
    pub exe: MzExe,

    pub main_board: Image,
    pub spring: Image,
    pub ball: Image,
    pub occmaps: EnumMap<Layer, Array2<u8>>,
    pub physmaps: EnumMap<Layer, Array2<u8>>,
    pub physmap_patches: EnumMap<PhysmapBind, Option<PhysmapPatch>>,
    pub ramps: Vec<Ramp>,
    pub ball_outline: Vec<BallOutlinePixel>,
    pub ball_outline_by_angle: Vec<(u16, u16)>,

    pub lights: EntityVec<LightId, Light>,
    pub attract_lights: EntityVec<AttractLightId, AttractLight>,
    pub dm_palette: DmPalette,
    pub dm_fonts: EnumMap<DmFont, HashMap<u8, ArrayVec<u8, 13>>>,
    pub flippers: EntityVec<FlipperId, Flipper>,
    pub light_binds: EnumMap<LightBind, Vec<LightId>>,
    pub dm_tower: Option<Box<[[bool; 160]; 167]>>,

    pub transitions_down: Vec<Rect>,
    pub transitions_up: Vec<Rect>,
    pub bumpers: EntityVec<BumperId, Bumper>,
    pub roll_triggers: EnumMap<Layer, Vec<RollTriggerArea>>,
    pub roll_triggers_tilt: EnumMap<Layer, Vec<RollTriggerArea>>,
    pub hit_triggers: Vec<HitTriggerArea>,

    pub jingle_binds: EnumMap<JingleBind, Option<Jingle>>,
    pub sfx_binds: EnumMap<SfxBind, Option<Sfx>>,
    pub position_jingle_start: u8,

    pub scripts: EntityVec<ScriptPosId, Uop>,
    pub msgs: EntityVec<MsgId, Box<[u8]>>,
    pub anims: EntityVec<DmAnimId, DmAnim>,
    pub anim_frames: EntityVec<DmAnimFrameId, DmAnimFrame>,
    pub script_binds: EnumMap<ScriptBind, Option<ScriptPosId>>,
    pub cheats: Vec<Cheat>,
    pub effects: EnumMap<EffectBind, Option<Effect>>,

    pub sine_table: [i16; 0xa00],

    pub score_jackpot_init: Bcd,
    pub score_jackpot_incr: Bcd,
    pub score_mode_hit_incr: Bcd,
    pub score_mode_ramp_incr: Bcd,
    pub issue_ball_pos: (u16, u16),
    pub issue_ball_release_pos: (u16, u16),
}

impl Assets {
    pub fn load(file: impl AsRef<Path>, table: TableId) -> std::io::Result<Self> {
        let mut exe = MzExe::load(file, 0)?;
        assert_eq!(exe.code_byte(exe.ip + 0xe), 0xb8);
        let ds = exe.code_word(exe.ip + 0xf);
        exe.ds = ds;

        let (lights, dm_palette) = extract_lights(&exe, table);
        let attract_lights = extract_attract_lights(&exe, table);
        let light_binds = extract_light_binds(table);
        let main_board = extract_main_board(&exe, table);
        let occmaps = gfx::extract_occmaps(&exe, table);
        let spring = Image {
            data: extract_spring(&exe, table),
            cmap: main_board.cmap.clone(),
        };
        let ball = Image {
            data: extract_ball(&exe, table),
            cmap: main_board.cmap.clone(),
        };

        let physmaps = extract_physmaps(&exe, table);
        let physmap_patches = extract_physmap_patches(&exe, table, &physmaps);
        let sine_table = extract_sine_table(&exe, table);
        let dm_fonts = extract_dm_fonts(&exe, table);
        let dm_tower = if table == TableId::Table4 {
            Some(extract_dm_tower(&exe))
        } else {
            None
        };
        let flippers = extract_flippers(&exe, table, &main_board, &physmaps);
        let ramps = extract_ramps(&exe, table);
        let ball_outline = extract_ball_outline(&exe, table);
        let mut tmp = ball_outline.clone();
        tmp.sort_by_key(|x| x.angle);
        let ball_outline_by_angle = tmp.into_iter().map(|x| (x.x, x.y)).collect();

        let jingle_binds = extract_jingle_binds(&exe, table);
        let sfx_binds = extract_sfx_binds(&exe, table);
        let position_jingle_start = match table {
            TableId::Table1 => 0x06,
            TableId::Table2 => 0x0a,
            TableId::Table3 => 0x07,
            TableId::Table4 => 0x0a,
        };

        let (scripts, uop_by_addr, msgs, anims, anim_frames) = extract_scripts(&exe, table);
        let script_binds = extract_script_binds(table, &uop_by_addr);
        let cheats = extract_cheats(table, &uop_by_addr);
        let effects = extract_effects(&exe, table, &uop_by_addr);

        let score_jackpot_init = match table {
            TableId::Table1 => Bcd::from_ascii(b"10000000"),
            TableId::Table2 => Bcd::from_ascii(b"5000000"),
            TableId::Table3 => Bcd::from_ascii(b"10000000"),
            TableId::Table4 => Bcd::from_ascii(b"10000000"),
        };
        let score_jackpot_incr = match table {
            TableId::Table1 => Bcd::from_ascii(b"50000"),
            TableId::Table2 => Bcd::from_ascii(b"100000"),
            TableId::Table3 => Bcd::from_ascii(b"100000"),
            TableId::Table4 => Bcd::from_ascii(b"100000"),
        };

        let score_mode_hit_incr = match table {
            TableId::Table1 => Bcd::from_ascii(b"1000000"),
            TableId::Table2 => Bcd::from_ascii(b"100000"),
            TableId::Table3 => Bcd::from_ascii(b"500000"),
            TableId::Table4 => Bcd::from_ascii(b"1000000"),
        };
        let score_mode_ramp_incr = match table {
            TableId::Table1 => Bcd::from_ascii(b"5000000"),
            TableId::Table2 => Bcd::from_ascii(b"5000000"),
            TableId::Table3 => Bcd::from_ascii(b"1000000"),
            TableId::Table4 => Bcd::from_ascii(b"5000000"),
        };

        let issue_ball_pos = match table {
            TableId::Table1 => (282, 530),
            TableId::Table2 => (285, 530),
            TableId::Table3 => (284, 530),
            TableId::Table4 => (280, 525),
        };
        let issue_ball_release_pos = match table {
            TableId::Table1 => (297, 530),
            TableId::Table2 => (300, 530),
            TableId::Table3 => (299, 530),
            TableId::Table4 => (295, 525),
        };

        let (transitions_down, transitions_up) = extract_transitions(&exe, table);
        let bumpers = extract_bumpers(&exe, table);
        let (roll_triggers, roll_triggers_tilt) = extract_roll_triggers(&exe, table);
        let hit_triggers = extract_hit_triggers(&exe, table);

        Ok(Assets {
            table,
            exe,
            main_board,
            occmaps,
            spring,
            ball,
            physmaps,
            physmap_patches,
            ramps,
            ball_outline,
            ball_outline_by_angle,
            lights,
            attract_lights,
            light_binds,
            dm_palette,
            dm_fonts,
            dm_tower,
            flippers,
            transitions_down,
            transitions_up,
            bumpers,
            roll_triggers,
            roll_triggers_tilt,
            hit_triggers,
            jingle_binds,
            sfx_binds,
            position_jingle_start,
            scripts,
            msgs,
            anims,
            anim_frames,
            script_binds,
            cheats,
            effects,
            sine_table,
            score_jackpot_init,
            score_jackpot_incr,
            score_mode_hit_incr,
            score_mode_ramp_incr,
            issue_ball_pos,
            issue_ball_release_pos,
        })
    }
}
