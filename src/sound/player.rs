use std::{ops::Deref, sync::Arc};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, SampleRate, Stream, StreamConfig,
};

use super::{
    controller::{Controller, Sequencer, SimpleSequencer},
    MiscEffect, Mod, Note, PERIODS,
};

const VIBRATO_LUT: [u8; 32] = [
    0x00, 0x18, 0x31, 0x4a, 0x61, 0x78, 0x8d, 0xa1, 0xb4, 0xc5, 0xd4, 0xe0, 0xeb, 0xf4, 0xfa, 0xfd,
    0xff, 0xfd, 0xfa, 0xf4, 0xeb, 0xe0, 0xd4, 0xc5, 0xb4, 0xa1, 0x8d, 0x78, 0x61, 0x4a, 0x31, 0x18,
];

struct PlayerState {
    module: Mod,
    sequencer: Arc<dyn Sequencer>,
    controller: Arc<Controller>,
    sample_rate: u32,
    speed: u8,
    ticks_left: u8,
    samples_left: u32,
    samples_in_tick: u32,
    position: usize,
    row: usize,
    channels: [ChannelState; 4],
    pattern_break: Option<u8>,
    jump: Option<u8>,
}

enum ChannelToneEffect {
    None,
    Portamento,
    Vibrato,
    Arpeggio,
    Retrig,
}

enum ChannelVolumeEffect {
    None,
    Slide,
}

struct ChannelState {
    volume: u8,
    sample: usize,
    sample_pos: u64,
    sample_bytes_per_frame: u64,
    sample_pos_reload: u64,
    xperiod: u8,
    period: u16,
    tone_effect: ChannelToneEffect,
    arpeggio_periods: [u16; 2],
    portamento_target: u16,
    portamento_speed: u8,
    vibrato_phase: u8,
    vibrato_rate: u8,
    vibrato_depth: u8,
    volume_effect: ChannelVolumeEffect,
    volume_slide_speed: i8,
    retrig_period: u8,
    retrig_left: u8,
}

pub struct Player {
    _stream: Stream,
    controller: Arc<Controller>,
}

impl Deref for Player {
    type Target = Controller;

    fn deref(&self) -> &Self::Target {
        &self.controller
    }
}

pub fn play(module: Mod, sequencer: Option<Arc<dyn Sequencer>>) -> Player {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    /*let supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    for cfg in supported_configs_range {
        println!("{cfg:#?}");
    }*/
    let sample_rate = 48000;
    let sequencer = sequencer.unwrap_or_else(|| Arc::new(SimpleSequencer::new(&module)));
    let position = sequencer.next_position() as usize;
    let controller = Arc::new(Controller::new());
    let mut state = PlayerState {
        module,
        speed: 6,
        ticks_left: 0,
        samples_left: 0,
        sequencer,
        controller: controller.clone(),
        samples_in_tick: sample_rate / 50,
        position,
        row: 0,
        channels: std::array::from_fn(|_| ChannelState {
            volume: 0x40,
            sample: 0,
            sample_pos: 0,
            sample_bytes_per_frame: 0,
            sample_pos_reload: 0,
            period: 0,
            vibrato_phase: 0,
            tone_effect: ChannelToneEffect::None,
            arpeggio_periods: [0, 0],
            portamento_target: 0,
            portamento_speed: 0,
            vibrato_rate: 0,
            vibrato_depth: 0,
            volume_effect: ChannelVolumeEffect::None,
            volume_slide_speed: 0,
            retrig_period: 0,
            retrig_left: 0,
            xperiod: 0,
        }),
        sample_rate,
        pattern_break: None,
        jump: None,
    };
    
    let config = StreamConfig {
        channels: 2,
        sample_rate: SampleRate(sample_rate),
        
        #[cfg(target_arch = "wasm32")]
        buffer_size: BufferSize::Fixed(sample_rate / 32),
        
        #[cfg(not(target_arch = "wasm32"))]
        buffer_size: BufferSize::Fixed(sample_rate / 50),
    };
    
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| state.make_samples(data),
            move |err| eprintln!("audio error: {err:?}"),
            None, // None=blocking, Some(Duration)=timeout
        )
        .expect("failed to make stream");
    stream.play().unwrap();
    Player {
        _stream: stream,
        controller,
    }
}

