use arrayref::array_ref;

use crate::bcd::Bcd;

#[derive(Clone, Debug)]
pub struct MzExe {
    pub image: Vec<u8>,
    pub relocs: Vec<FarPtr>,
    pub cs: u16,
    pub ip: u16,
    pub ss: u16,
    pub sp: u16,
    pub ds: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct FarPtr {
    pub seg: u16,
    pub off: u16,
}

impl MzExe {
    pub fn load(data: &[u8], ds: u16) -> Self {
        assert_eq!(&data[..2], b"MZ");
        let sz_last = u16::from_le_bytes(*array_ref![data, 2, 2]) as usize;
        let sz_pages = u16::from_le_bytes(*array_ref![data, 4, 2]) as usize;
        let image_sz = (sz_pages - 1) * 0x200 + sz_last;
        let num_relocs = u16::from_le_bytes(*array_ref![data, 6, 2]) as usize;
        let header_sz = u16::from_le_bytes(*array_ref![data, 8, 2]) as usize * 0x10;
        let image = data[header_sz..image_sz].to_vec();
        let ss = u16::from_le_bytes(*array_ref![data, 0xe, 2]);
        let sp = u16::from_le_bytes(*array_ref![data, 0x10, 2]);
        let ip = u16::from_le_bytes(*array_ref![data, 0x14, 2]);
        let cs = u16::from_le_bytes(*array_ref![data, 0x16, 2]);
        let reloc_base = u16::from_le_bytes(*array_ref![data, 0x18, 2]) as usize;
        let relocs = (0..num_relocs)
            .map(|i| {
                let off = reloc_base + i * 4;
                FarPtr {
                    off: u16::from_le_bytes(*array_ref![data, off, 2]),
                    seg: u16::from_le_bytes(*array_ref![data, off + 2, 2]),
                }
            })
            .collect();
        MzExe {
            image,
            relocs,
            cs,
            ip,
            ss,
            sp,
            ds,
        }
    }

    pub fn segment(&self, seg: u16) -> &[u8] {
        let off = seg as usize * 0x10;
        &self.image[off..]
    }

    pub fn byte(&self, seg: u16, off: u16) -> u8 {
        self.segment(seg)[off as usize]
    }

    pub fn bytes(&self, seg: u16, off: u16, num: usize) -> &[u8] {
        &self.segment(seg)[off as usize..off as usize + num]
    }

    pub fn word(&self, seg: u16, off: u16) -> u16 {
        u16::from_le_bytes(*array_ref![self.segment(seg), off as usize, 2])
    }

    pub fn word_s(&self, seg: u16, off: u16) -> i16 {
        i16::from_le_bytes(*array_ref![self.segment(seg), off as usize, 2])
    }

    pub fn data_bytes(&self, off: u16, num: usize) -> &[u8] {
        self.bytes(self.ds, off, num)
    }

    pub fn data_byte(&self, off: u16) -> u8 {
        self.byte(self.ds, off)
    }

    pub fn data_word(&self, off: u16) -> u16 {
        self.word(self.ds, off)
    }

    pub fn data_word_s(&self, off: u16) -> i16 {
        self.word_s(self.ds, off)
    }

    pub fn data_bcd(&self, off: u16) -> Bcd {
        Bcd::from_bytes(*array_ref![self.data_bytes(off, 12), 0, 12])
    }

    pub fn code_bytes(&self, off: u16, num: usize) -> &[u8] {
        self.bytes(self.cs, off, num)
    }

    pub fn code_byte(&self, off: u16) -> u8 {
        self.byte(self.cs, off)
    }

    pub fn code_word(&self, off: u16) -> u16 {
        self.word(self.cs, off)
    }
}
