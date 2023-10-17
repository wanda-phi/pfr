use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};

use crate::assets::table::sound::{Jingle, Sfx};

use super::{MiscEffect, Mod, Note, ToneEffect, VolumeEffect};

pub struct Controller {
    ticks: AtomicU32,
    volume: AtomicU32,
    sfx: AtomicU32,
    paused: AtomicBool,
}

impl Controller {
    pub(super) fn new() -> Self {
        Self {
            ticks: AtomicU32::new(0),
            sfx: AtomicU32::new(0),
            volume: AtomicU32::new(0x100),
            paused: AtomicBool::new(false),
        }
    }

    pub(super) fn incr_tick(&self) {
        let ticks = self.ticks.load(Ordering::Relaxed);
        self.ticks.store(ticks + 1, Ordering::Release);
    }

    pub fn ticks(&self) -> u32 {
        self.ticks.load(Ordering::Acquire)
    }

    pub fn set_master_volume(&self, volume: u32) {
        assert!(volume <= 0x100);
        self.volume.store(volume, Ordering::Relaxed);
    }

    pub fn master_volume(&self) -> u32 {
        self.volume.load(Ordering::Relaxed)
    }

    pub fn pause(&self) {
        self.paused.store(true, Ordering::Relaxed);
    }

    pub fn unpause(&self) {
        self.paused.store(false, Ordering::Relaxed);
    }

    pub fn paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    pub fn play_sfx(&self, sfx: Sfx, volume: u8) {
        let val = (sfx.period as u32)
            | (sfx.sample as u32) << 8
            | (volume as u32) << 16
            | (sfx.channel as u32) << 24;
        self.sfx.store(val, Ordering::Relaxed);
    }

    pub(super) fn get_sfx(&self) -> Option<(usize, Note)> {
        let sfx = self.sfx.swap(0, Ordering::Relaxed);
        if sfx != 0 {
            let volume = (sfx >> 16 & 0xff) as u8;
            let channel = (sfx >> 24 & 0xff) as usize;
            Some((
                channel,
                Note {
                    period: Some((sfx & 0xff) as u8),
                    sample: Some((sfx >> 8 & 0xff) as u8),
                    tone_effect: ToneEffect::None,
                    volume_effect: if volume == 0 {
                        VolumeEffect::None
                    } else {
                        VolumeEffect::SetVolume(volume)
                    },
                    misc_effect: MiscEffect::None,
                },
            ))
        } else {
            None
        }
    }
}

pub trait Sequencer: Sync + Send {
    fn check_interrupt(&self) -> Option<u8>;
    fn next_position(&self) -> u8;
    fn jump(&self, target: u8) -> u8;
}

pub struct SimpleSequencer {
    position: AtomicU8,
    wrap: u8,
}

impl SimpleSequencer {
    pub fn new(module: &Mod) -> Self {
        Self {
            position: AtomicU8::new(0),
            wrap: module.positions.len() as u8,
        }
    }
}

impl Sequencer for SimpleSequencer {
    fn check_interrupt(&self) -> Option<u8> {
        None
    }

    fn next_position(&self) -> u8 {
        let next = self.position.load(Ordering::Relaxed);
        if next + 1 == self.wrap {
            self.position.store(0, Ordering::Relaxed)
        } else {
            self.position.store(next + 1, Ordering::Relaxed)
        }
        next
    }

    fn jump(&self, target: u8) -> u8 {
        self.position.store(target, Ordering::Relaxed);
        self.next_position()
    }
}

pub struct TableSequencer {
    state: AtomicU32,
    position_jingle_start: u8,
    position_silence: u8,
}

#[derive(Copy, Clone, Debug)]
struct State {
    position: u8,
    interrupt: bool,
    repeat: u8,
    priority: u8,
    music: u8,
    no_music: bool,
}

impl From<u32> for State {
    fn from(value: u32) -> Self {
        State {
            position: (value & 0x7f) as u8,
            interrupt: (value & 0x80) != 0,
            repeat: (value >> 8 & 0xff) as u8,
            priority: (value >> 16 & 0xff) as u8,
            music: (value >> 24 & 0x7f) as u8,
            no_music: (value & 0x80000000) != 0,
        }
    }
}

impl From<State> for u32 {
    fn from(value: State) -> Self {
        assert!(value.position < 0x80);
        assert!(value.music < 0x80);
        u32::from(value.position)
            | u32::from(value.interrupt) << 7
            | u32::from(value.repeat) << 8
            | u32::from(value.priority) << 16
            | u32::from(value.music) << 24
            | u32::from(value.no_music) << 31
    }
}

