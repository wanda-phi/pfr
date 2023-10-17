pub mod controller;
pub mod loader;
pub mod player;

use std::{fmt::Display, num::NonZeroU8};

pub const PERIODS: [[u16; 36]; 16] = [
    [
        856, 808, 762, 720, 678, 640, 604, 570, 538, 508, 480, 453, 428, 404, 381, 360, 339, 320,
        302, 285, 269, 254, 240, 226, 214, 202, 190, 180, 170, 160, 151, 143, 135, 127, 120, 113,
    ],
    [
        850, 802, 757, 715, 674, 637, 601, 567, 535, 505, 477, 450, 425, 401, 379, 357, 337, 318,
        300, 284, 268, 253, 239, 225, 213, 201, 189, 179, 169, 159, 150, 142, 134, 126, 119, 113,
    ],
    [
        844, 796, 752, 709, 670, 632, 597, 563, 532, 502, 474, 447, 422, 398, 376, 355, 335, 316,
        298, 282, 266, 251, 237, 224, 211, 199, 188, 177, 167, 158, 149, 141, 133, 125, 118, 112,
    ],
    [
        838, 791, 746, 704, 665, 628, 592, 559, 528, 498, 470, 444, 419, 395, 373, 352, 332, 314,
        296, 280, 264, 249, 235, 222, 209, 198, 187, 176, 166, 157, 148, 140, 132, 125, 118, 111,
    ],
    [
        832, 785, 741, 699, 660, 623, 588, 555, 524, 495, 467, 441, 416, 392, 370, 350, 330, 312,
        294, 278, 262, 247, 233, 220, 208, 196, 185, 175, 165, 156, 147, 139, 131, 124, 117, 110,
    ],
    [
        826, 779, 736, 694, 655, 619, 584, 551, 520, 491, 463, 437, 413, 390, 368, 347, 328, 309,
        292, 276, 260, 245, 232, 219, 206, 195, 184, 174, 164, 155, 146, 138, 130, 123, 116, 109,
    ],
    [
        820, 774, 730, 689, 651, 614, 580, 547, 516, 487, 460, 434, 410, 387, 365, 345, 325, 307,
        290, 274, 258, 244, 230, 217, 205, 193, 183, 172, 163, 154, 145, 137, 129, 122, 115, 109,
    ],
    [
        814, 768, 725, 684, 646, 610, 575, 543, 513, 484, 457, 431, 407, 384, 363, 342, 323, 305,
        288, 272, 256, 242, 228, 216, 204, 192, 181, 171, 161, 152, 144, 136, 128, 121, 114, 108,
    ],
    [
        907, 856, 808, 762, 720, 678, 640, 604, 570, 538, 504, 480, 453, 428, 404, 381, 360, 339,
        320, 302, 285, 269, 254, 240, 226, 214, 202, 190, 180, 170, 160, 151, 143, 135, 127, 120,
    ],
    [
        900, 850, 802, 757, 715, 675, 636, 601, 567, 535, 505, 477, 450, 425, 401, 379, 357, 337,
        318, 300, 284, 268, 253, 238, 225, 212, 200, 189, 179, 169, 159, 150, 142, 134, 126, 119,
    ],
    [
        894, 844, 796, 752, 709, 670, 632, 597, 563, 532, 502, 474, 447, 422, 398, 376, 355, 335,
        316, 298, 282, 266, 251, 237, 223, 211, 199, 188, 177, 167, 158, 149, 141, 133, 125, 118,
    ],
    [
        887, 838, 791, 746, 704, 665, 628, 592, 559, 528, 498, 470, 444, 419, 395, 373, 352, 332,
        314, 296, 280, 264, 249, 235, 222, 209, 198, 187, 176, 166, 157, 148, 140, 132, 125, 118,
    ],
    [
        881, 832, 785, 741, 699, 660, 623, 588, 555, 524, 494, 467, 441, 416, 392, 370, 350, 330,
        312, 294, 278, 262, 247, 233, 220, 208, 196, 185, 175, 165, 156, 147, 139, 131, 123, 117,
    ],
    [
        875, 826, 779, 736, 694, 655, 619, 584, 551, 520, 491, 463, 437, 413, 390, 368, 347, 338,
        309, 292, 276, 260, 245, 232, 219, 206, 195, 184, 174, 164, 155, 146, 138, 130, 123, 116,
    ],
    [
        868, 820, 774, 730, 689, 651, 614, 580, 547, 516, 487, 460, 434, 410, 387, 365, 345, 325,
        307, 290, 274, 258, 244, 230, 217, 205, 193, 183, 172, 163, 154, 145, 137, 129, 122, 115,
    ],
    [
        862, 814, 768, 725, 684, 646, 610, 575, 543, 513, 484, 457, 431, 407, 384, 363, 342, 323,
        305, 288, 272, 256, 242, 228, 216, 203, 192, 181, 171, 161, 152, 144, 136, 128, 121, 114,
    ],
];

