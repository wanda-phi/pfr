use unnamed_entity::EntityVec;

use crate::assets::table::{
    lights::{AttractLightId, LightBind, LightId},
    Assets,
};

use super::Table;

pub struct Lights {
    lights: EntityVec<LightId, LightState>,
    attract: EntityVec<AttractLightId, u16>,
}

struct LightState {
    lit: bool,
    state: bool,
    blink: Option<LightBlink>,
}

pub struct LightBlink {
    ctr: u8,
    ctr_off: u8,
    ctr_reset: u8,
}

impl LightBlink {
    pub fn new(half_period: u8, phase: u8) -> Self {
        LightBlink {
            ctr: phase,
            ctr_off: half_period,
            ctr_reset: half_period * 2,
        }
    }
}

impl Lights {
    pub fn new(assets: &Assets) -> Self {
        Lights {
            lights: assets.lights.map(|_, _| LightState {
                lit: false,
                state: false,
                blink: None,
            }),
            attract: assets.attract_lights.map(|_, _| 0),
        }
    }

    pub fn attract_frame(&mut self, assets: &Assets) {
        for (id, ctr) in &mut self.attract {
            *ctr += 1;
            let data = &assets.attract_lights[id];
            if *ctr == data.ctr_off {
                self.lights[data.light].lit = false;
            } else if *ctr == data.ctr_on {
                self.lights[data.light].lit = true;
                *ctr = data.ctr_reset;
            }
        }
    }

    pub fn reset(&mut self) {
        for light in self.lights.ids() {
            self.set_state(light, false);
        }
    }

    pub fn tilt(&mut self) {
        for light in self.lights.ids() {
            self.lights[light].lit = false;
        }
    }

    pub fn is_lit(&self, light: LightId) -> bool {
        self.lights[light].lit
    }

    pub fn state(&self, light: LightId) -> bool {
        self.lights[light].state
    }

    pub fn set_blink(&mut self, light: LightId, blink: LightBlink) {
        let light = &mut self.lights[light];
        light.blink = Some(blink);
    }

    pub fn set_state(&mut self, light: LightId, state: bool) {
        self.lights[light] = LightState {
            lit: state,
            state,
            blink: None,
        };
    }

    pub fn blink_frame(&mut self) {
        for light in self.lights.values_mut() {
            if let Some(ref mut blink) = light.blink {
                if blink.ctr == 0 || blink.ctr == blink.ctr_reset {
                    light.lit = true;
                    blink.ctr = 0;
                } else if blink.ctr == blink.ctr_off {
                    light.lit = false;
                }
                blink.ctr += 1;
            }
        }
    }
}

impl Table {
    pub fn light_blink(&mut self, bind: LightBind, idx: u8, half_period: u8, phase: u8) {
        let light = self.assets.light_binds[bind][idx as usize];
        self.lights
            .set_blink(light, LightBlink::new(half_period, phase));
    }

    pub fn light_set(&mut self, bind: LightBind, idx: u8, state: bool) {
        let light = self.assets.light_binds[bind][idx as usize];
        self.lights.set_state(light, state);
    }

    pub fn light_set_all(&mut self, bind: LightBind, state: bool) {
        for &light in &self.assets.light_binds[bind] {
            self.lights.set_state(light, state);
        }
    }

    pub fn light_state(&self, bind: LightBind, idx: u8) -> bool {
        let light = self.assets.light_binds[bind][idx as usize];
        self.lights.state(light)
    }

    pub fn light_all_lit(&self, bind: LightBind) -> bool {
        self.assets.light_binds[bind]
            .iter()
            .all(|&light| self.lights.state(light))
    }

    pub fn light_all_unlit(&self, bind: LightBind) -> bool {
        self.assets.light_binds[bind]
            .iter()
            .all(|&light| !self.lights.state(light))
    }

    pub fn light_rotate(&mut self, bind: LightBind) {
        let lights = &self.assets.light_binds[bind];
        let states: Vec<_> = lights.iter().map(|&x| self.lights.state(x)).collect();
        for (&light, &state) in lights.iter().zip(states[1..].iter().chain(&states[..1])) {
            self.lights.set_state(light, state);
        }
    }

    pub fn light_sequence(&mut self, bind: LightBind) -> u8 {
        let lights = &self.assets.light_binds[bind];
        for (i, &light) in lights.iter().enumerate() {
            if !self.lights.state(light) {
                self.lights.set_state(light, true);
                return i as u8;
            }
        }
        lights.len() as u8
    }

    pub fn light_save<const N: usize>(&self, bind: LightBind) -> [bool; N] {
        assert_eq!(self.assets.light_binds[bind].len(), N);
        core::array::from_fn(|idx| self.light_state(bind, idx as u8))
    }

    pub fn light_load<const N: usize>(&mut self, bind: LightBind, data: [bool; N]) {
        assert_eq!(self.assets.light_binds[bind].len(), N);
        for (idx, state) in data.into_iter().enumerate() {
            self.light_set(bind, idx as u8, state);
        }
    }
}