impl TableSequencer {
    pub fn new(
        position: u8,
        position_jingle_start: u8,
        position_silence: u8,
        no_music: bool,
    ) -> Self {
        Self {
            state: AtomicU32::new(
                State {
                    position,
                    interrupt: true,
                    repeat: 0,
                    priority: 0,
                    music: position,
                    no_music,
                }
                .into(),
            ),
            position_jingle_start,
            position_silence,
        }
    }

    pub fn play_jingle(&self, jingle: Jingle, force: bool, music: Option<u8>) -> bool {
        assert!(jingle.position < 0x80);
        let mut val = self.state.load(Ordering::Acquire);
        loop {
            let mut state = State::from(val);
            if jingle.priority < state.priority && !force {
                return false;
            }
            if state.repeat == 0 {
                state.music = state.position;
            }
            state.position = jingle.position;
            state.interrupt = true;
            state.repeat = jingle.repeat;
            state.priority = jingle.priority;
            if let Some(music) = music {
                assert!(music < 0x80);
                state.music = music;
            }
            match self.state.compare_exchange(
                val,
                state.into(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(x) => val = x,
            }
        }
    }

    pub fn set_music(&self, position: u8) {
        assert!(position < 0x80);
        let mut val = self.state.load(Ordering::Acquire);
        loop {
            let mut state = State::from(val);
            state.music = position;
            match self.state.compare_exchange(
                val,
                state.into(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(x) => val = x,
            }
        }
    }

    pub fn reset_priority(&self) {
        let mut val = self.state.load(Ordering::Acquire);
        loop {
            let mut state = State::from(val);
            state.priority = 0;
            match self.state.compare_exchange(
                val,
                state.into(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(x) => val = x,
            }
        }
    }

    pub fn set_no_music(&self, flag: bool) {
        let mut val = self.state.load(Ordering::Acquire);
        loop {
            let mut state = State::from(val);
            state.no_music = flag;
            match self.state.compare_exchange(
                val,
                state.into(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(x) => val = x,
            }
        }
    }

    pub fn force_end_loop(&self) {
        let mut val = self.state.load(Ordering::Acquire);
        loop {
            let mut state = State::from(val);
            if state.repeat != 0 {
                return;
            }
            state.repeat = 1;
            match self.state.compare_exchange(
                val,
                state.into(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(x) => val = x,
            }
        }
    }

    pub fn music(&self) -> u8 {
        State::from(self.state.load(Ordering::Acquire)).music
    }

    pub fn priority(&self) -> u8 {
        State::from(self.state.load(Ordering::Acquire)).priority
    }

    pub fn jingle_playing(&self) -> bool {
        State::from(self.state.load(Ordering::Acquire)).repeat != 0
    }
}

impl Sequencer for TableSequencer {
    fn check_interrupt(&self) -> Option<u8> {
        let mut val = self.state.load(Ordering::Acquire);
        loop {
            let mut state = State::from(val);
            if !state.interrupt {
                return None;
            }
            state.interrupt = false;
            match self.state.compare_exchange(
                val,
                state.into(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(state.position),
                Err(x) => val = x,
            }
        }
    }

    fn next_position(&self) -> u8 {
        let mut val = self.state.load(Ordering::Acquire);
        loop {
            let mut state = State::from(val);
            if state.interrupt {
                // doesn't matter, the interrupt will override everything anyway.
                return state.position;
            }
            state.position += 1;
            match self.state.compare_exchange(
                val,
                state.into(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => return state.position,
                Err(x) => val = x,
            }
        }
    }

    fn jump(&self, mut target: u8) -> u8 {
        let mut val = self.state.load(Ordering::Acquire);
        loop {
            let mut state = State::from(val);
            if state.interrupt {
                // doesn't matter, the interrupt will override everything anyway.
                break;
            }
            match state.repeat {
                // nothing to worry about, just jump
                0 => (),
                1 => {
                    // repeat ran out, jump to music instead
                    state.priority = 0;
                    state.repeat = 0;
                    target = state.music;
                }
                // decrease repeat count
                _ => state.repeat -= 1,
            }
            if target < self.position_jingle_start && state.no_music {
                target = self.position_silence;
            }
            state.position = target;
            match self.state.compare_exchange(
                val,
                state.into(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => return state.position,
                Err(x) => val = x,
            }
        }
        target
    }
}
