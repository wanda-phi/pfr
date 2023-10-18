use enum_map::{Enum, EnumMap};
use ndarray::prelude::*;
use unnamed_entity::{entity_id, EntityVec};

use crate::{
    assets::{iff::Image, mz::MzExe, table::physics::extract_physmap_rect_patched_or},
    config::TableId,
};

use super::physics::{Layer, Rect};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum)]
pub enum FlipperSide {
    Left,
    Right,
}

entity_id! {
    pub id FlipperId u8;
}

#[derive(Clone, Debug)]
pub struct Flipper {
    pub side: FlipperSide,
    pub rect_pos: (u16, u16),
    pub physmap: Vec<Array2<u8>>,
    pub gfx: Vec<Array2<u8>>,
    pub ball_bbox: Rect,
    pub origin: (i16, i16),
    pub is_vertical: bool,
    pub quantum_max: u16,
    pub pos_max: i16,
    pub accel_press: i16,
    pub accel_release: i16,
    pub speed_press_start: i16,
}

pub(super) fn extract_flippers(
    exe: &MzExe,
    table: TableId,
    main_board: &Image,
    physmaps: &EnumMap<Layer, Array2<u8>>,
) -> EntityVec<FlipperId, Flipper> {
    let (off, physmap_segs, gfx_seg, glut_seg, glut_len) = match table {
        TableId::Table1 => (
            0x6950,
            &[0x4c54, 0x4fc2, 0x4eb6][..],
            0xc0f,
            0x8274,
            0x6d4 / 4,
        ),
        TableId::Table2 => (
            0x6940,
            &[0x4998, 0x4df2, 0x4bfa][..],
            0xb8e,
            0x7e3d,
            0xa8 / 4,
        ),
        TableId::Table3 => (
            0x66d0,
            &[0x3bcf, 0x3f2a, 0x3e31][..],
            0xb37,
            0x7aff,
            0xdc / 4,
        ),
        TableId::Table4 => (0x7360, &[0x453b, 0x479d][..], 0xc88, 0x7f47, 0x78 / 4),
    };
    physmap_segs
        .iter()
        .enumerate()
        .map(|(i, &physmap_seg)| {
            let foff = off + (i as u16) * 0x3c;
            let width = exe.data_word(foff + 0x06) * 16;
            let height = exe.data_word(foff + 0x08);
            let physmap_stride = exe.data_word(foff + 0x18) / 3;
            assert_eq!((width / 8) * height, physmap_stride);
            let quantum_max = exe.data_word(foff + 0x20);
            let rect_pos = (exe.data_word(foff + 0x02), exe.data_word(foff + 0x04));
            let mut gfx = vec![main_board
                .data
                .slice(s![
                    (rect_pos.0 as usize)..((rect_pos.0 + width) as usize),
                    (rect_pos.1 as usize)..((rect_pos.1 + height) as usize),
                ])
                .to_owned()];
            let copy_list_ptr = exe.data_word(foff + 0x36) + (i as u16) * 8;
            let copy_list_stride = exe.data_word(foff + 0x38);
            for i in 0..quantum_max {
                let mut image = gfx.last().unwrap().clone();
                let cloff = copy_list_ptr + copy_list_stride * i;
                let cnt = exe.word(gfx_seg, cloff);
                for j in 0..cnt {
                    let off = cloff + 0x12 + j * 4;
                    let dst = exe.word(gfx_seg, off);
                    let src = exe.word(gfx_seg, off + 2) - 0xd4f4;
                    let dx = (dst % 0x54) * 4;
                    let dy = dst / 0x54;
                    assert!((0..width).contains(&dx));
                    assert!((0..height).contains(&dy));
                    for k in 0..4 {
                        image[((dx + k) as usize, dy as usize)] =
                            exe.byte(glut_seg, src + glut_len * k);
                    }
                }
                gfx.push(image);
            }

            Flipper {
                side: match exe.data_byte(foff) {
                    1 => FlipperSide::Right,
                    2 => FlipperSide::Left,
                    _ => unreachable!(),
                },
                rect_pos,
                physmap: (0..=quantum_max)
                    .map(|q| {
                        extract_physmap_rect_patched_or(
                            exe,
                            physmaps,
                            Layer::Ground,
                            rect_pos,
                            width / 8,
                            height,
                            physmap_seg,
                            q * physmap_stride,
                        )
                    })
                    .collect(),
                gfx,
                ball_bbox: Rect {
                    xy_min: (exe.data_word_s(foff + 0x0a), exe.data_word_s(foff + 0x0e)),
                    xy_max: (exe.data_word_s(foff + 0x0c), exe.data_word_s(foff + 0x10)),
                },
                origin: (exe.data_word_s(foff + 0x12), exe.data_word_s(foff + 0x14)),
                is_vertical: match exe.data_word(foff + 0x16) {
                    0 => false,
                    0xffff => true,
                    _ => unreachable!(),
                },
                quantum_max,
                pos_max: exe.data_word_s(foff + 0x22),
                accel_press: -exe.data_word_s(foff + 0x24),
                accel_release: -exe.data_word_s(foff + 0x26),
                speed_press_start: -exe.data_word_s(foff + 0x28),
            }
        })
        .collect()
}