impl PlayerState {
    fn make_samples(&mut self, data: &mut [f32]) {
        if self.controller.paused() {
            for v in data {
                *v = 0.0;
            }
            return;
        }
        let master_volume = self.controller.master_volume() as i32;
        self.process_interrupt();
        if let Some((channel, note)) = self.controller.get_sfx() {
            self.play_note(channel, note);
        }
        let mut pos = 0;
        while pos < data.len() {
            if self.samples_left == 0 {
                if self.ticks_left == 0 {
                    self.play_row();
                    self.ticks_left = self.speed - 1;
                } else {
                    self.ticks_left -= 1;
                    self.play_effects();
                }
                self.samples_left = self.samples_in_tick;
                self.controller.incr_tick();
            }
            data[pos] = ((self.play_channel(0) + self.play_channel(1)) / 0x100 * master_volume)
                as f32
                / (0x80000000u32 as f32);
            data[pos + 1] = ((self.play_channel(2) + self.play_channel(3)) / 0x100 * master_volume)
                as f32
                / (0x80000000u32 as f32);
            pos += 2;
            self.samples_left -= 1;
        }
    }

    fn process_interrupt(&mut self) {
        if let Some(position) = self.sequencer.check_interrupt() {
            self.position = position as usize;
            self.row = 0;
            self.ticks_left = 0;
            self.samples_left = 0;
        }
    }

    fn play_row(&mut self) {
        let pattern = self.module.positions[self.position] as usize;
        let row = self.module.patterns[pattern][self.row];
        // print!(
        //     "{pos:02x}/{pattern:02x}.{r:02x}",
        //     pos = self.position,
        //     r = self.row
        // );
        for (i, &note) in row.iter().enumerate() {
            self.play_note(i, note);
            // print!("   {note}");
        }
        // println!();
        if let Some(pos) = self.jump {
            // println!("---JUMP---");
            self.position = pos as usize;
            self.row = 0;
            self.jump = None;
        } else if let Some(row) = self.pattern_break {
            // println!("---BREAK---");
            self.row = row as usize;
            self.position = self.sequencer.next_position() as usize;
            self.pattern_break = None;
        } else {
            self.row += 1;
            if self.row == 0x40 {
                // println!("---");
                self.row = 0;
                self.position = self.sequencer.next_position() as usize;
            }
        }
    }

    fn play_note(&mut self, cidx: usize, note: Note) {
        let channel = &mut self.channels[cidx];
        if let Some(sidx) = note.sample {
            channel.sample = sidx as usize;
            channel.sample_pos_reload = 0;
            let sample = &self.module.samples[channel.sample];
            channel.volume = sample.volume;
        }
        let sample = &self.module.samples[channel.sample];
        if let Some(xperiod) = note.period {
            let period = PERIODS[sample.finetune as usize][xperiod as usize];
            channel.xperiod = xperiod;
            channel.period = period;
            channel.sample_pos = channel.sample_pos_reload;
            channel.vibrato_phase = 0;
            let byte_len = 0x361f0f / (period as u32);
            channel.sample_bytes_per_frame = ((byte_len as u64) << 32) / (self.sample_rate as u64);
        }
        match note.tone_effect {
            super::ToneEffect::None => channel.tone_effect = ChannelToneEffect::None,
            super::ToneEffect::Arpeggio(a, b) => {
                channel.tone_effect = ChannelToneEffect::Arpeggio;
                channel.arpeggio_periods = [
                    PERIODS[sample.finetune as usize][(channel.xperiod + a).min(35) as usize],
                    PERIODS[sample.finetune as usize][(channel.xperiod + b).min(35) as usize],
                ];
            }
            super::ToneEffect::Portamento { target, speed } => {
                channel.tone_effect = ChannelToneEffect::Portamento;
                if let Some(v) = target {
                    channel.portamento_target = PERIODS[sample.finetune as usize][v as usize];
                }
                if let Some(v) = speed {
                    channel.portamento_speed = v.into();
                }
            }
            super::ToneEffect::Vibrato { rate, depth } => {
                channel.tone_effect = ChannelToneEffect::Vibrato;
                if let Some(v) = rate {
                    channel.vibrato_rate = v.get() * 4;
                }
                if let Some(v) = depth {
                    channel.vibrato_depth = v.get();
                }
            }
        }
        match note.volume_effect {
            super::VolumeEffect::None => channel.volume_effect = ChannelVolumeEffect::None,
            super::VolumeEffect::SetVolume(v) => {
                channel.volume_effect = ChannelVolumeEffect::None;
                channel.volume = v;
            }
            super::VolumeEffect::VolumeSlide(s) => {
                channel.volume_effect = ChannelVolumeEffect::Slide;
                channel.volume_slide_speed = s;
            }
            super::VolumeEffect::Reset => {
                channel.volume_effect = ChannelVolumeEffect::None;
                channel.volume = sample.volume;
            }
        }
        match note.misc_effect {
            MiscEffect::None => {}
            MiscEffect::SetSampleOffset(off) => {
                channel.sample_pos_reload = (off as u64) << 40;
                if note.sample.is_some() {
                    channel.sample_pos = channel.sample_pos_reload;
                }
            }
            MiscEffect::PositionJump(pos) => {
                self.jump = Some(self.sequencer.jump(pos));
            }
            MiscEffect::PatternBreak(x) => {
                self.pattern_break = Some(x);
            }
            MiscEffect::RetrigNote(x) => {
                channel.tone_effect = ChannelToneEffect::Retrig;
                channel.retrig_period = x;
                channel.retrig_left = x - 1;
            }
            MiscEffect::SetSpeed(s) => {
                self.speed = s;
                self.ticks_left = s - 1;
            }
        }
    }

