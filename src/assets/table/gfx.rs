use enum_map::{enum_map, EnumMap};
use ndarray::{concatenate, prelude::*};
use unnamed_entity::EntityVec;

use crate::{
    assets::{iff::Image, mz::MzExe},
    config::TableId,
};

use super::{dm::DmPalette, physics::Layer, Light, LightId};

pub(super) fn extract_main_board(exe: &MzExe, table: TableId) -> Image {
    let pbm_segs = match table {
        TableId::Table1 => [0x5224, 0x5947, 0x617b, 0x6a9c],
        TableId::Table2 => [0x5054, 0x5820, 0x5fe4, 0x6791],
        TableId::Table3 => [0x4c96, 0x5221, 0x5a4b, 0x632d],
        TableId::Table4 => [0x4ba1, 0x5480, 0x5d87, 0x66c2],
    };
    let pbms = pbm_segs.map(|x| Image::parse(exe.segment(x)));
    Image {
        data: concatenate!(
            Axis(1),
            pbms[0].data.slice(s![.., ..144]),
            pbms[1].data.slice(s![.., ..144]),
            pbms[2].data.slice(s![.., ..144]),
            pbms[3].data.slice(s![.., ..144]),
        ),
        cmap: pbms[3].cmap.clone(),
    }
}

pub(super) fn extract_lights(
    exe: &MzExe,
    table: TableId,
) -> (EntityVec<LightId, Light>, DmPalette) {
    fn fixup_color_a(color: (u8, u8, u8)) -> (u8, u8, u8) {
        (
            ((color.0 as u16 * 0xa2) >> 6) as u8,
            ((color.1 as u16 * 0xa2) >> 6) as u8,
            ((color.2 as u16 * 0xa2) >> 6) as u8,
        )
    }
    fn fixup_color_b(color: (u8, u8, u8)) -> (u8, u8, u8) {
        (
            color.0 << 2 | color.0 >> 4,
            color.1 << 2 | color.1 >> 4,
            color.2 << 2 | color.2 >> 4,
        )
    }

    let (table_off, num) = match table {
        TableId::Table1 => (0x12bd, 56),
        TableId::Table2 => (0xfdc, 67),
        TableId::Table3 => (0xd8b, 38),
        TableId::Table4 => (0x11d0, 44),
    };

    let lights = (0..num)
        .map(|i| {
            let off = exe.data_word(table_off + i * 2);
            let base_index = exe.data_byte(off);
            let cnt = exe.data_byte(off + 1) as u16;
            let mut colors = (0..cnt)
                .map(|i| {
                    fixup_color_a((
                        exe.data_byte(off + 2 + i * 3),
                        exe.data_byte(off + 2 + i * 3 + 1),
                        exe.data_byte(off + 2 + i * 3 + 2),
                    ))
                })
                .collect();
            if table == TableId::Table1 && i == 0x27 {
                colors = vec![];
            }
            Light { base_index, colors }
        })
        .collect();

    let (index_off, index_on) = match table {
        TableId::Table1 => (0x60, 0xf2),
        TableId::Table2 => (0x62, 0x80),
        TableId::Table3 => (0x72, 0x99),
        TableId::Table4 => (0xe7, 0x4f),
    };

    let dm0_off = table_off - 6;
    assert_eq!(exe.data_byte(dm0_off), index_on);
    assert_eq!(exe.data_byte(dm0_off + 1), 1);
    let color_on = fixup_color_a((
        exe.data_byte(dm0_off + 2),
        exe.data_byte(dm0_off + 3),
        exe.data_byte(dm0_off + 4),
    ));
    let dm1_off = table_off + num * 2;
    assert_eq!(exe.data_byte(dm1_off), index_on);
    assert_eq!(exe.data_byte(dm1_off + 1), 3);
    let color_off = fixup_color_b((
        exe.data_byte(dm1_off + 2),
        exe.data_byte(dm1_off + 3),
        exe.data_byte(dm1_off + 4),
    ));

    (
        lights,
        DmPalette {
            index_off,
            index_on,
            color_off,
            color_on,
        },
    )
}

pub(super) fn extract_occmaps(exe: &MzExe, table: TableId) -> EnumMap<Layer, Array2<u8>> {
    let seg = match table {
        TableId::Table1 => 0x2f94,
        TableId::Table2 => 0x2cd8,
        TableId::Table3 => 0x1f0f,
        TableId::Table4 => 0x287b,
    };
    let extract_layer = |off| {
        Array2::from_shape_fn((320, 576), |(x, y)| {
            let byte = exe.byte(seg, off + (x / 8 + y * 40) as u16);
            byte >> (7 - x % 8) & 1
        })
    };
    enum_map! {
        Layer::Ground => extract_layer(0x580),
        Layer::Overhead => extract_layer(0x6400),
    }
}