#[rustfmt::skip]
pub const NAMES: [&str; 36] = [
    "C-1", "C#1", "D-1", "D#1", "E-1", "F-1", "F#1", "G-1", "G#1", "A-1", "A#1", "B-1",
    "C-2", "C#2", "D-2", "D#2", "E-2", "F-2", "F#2", "G-2", "G#2", "A-2", "A#2", "B-2",
    "C-3", "C#3", "D-3", "D#3", "E-3", "F-3", "F#3", "G-3", "G#3", "A-3", "A#3", "B-3",
];

pub struct Mod {
    pub name: String,
    pub samples: Vec<Sample>,
    pub patterns: Vec<Pattern>,
    pub positions: Vec<u8>,
    pub pos_restart: u8,
}

#[derive(Clone, Debug)]
pub struct Sample {
    pub name: String,
    pub data: Vec<u8>,
    pub finetune: u8,
    pub volume: u8,
    pub repeat: Option<(usize, usize)>,
}

pub type Pattern = [Row; 0x40];
pub type Row = [Note; 4];

#[derive(Copy, Clone, Debug)]
pub struct Note {
    pub period: Option<u8>,
    pub sample: Option<u8>,
    pub tone_effect: ToneEffect,
    pub volume_effect: VolumeEffect,
    pub misc_effect: MiscEffect,
}

#[derive(Copy, Clone, Debug)]
pub enum ToneEffect {
    None,
    Arpeggio(u8, u8),
    Portamento {
        target: Option<u8>,
        speed: Option<NonZeroU8>,
    },
    Vibrato {
        rate: Option<NonZeroU8>,
        depth: Option<NonZeroU8>,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum VolumeEffect {
    None,
    SetVolume(u8),
    VolumeSlide(i8),
    Reset,
}

#[derive(Copy, Clone, Debug)]
pub enum MiscEffect {
    None,
    SetSampleOffset(u8),
    PositionJump(u8),
    PatternBreak(u8),
    RetrigNote(u8),
    SetSpeed(u8),
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.period {
            Some(period) => write!(f, "{}", NAMES[period as usize])?,
            None => write!(f, "---")?,
        }
        match self.sample {
            Some(idx) => write!(f, " {:02x}", idx + 1)?,
            None => write!(f, " --")?,
        }
        match self.tone_effect {
            ToneEffect::None => match self.misc_effect {
                MiscEffect::None => write!(f, " ---- ---")?,
                MiscEffect::SetSampleOffset(x) => write!(f, " SO{x:02x} ---")?,
                MiscEffect::PositionJump(x) => write!(f, " PJ{x:02x} ---")?,
                MiscEffect::PatternBreak(x) => write!(f, " PB{x:02x} ---")?,
                MiscEffect::RetrigNote(x) => write!(f, " RN{x:02x} ---")?,
                MiscEffect::SetSpeed(x) => write!(f, " SS{x:02x} ---")?,
            },
            ToneEffect::Arpeggio(a, b) => write!(f, " Ar{a:x}{b:x} ---")?,
            ToneEffect::Portamento { target, speed } => {
                write!(f, " Po")?;
                match speed {
                    Some(s) => write!(f, "{s:02x}")?,
                    None => write!(f, "--")?,
                }
                match target {
                    Some(t) => write!(f, " {}", NAMES[t as usize])?,
                    None => write!(f, " ---")?,
                }
            }
            ToneEffect::Vibrato { rate, depth } => {
                write!(f, " Vi")?;
                match rate {
                    Some(x) => write!(f, "{x:x}")?,
                    None => write!(f, "-")?,
                }
                match depth {
                    Some(x) => write!(f, "{x:x}")?,
                    None => write!(f, "-")?,
                }
                write!(f, " ---")?;
            }
        }
        match self.volume_effect {
            VolumeEffect::None => write!(f, " ----")?,
            VolumeEffect::SetVolume(v) => write!(f, " Vo{v:02x}")?,
            VolumeEffect::VolumeSlide(v) => {
                if v < 0 {
                    write!(f, " VS-{:x}", -v)?
                } else {
                    write!(f, " VS+{v:x}")?
                }
            }
            VolumeEffect::Reset => write!(f, " VR--")?,
        }
        Ok(())
    }
}
