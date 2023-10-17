use std::collections::HashMap;

use arrayvec::ArrayVec;
use enum_map::{enum_map, Enum, EnumMap};

use crate::{assets::mz::MzExe, config::TableId};

#[derive(Clone, Debug)]
pub struct DmPalette {
    pub index_off: u8,
    pub index_on: u8,
    pub color_off: (u8, u8, u8),
    pub color_on: (u8, u8, u8),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum)]
pub enum DmFont {
    H5,
    H8,
    H11,
    H13,
}

impl DmFont {
    pub fn height(self) -> usize {
        match self {
            DmFont::H5 => 5,
            DmFont::H8 => 8,
            DmFont::H11 => 11,
            DmFont::H13 => 13,
        }
    }
}

pub(super) fn extract_dm_fonts(
    exe: &MzExe,
    table: TableId,
) -> EnumMap<DmFont, HashMap<u8, ArrayVec<u8, 13>>> {
    let res = enum_map! {
        font => {
            let off = match (table, font) {
                (TableId::Table1, DmFont::H5) => 0x6710,
                (TableId::Table1, DmFont::H8) => 0x65d0,
                (TableId::Table1, DmFont::H11) => 0x6410,
                (TableId::Table1, DmFont::H13) => 0x6200,
                (TableId::Table2, DmFont::H5) => 0x67a0,
                (TableId::Table2, DmFont::H8) => 0x6660,
                (TableId::Table2, DmFont::H11) => 0x64a0,
                (TableId::Table2, DmFont::H13) => 0x6290,
                (TableId::Table3, DmFont::H5) => 0x5ff0,
                (TableId::Table3, DmFont::H8) => 0x5eb0,
                (TableId::Table3, DmFont::H11) => 0x5cf0,
                (TableId::Table3, DmFont::H13) => 0x5ae0,
                (TableId::Table4, DmFont::H5) => 0x6d00,
                (TableId::Table4, DmFont::H8) => 0x6bc0,
                (TableId::Table4, DmFont::H11) => 0x6a00,
                (TableId::Table4, DmFont::H13) => 0x67f0,
            };
            let mut res = HashMap::new();
            for (i, &chr) in b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ?()-".iter().enumerate() {
                res.insert(chr, exe.data_bytes(off + (i * font.height()) as u16, font.height()).iter().copied().collect());
            }
            res.insert(b'_', core::iter::repeat(0).take(font.height()).collect());
            res
        }
    };
    extract_num_fonts(exe, table, &res[DmFont::H13]);
    extract_long_fonts(exe, table, &res[DmFont::H13]);
    res
}

fn extract_num_fonts(exe: &MzExe, table: TableId, font: &HashMap<u8, ArrayVec<u8, 13>>) {
    let seg = match table {
        TableId::Table1 => 0xac7,
        TableId::Table2 => 0xa46,
        TableId::Table3 => 0x9ef,
        TableId::Table4 => 0xb40,
    };
    let mut pos0 = 0x1a0;
    let mut pos1 = 0xc40;
    for i in 0..0xff {
        let mut chr = [0; 13];
        for (pos, dx) in [(&mut pos0, 0), (&mut pos1, 1)] {
            while exe.byte(seg, *pos) != 0xc3 {
                assert_eq!(exe.byte(seg, *pos), 0x88);
                let val = match exe.byte(seg, *pos + 1) {
                    0xa7 => false,
                    0x87 => true,
                    _ => unreachable!(),
                };
                let off = exe.word(seg, *pos + 2);
                let x = off % 0xa8;
                let y = off / 0xa8;
                assert!(x < 4);
                assert!(y < 14);
                if y == 0 {
                    assert!(!val);
                } else if val {
                    chr[usize::from(y - 1)] |= 0x80 >> (x * 2 + dx);
                }
                *pos += 4;
            }
            *pos += 1;
        }
        let expected: &[u8] = match i {
            0x2f => &font[&b'_'],
            0x30..=0x39 => &font[&i],
            _ => &[0; 13],
        };
        assert_eq!(expected, &chr);
    }
}

fn extract_long_fonts(exe: &MzExe, table: TableId, font: &HashMap<u8, ArrayVec<u8, 13>>) {
    let (mut pos0, mut pos1) = match table {
        TableId::Table1 => (0x7c10, 0x70a0),
        TableId::Table2 => (0x7400, 0x6890),
        TableId::Table3 => (0x6e90, 0x6320),
        TableId::Table4 => (0x83a0, 0x7830),
    };
    for i in 0..0xff {
        let mut chr1 = [0; 16];
        let mut chr0 = [0; 16];
        for (pos, dx) in [(&mut pos0, 0), (&mut pos1, 1)] {
            while exe.code_byte(*pos) != 0xc3 {
                assert_eq!(exe.code_byte(*pos), 0x88);
                let val = match exe.code_byte(*pos + 1) {
                    0xa7 => false,
                    0x87 => true,
                    _ => unreachable!(),
                };
                let off = exe.code_word(*pos + 2);
                let x = off % 0xa8;
                let y = off / 0xa8;
                assert!(x < 4);
                if val {
                    chr1[usize::from(y)] |= 0x80 >> (x * 2 + dx);
                } else {
                    chr0[usize::from(y)] |= 0x80 >> (x * 2 + dx);
                }
                *pos += 4;
            }
            *pos += 1;
        }
        let mut chr = [0; 16];
        for y in 0..16 {
            let mut state = false;
            for x in 0..8 {
                if chr1[y] & 0x80 >> x != 0 {
                    state = true;
                } else if chr0[y] & 0x80 >> x != 0 {
                    state = false;
                }
                if state {
                    chr[y] |= 0x80 >> x;
                }
            }
            assert!(!state);
        }
        if chr == [0; 16] {
            continue;
        }
        assert_eq!(chr[0], 0);
        assert_eq!(chr[14], 0);
        assert_eq!(chr[15], 0);
        let c = match i {
            b'0'..=b'9' | b'A'..=b'Z' => i,
            0x5b => b'?',
            0x5c => b'(',
            0x5d => b')',
            0x5e => b'-',
            _ => unreachable!(),
        };
        assert_eq!(&font[&c], &chr[1..14]);
    }
}

pub fn extract_dm_tower(exe: &MzExe) -> Box<[[bool; 160]; 167]> {
    let mut res = Box::new([[false; 160]; 167]);
    let seg = 0x49ff;
    for y in 0..167 {
        for x in 0..160 {
            let byte = exe.byte(seg, y * 40 + x / 4);
            res[y as usize][x as usize] = (byte << (2 * (x % 4)) & 0x80) != 0;
        }
    }
    res
}