pub(super) fn extract_spring(exe: &MzExe, table: TableId) -> Array2<u8> {
    let spring_seg = match table {
        TableId::Table1 => 0x82e2,
        TableId::Table2 => 0x7e48,
        TableId::Table3 => 0x7b0d,
        TableId::Table4 => 0x7f4f,
    };
    let spring = exe.segment(spring_seg);
    Array2::from_shape_fn((10, 23), |(x, y)| spring[y * 10 + x])
}

pub(super) fn extract_ball(exe: &MzExe, table: TableId) -> Array2<u8> {
    let base = match table {
        TableId::Table1 => 0x95b0,
        TableId::Table2 => 0x8da0,
        TableId::Table3 => 0x8830,
        TableId::Table4 => 0x9d40,
    };
    let mut res = Array2::zeros((15, 15));
    let mut pos = base + 0x57;
    let mut plane = 0;
    let mut bbit = 0;
    loop {
        match exe.code_byte(pos) {
            0x26 => {
                assert_eq!(exe.code_byte(pos), 0x26);
                assert_eq!(exe.code_byte(pos + 1), 0x84);
                let boff = match exe.code_byte(pos + 2) {
                    0x27 => {
                        pos += 3;
                        0
                    }
                    0x67 => {
                        let x = exe.code_byte(pos + 3);
                        assert!(x < 0x80);
                        pos += 4;
                        x as u16
                    }
                    0xa7 => {
                        let x = exe.code_word(pos + 3);
                        pos += 5;
                        x
                    }
                    _ => unreachable!(),
                };
                assert_eq!(exe.code_byte(pos), 0x75);
                let jd = exe.code_byte(pos + 1);
                assert!(jd < 0x80);
                pos += 2;
                let jdst = pos + (jd as u16);
                assert_eq!(exe.code_byte(pos), 0x8a);
                let poff = match exe.code_byte(pos + 1) {
                    0x44 => {
                        let x = exe.code_byte(pos + 2);
                        assert!(x < 0x80);
                        pos += 3;
                        x as u16
                    }
                    0x84 => {
                        let x = exe.code_word(pos + 2);
                        pos += 4;
                        x
                    }
                    _ => unreachable!(),
                };
                assert_eq!(exe.code_byte(pos), 0xaa);
                pos += 1;
                assert_eq!(exe.code_byte(pos), 0xc6);
                let poff2 = match exe.code_byte(pos + 1) {
                    0x44 => {
                        let x = exe.code_byte(pos + 2);
                        assert!(x < 0x80);
                        pos += 3;
                        x as u16
                    }
                    0x84 => {
                        let x = exe.code_word(pos + 2);
                        pos += 4;
                        x
                    }
                    _ => unreachable!(),
                };
                let pix = exe.code_byte(pos);
                pos += 1;
                let py = poff / 84;
                let px = poff % 84 * 4 + plane;
                assert_eq!(poff, poff2);
                assert_eq!(bbit, px % 8);
                assert_eq!(boff, px / 8 + py * 42);
                res[(px as usize, py as usize)] = pix;
                assert_eq!(jdst, pos);
            }
            0xd0 if exe.code_byte(pos + 1) == 0xcc => {
                for _ in 0..4 {
                    assert_eq!(exe.code_bytes(pos, 5), [0xd0, 0xcc, 0x73, 0x01, 0x43]);
                    pos += 5;
                }
                bbit += 4;
                bbit %= 8;
            }
            0xd0 if exe.code_byte(pos + 1) == 0xc1 => {
                assert_eq!(
                    exe.code_bytes(pos, 0x2e),
                    [
                        0xd0, 0xc1, 0x83, 0xd6, 0x00, 0xfe, 0xc5, 0x80, 0xe5, 0x03, 0x50, 0x8a,
                        0xe5, 0xb0, 0x04, 0xba, 0xce, 0x03, 0xef, 0xba, 0xc4, 0x03, 0xb0, 0x02,
                        0x8a, 0xe1, 0x80, 0xe4, 0x0f, 0xef, 0x58, 0xd0, 0xc4, 0x73, 0x01, 0x4b,
                        0xd0, 0xc4, 0x73, 0x01, 0x4b, 0xd0, 0xc4, 0x73, 0x01, 0x4b,
                    ]
                );
                pos += 0x2e;
                plane += 1;
                bbit += 5;
                bbit %= 8;
            }
            0x5a => {
                assert_eq!(exe.code_bytes(pos, 3), [0x5a, 0x5e, 0xc3]);
                break;
            }
            x => panic!("ummm {x:02x} at {pos:04x}"),
        }
    }
    res
}