    fn play_effects(&mut self) {
        for channel in &mut self.channels {
            match channel.tone_effect {
                ChannelToneEffect::None => {}
                ChannelToneEffect::Arpeggio => {
                    let tmp = channel.period;
                    channel.period = channel.arpeggio_periods[1];
                    channel.arpeggio_periods[1] = channel.arpeggio_periods[0];
                    channel.arpeggio_periods[0] = tmp;
                    let byte_len = 0x361f0f / (channel.period as u32);
                    channel.sample_bytes_per_frame =
                        ((byte_len as u64) << 32) / (self.sample_rate as u64);
                }
                ChannelToneEffect::Portamento => {
                    // println!("PORTAMENTO!");
                    if channel.portamento_target != 0 {
                        if channel.portamento_target < channel.period {
                            channel.period -= channel.portamento_speed as u16;
                            if channel.period < channel.portamento_target {
                                channel.period = channel.portamento_target;
                            }
                        } else {
                            channel.period += channel.portamento_speed as u16;
                            if channel.period > channel.portamento_target {
                                channel.period = channel.portamento_target;
                            }
                        }
                        let byte_len = 0x361f0f / (channel.period as u32);
                        channel.sample_bytes_per_frame =
                            ((byte_len as u64) << 32) / (self.sample_rate as u64);
                    }
                }
                ChannelToneEffect::Vibrato => {
                    let phase = channel.vibrato_phase;
                    channel.vibrato_phase = phase.wrapping_add(channel.vibrato_rate);
                    let mut delta = VIBRATO_LUT[(phase >> 2 & 0x1f) as usize] as i16;
                    delta *= channel.vibrato_depth as i16;
                    delta >>= 7;
                    if phase & 0x80 != 0 {
                        delta *= -1;
                    }
                    // println!("VIBRATO {delta}");
                    let period = channel.period.wrapping_add_signed(delta);
                    if period != 0 {
                        let byte_len = 0x361f0f / (period as u32);
                        channel.sample_bytes_per_frame =
                            ((byte_len as u64) << 32) / (self.sample_rate as u64);
                    }
                }
                ChannelToneEffect::Retrig => {
                    if channel.retrig_left == 0 {
                        channel.retrig_left = channel.retrig_period - 1;
                        channel.sample_pos = 0;
                    } else {
                        channel.retrig_left -= 1;
                    }
                }
            }
            match channel.volume_effect {
                ChannelVolumeEffect::None => {}
                ChannelVolumeEffect::Slide => {
                    channel.volume = channel
                        .volume
                        .saturating_add_signed(channel.volume_slide_speed);
                    if channel.volume > 0x40 {
                        channel.volume = 0x40;
                    }
                }
            }
        }
    }

    fn play_channel(&mut self, idx: usize) -> i32 {
        let channel = &mut self.channels[idx];
        let sample = &self.module.samples[channel.sample];
        let mut pos = (channel.sample_pos >> 32) as usize;
        if let Some((rs, rl)) = sample.repeat {
            while pos >= rs + rl {
                pos -= rl;
                channel.sample_pos -= (rl as u64) << 32;
            }
        } else if pos >= sample.data.len() {
            return 0;
        }
        channel.sample_pos += channel.sample_bytes_per_frame;
        let mut val = sample.data[pos] as i32;
        if val >= 0x80 {
            val -= 0x100;
        }
        val <<= 16;
        val *= channel.volume as i32;
        val
    }
}
