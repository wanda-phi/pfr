use rand::Rng;

use crate::assets::table::physics::Layer;

use super::physics::speed_fix;

pub struct BallState {
    pub layer: Layer,
    pub pos_hires: (i32, i32),
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

    pub fn pos(&self) -> (i16, i16) {
        (
            (self.pos_hires.0 >> 10) as i16,
            (self.pos_hires.1 >> 10) as i16,
        )
    }

    pub fn pos_center(&self) -> (i16, i16) {
        (
            (self.pos_hires.0 >> 10) as i16 + 8,
            (self.pos_hires.1 >> 10) as i16 + 8,
        )
    }

    pub fn set_pos(&mut self, pos: (i16, i16)) {
        self.pos_hires = (((pos.0 as i32) << 10), ((pos.1 as i32) << 10));
    }

    pub fn teleport_freeze(&mut self, layer: Layer, pos: (i16, i16)) {
        self.layer = layer;
        self.set_pos(pos);
        self.speed = (0, 0);
        self.frozen = true;
    }

    pub fn teleport(&mut self, layer: Layer, pos: (i16, i16), speed: (i16, i16)) {
        self.layer = layer;
        self.set_pos(pos);
        self.speed = speed;
        self.frozen = false;
        let random: i16 = rand::thread_rng().gen_range(0..0x400);
        if (random & 1) != 0 {
            self.rotation = -random;
        } else {
            self.rotation = random;
        }
    }
}
