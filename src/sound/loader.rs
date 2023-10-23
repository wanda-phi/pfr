use std::{array, num::NonZeroU8, str};

use arrayref::array_ref;

use super::{MiscEffect, Mod, Note, Sample, ToneEffect, VolumeEffect, PERIODS};

impl From<u32> for Note {
    fn from(value: u32) -> Self {
        let period = (value >> 16 & 0xfff) as u16;
        let mut period = if period == 0 {
            None
        } else {
            Some(PERIODS[0].iter().position(|&x| x == period).unwrap() as u8)
        };
        let sample = (value >> 24 & 0xf0 | value >> 12 & 0xf) as u8;
        let mut sample = if sample == 0 { None } else { Some(sample) };
        let effect = value & 0xfff;
        let effect_arg = (effect & 0xff) as u8;
        let effect_arg_hi = effect_arg >> 4 & 0xf;
        let effect_arg_lo = effect_arg & 0xf;
        let mut tone_effect = ToneEffect::None;
        let mut volume_effect = VolumeEffect::None;
        let mut misc_effect = MiscEffect::None;
        match effect >> 8 {
            0 => {
                // Arpeggio (or none)
                if effect != 0 {
                    tone_effect = ToneEffect::Arpeggio(effect_arg_hi, effect_arg_lo)
                }
            }
            1 => {
                // Portamento Up
                tone_effect = ToneEffect::Portamento {
                    target: Some(35),
                    speed: NonZeroU8::new(effect_arg),
                }
            }
            2 => {
                // Portamento Down
                tone_effect = ToneEffect::Portamento {
                    target: Some(0),
                    speed: NonZeroU8::new(effect_arg),
                }
            }
            3 => {
                // Tone Portamento
                tone_effect = ToneEffect::Portamento {
                    target: period,
                    speed: NonZeroU8::new(effect_arg),
                };
                if sample.is_some() {
                    volume_effect = VolumeEffect::Reset;
                }
                period = None;
                sample = None;
            }
            4 => {
                // Vibrato
                tone_effect = ToneEffect::Vibrato {
                    rate: NonZeroU8::new(effect_arg_hi),
                    depth: NonZeroU8::new(effect_arg_lo),
                }
            }
            5 => {
                // Tone Portamento + Volume Slide
                tone_effect = ToneEffect::Portamento {
                    target: period,
                    speed: None,
                };
                let speed = if effect_arg_hi != 0 {
                    effect_arg_hi as i8
                } else {
                    -(effect_arg_lo as i8)
                };
                volume_effect = VolumeEffect::VolumeSlide(speed);
                period = None;
                sample = None;
            }
            6 => {
                // Vibrato + Volume Slide
                tone_effect = ToneEffect::Vibrato {
                    rate: None,
                    depth: None,
                };
                let speed = if effect_arg_hi != 0 {
                    effect_arg_hi as i8
                } else {
                    -(effect_arg_lo as i8)
                };
                volume_effect = VolumeEffect::VolumeSlide(speed);
            }
            9 => misc_effect = MiscEffect::SetSampleOffset(effect_arg),
            0xa => {
                // Volume Slide
                let speed = if effect_arg_hi != 0 {
                    effect_arg_hi as i8
                } else {
                    -(effect_arg_lo as i8)
                };
                volume_effect = VolumeEffect::VolumeSlide(speed);
            }
            0xb => misc_effect = MiscEffect::PositionJump(effect_arg),
            0xc => volume_effect = VolumeEffect::SetVolume(effect_arg),
            0xd => misc_effect = MiscEffect::PatternBreak(effect_arg),
            0xe if effect_arg_hi == 9 => misc_effect = MiscEffect::RetrigNote(effect_arg_lo),
            0xf => misc_effect = MiscEffect::SetSpeed(effect_arg),
            _ => panic!("unknown effect {effect:03x}"),
        }
        Note {
            period,
            sample,
            tone_effect,
            volume_effect,
            misc_effect,
        }
    }
}

pub fn load(data: &[u8]) -> Mod {
    assert!(data.len() > 1080);
    let name = str::from_utf8(&data[..20])
        .unwrap()
        .trim_end_matches('\0')
        .to_string();
    let mut sample_lens = vec![0];
    let mut samples = vec![Sample {
        name: "".into(),
        data: vec![],
        finetune: 0,
        volume: 0,
        repeat: None,
    }];
    let mut pos = 20;
    for _ in 0..31 {
        let buf = &data[pos..pos + 30];
        pos += 30;
        sample_lens.push(u16::from_be_bytes(*array_ref![buf, 22, 2]) as usize * 2);
        assert_eq!(buf[24] & 0xf0, 0);
        let rep_pos = u16::from_be_bytes(*array_ref![buf, 26, 2]) as usize * 2;
        let rep_len = u16::from_be_bytes(*array_ref![buf, 28, 2]) as usize * 2;
        let repeat = if rep_pos == 0 && rep_len == 2 {
            None
        } else {
            Some((rep_pos, rep_len))
        };
        samples.push(Sample {
            name: str::from_utf8(&buf[..22])
                .unwrap()
                .trim_end_matches('\0')
                .to_string(),
            data: vec![],
            finetune: buf[24],
            volume: buf[25],
            repeat,
        });
    }
    let buf = &data[pos..pos + 134];
    pos += 134;
    let song_len = buf[0];
    let pos_restart = if buf[1] == 127 { 0 } else { buf[1] };
    assert!(song_len <= 128);
    assert!(song_len != 0);
    assert!(pos_restart < song_len);
    let positions = &buf[2..130];
    let num_patterns = positions.iter().copied().max().unwrap() as usize + 1;
    let positions = positions[..song_len as usize].to_vec();
    let mut patterns = vec![];
    for _ in 0..num_patterns {
        let buf = &data[pos..pos + 0x400];
        pos += 0x400;
        patterns.push(array::from_fn(|pat| {
            array::from_fn(|ch| {
                let pos = pat << 4 | ch << 2;
                Note::from(u32::from_be_bytes(*array_ref![buf, pos, 4]))
            })
        }));
    }
    for (sample, len) in samples.iter_mut().zip(sample_lens) {
        if len <= 2 {
            continue;
        }
        sample.data = data[pos..pos + len].to_vec();
        pos += len;
    }
    Mod {
        name,
        samples,
        patterns,
        positions,
        pos_restart,
    }
}
