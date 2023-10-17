use rand::Rng;

use crate::assets::table::physics::Layer;

use super::physics::speed_fix;

pub struct BallState {
    pub layer: Layer,
    pub pos_hires: (u32, u32),
    pub speed: (i16, i16),
    pub accel: (i16, i16),
    pub frozen: bool,
    pub rotation: i16,
    pub max_speed: i16,
}

impl BallState {
    pub fn new(hifps: bool) -> Self {
        Self {
            layer: Layer::Ground,
            pos_hires: (0, 0),
            speed: (0, 0),
            accel: (0, 8),
            frozen: true,
            rotation: 0,
            max_speed: speed_fix(4100, hifps),
        }
    }

    pub fn pos(&self) -> (u16, u16) {
        (
            (self.pos_hires.0 >> 10) as u16,
            (self.pos_hires.1 >> 10) as u16,
        )
    }

    pub fn pos_center(&self) -> (u16, u16) {
        (
            (self.pos_hires.0 >> 10) as u16 + 8,
            (self.pos_hires.1 >> 10) as u16 + 8,
        )
    }

    pub fn set_pos(&mut self, pos: (u16, u16)) {
        self.pos_hires = (((pos.0 as u32) << 10), ((pos.1 as u32) << 10));
    }

    pub fn teleport_freeze(&mut self, layer: Layer, pos: (u16, u16)) {
        self.layer = layer;
        self.set_pos(pos);
        self.speed = (0, 0);
        self.frozen = true;
    }

    pub fn teleport(&mut self, layer: Layer, pos: (u16, u16), speed: (i16, i16)) {
        self.layer = layer;
        self.set_pos(pos);
        self.speed = speed;
        self.frozen = false;
        let random: i16 = rand::thread_rng().gen_range(0..0x400);
        if (random & 1) != 0 {
            self.rotation = -(random as i16);
        } else {
            self.rotation = random as i16;
        }
    }
}
