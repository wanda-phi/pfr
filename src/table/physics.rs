use enum_map::EnumMap;
use ndarray::{s, Array2};
use rand::Rng;

use crate::{
    assets::table::{
        flippers::{Flipper, FlipperSide},
        physics::{Layer, Material, PhysmapBind, MATERIALS, MATERIAL_KICKER},
        sound::SfxBind,
    },
    bcd::Bcd,
};

use super::Table;

pub struct PushState {
    offset_f9: i16,
    speed: i16,
    speed_attack: i16,
    speed_release: i16,
}

impl PushState {
    pub fn new(hifps: bool) -> Self {
        Self {
            offset_f9: 0,
            speed: 0,
            speed_attack: speed_fix(600, hifps),
            speed_release: speed_fix(-200, hifps),
        }
    }

    pub fn frame(&mut self, state: bool) {
        if state {
            self.speed = self.speed_attack;
            self.offset_f9 += self.speed;
            if self.offset_f9 > 0x800 {
                self.speed = 0;
                self.offset_f9 = 0x800;
            }
        } else {
            self.speed = self.speed_release;
            self.offset_f9 += self.speed;
            if self.offset_f9 < 0 {
                self.speed = 0;
                self.offset_f9 = 0;
            }
        }
    }

    pub fn offset(&self) -> u16 {
        (self.offset_f9 >> 9) as u16
    }
}

pub struct FlipperState {
    pub pos: i16,
    pub speed: i16,
    pub quantum: u16,
    pub prev_quantum: u16,
    pub accel_press: i16,
    pub accel_release: i16,
    pub speed_press_start: i16,
}

impl FlipperState {
    pub fn new(flipper: &Flipper, hifps: bool) -> Self {
        Self {
            pos: 0,
            speed: 0,
            quantum: 0,
            prev_quantum: 1,
            accel_press: speed_fix(flipper.accel_press, hifps),
            accel_release: speed_fix(flipper.accel_release, hifps),
            speed_press_start: speed_fix(flipper.speed_press_start, hifps),
        }
    }
}

pub fn speed_fix(speed: i16, hifps: bool) -> i16 {
    if hifps {
        speed
    } else {
        ((speed as i32) * 5 / 6) as i16
    }
}

pub fn prep_materials(hifps: bool) -> [Material; 8] {
    MATERIALS.map(|mut x| {
        x.min_bounce_speed = speed_fix(x.min_bounce_speed, hifps);
        x
    })
}

fn physmap_patch(
    physmaps: &mut EnumMap<Layer, Array2<u8>>,
    layer: Layer,
    pos: (u16, u16),
    src: &Array2<u8>,
) {
    physmaps[layer]
        .slice_mut(s![
            (pos.0 as usize)..(pos.0 as usize + src.dim().0),
            (pos.1 as usize)..(pos.1 as usize + src.dim().1),
        ])
        .assign(src);
}

#[derive(Copy, Clone, Debug)]
struct Collision {
    flipper_speed: (i16, i16),
    angle: u16,
    material: usize,
    cnt: u16,
}

impl Table {
    pub fn physics_frame(&mut self) {
        if self.ball.frozen {
            self.push.frame(self.space_state);
            self.flippers_move();
            self.flippers_physmap_update();
        } else {
            if let Some(coll) = self.physics_check_collision() {
                self.physics_new_dir(coll);
            }
            self.push.frame(self.space_state);
            self.flippers_move();
            self.ball_move();
            self.flippers_physmap_update();
        }
    }

    fn physics_check_collision(&mut self) -> Option<Collision> {
        let mut angle_sum = 0;
        let mut quad = 0;
        let mut ctr_b = 0;
        let mut cnt = 0;
        let mut material = None;
        for pix in &self.assets.ball_outline {
            let x = self.ball.pos().0 + pix.x - 1;
            let y = self.ball.pos().1 + self.push.offset() + pix.y - 1;
            if !(0..576).contains(&y) || !(0..320).contains(&x) {
                continue;
            }
            let byte = self.physmaps[self.ball.layer][(x as usize, y as usize)];
            if (byte & 2) != 0 {
                angle_sum += pix.angle;
                quad |= pix.quad;
                if pix.is_bot {
                    ctr_b += 1;
                }
                material = Some(byte as usize & 7);
                cnt += 1;
            }
        }
        if cnt == 0 {
            return None;
        }
        let material = material.unwrap();
        if matches!(quad, 0xb | 9 | 0xd) {
            angle_sum += (ctr_b as u16) << 11;
        }
        let angle = (angle_sum / cnt) & 0x7ff;
        let idx = (angle as usize * 0x580 + 0x8000) >> 16;
        let hit_pos = self.assets.ball_outline_by_angle[idx % 44];
        let hit_pos = (hit_pos.0 + self.ball.pos().0, hit_pos.1 + self.ball.pos().1);
        self.hit_pos = Some(hit_pos);
        let mut flipper_speed = (0, 0);
        match material {
            2 => {
                for (fid, flipper) in &self.assets.flippers {
                    let state = &self.flippers[fid];
                    if flipper.ball_bbox.contains(hit_pos) {
                        let mut dx = hit_pos.0 as i16 - flipper.origin.0 as i16;
                        let mut dy = hit_pos.1 as i16 - flipper.origin.1 as i16;
                        match flipper.side {
                            FlipperSide::Left => {
                                if dx < 0 {
                                    continue;
                                }
                            }
                            FlipperSide::Right => {
                                if dx >= 0 {
                                    continue;
                                }
                                if !flipper.is_vertical {
                                    dx = -dx;
                                    dy = -dy;
                                }
                            }
                        }
                        let extra = if flipper.is_vertical {
                            core::mem::swap(&mut dx, &mut dy);
                            (dy >> 1).abs()
                        } else {
                            dy.abs() >> 2
                        };
                        flipper_speed = (dy * -state.speed, -(dx + extra) * -state.speed);
                    }
                }
            }
            3 | 7 if !self.tilted => {
                for (bid, bumper) in &self.assets.bumpers {
                    if bumper.is_kicker != (material == 3) {
                        continue;
                    }
                    if bumper.rect.contains(hit_pos) {
                        self.hit_bumper = Some(bid);
                    }
                }
            }
            _ => (),
        }
        Some(Collision {
            flipper_speed,
            angle,
            material,
            cnt,
        })
    }

    fn physics_new_dir(&mut self, collision: Collision) {
        let material = &self.materials[collision.material];
        let speed = (
            (self.ball.speed.0 + collision.flipper_speed.0)
                .max(-self.ball.max_speed)
                .min(self.ball.max_speed),
            (self.ball.speed.1 + collision.flipper_speed.1 + self.push.speed)
                .max(-self.ball.max_speed)
                .min(self.ball.max_speed),
        );
        let angle = ((0x800 - collision.angle) & 0x7ff) as usize;
        let cos = self.assets.sine_table[angle + 0x200];
        let sin = self.assets.sine_table[angle];
        let mut dot = ((speed.0 as i32 * cos as i32) - (speed.1 as i32 * sin as i32)) >> 13;
        let mut cross = ((speed.0 as i32 * sin as i32) + (speed.1 as i32 * cos as i32)) >> 13;
        if dot <= 0 {
            self.hit_bumper = None;
            return;
        }
        if dot <= material.min_bounce_speed as i32 {
            dot = 0;
            self.hit_bumper = None;
        } else {
            let bounce_factor = ((cross * 0x10) / dot).abs() as i16;
            if bounce_factor < material.max_bounce_angle {
                if self.hit_bumper.is_some() {
                    if collision.material == MATERIAL_KICKER {
                        if dot < self.kicker_speed_threshold as i32 {
                            self.hit_bumper = None;
                        } else {
                            dot += self.kicker_speed_boost as i32;
                        }
                    } else {
                        dot += self.bumper_speed_boost as i32;
                    }
                }
            } else {
                dot = 0;
                self.hit_bumper = None;
            }
        }
        dot -= dot * 256 / material.bounce_factor as i32;
        let mut cx = material.unk0 as i32;
        let mut bp = material.unk2 as i32;
        if dot < 1024 {
            let factor = (dot >> 6) + 1;
            cx *= factor;
            bp *= factor;
        }
        dot = -dot;
        let rot = self.ball.rotation as i32 + self.push.speed as i32 - cross;
        cross += rot * 256 / cx;
        self.ball.rotation -= (rot * 256 / bp) as i16;
        cross = cross * 0x800 / 0x801;
        let cos = self.assets.sine_table[0x200 + collision.angle as usize] as i32;
        let sin = self.assets.sine_table[collision.angle as usize] as i32;
        let mut speed_x = ((dot * cos - cross * sin) >> 15) as i16;
        let mut speed_y = ((dot * sin + cross * cos) >> 15) as i16;
        speed_x -= collision.flipper_speed.0;
        speed_y -= collision.flipper_speed.1;
        speed_y -= self.push.speed;
        self.ball.speed = (
            speed_x.min(self.ball.max_speed).max(-self.ball.max_speed),
            speed_y.min(self.ball.max_speed).max(-self.ball.max_speed),
        );
        if collision.cnt >= 6 {
            self.ball.pos_hires.0 = self.ball.pos_hires.0.wrapping_add_signed(-cos >> 6);
            self.ball.pos_hires.1 = self.ball.pos_hires.1.wrapping_add_signed(-sin >> 6);
        }
    }

    fn ball_move(&mut self) {
        self.ball.pos_hires = (
            self.ball
                .pos_hires
                .0
                .wrapping_add_signed(self.ball.speed.0.into()),
            self.ball
                .pos_hires
                .1
                .wrapping_add_signed(self.ball.speed.1.into()),
        );
        if self.ball.pos().1 >= 576 {
            self.drained = true;
        }
        self.ball.speed.0 += self.ball.accel.0;
        self.ball.speed.1 += self.ball.accel.1;
        if self.ball.rotation < 0 {
            self.ball.rotation += 2;
            if self.ball.rotation > 0 {
                self.ball.rotation = 0;
            }
        } else {
            self.ball.rotation -= 2;
            if self.ball.rotation < 0 {
                self.ball.rotation = 0;
            }
        }
    }

    pub fn spring_release(&mut self) {
        if self.at_spring {
            let factor = if self.hifps { -166 } else { -138 };
            self.ball.speed = (
                0,
                factor * self.spring_pos as i16 - rand::thread_rng().gen_range(0..0x100),
            );
            self.ball.rotation = rand::thread_rng().gen_range(0..0x10);
        }
        let volume = self.spring_pos * 2;
        self.play_sfx_bind_volume(SfxBind::SpringUp, volume);
        self.spring_pos = 0;
    }

    fn flippers_move(&mut self) {
        for (fid, flipper) in &self.assets.flippers {
            let state = &mut self.flippers[fid];
            if self.flipper_state[flipper.side] && self.flippers_enabled {
                state.speed = (state.speed + state.accel_press).max(state.speed_press_start);
            } else {
                state.speed += state.accel_release;
            }
            state.pos += state.speed;
            if state.pos < 55 {
                state.pos = 0;
                state.speed = 0;
            }
            if state.pos > flipper.pos_max {
                state.pos = flipper.pos_max;
                state.speed = 0;
            }
            state.quantum = (state.pos / 55) as u16;
        }
    }

    pub fn flippers_physmap_update(&mut self) {
        for (fid, flipper) in &self.assets.flippers {
            let state = &mut self.flippers[fid];
            if state.quantum != state.prev_quantum {
                state.prev_quantum = state.quantum;
                physmap_patch(
                    &mut self.physmaps,
                    Layer::Ground,
                    flipper.rect_pos,
                    &flipper.physmap[state.quantum as usize],
                )
            }
        }
    }

    pub fn drop_physmap(&mut self, bind: PhysmapBind) {
        let patch = self.assets.physmap_patches[bind].as_ref().unwrap();
        physmap_patch(&mut self.physmaps, patch.layer, patch.pos, &patch.dropped);
    }

    pub fn raise_physmap(&mut self, bind: PhysmapBind) {
        let patch = self.assets.physmap_patches[bind].as_ref().unwrap();
        physmap_patch(&mut self.physmaps, patch.layer, patch.pos, &patch.raised);
    }

    pub fn ball_gravity(&mut self) {
        let pos = self.ball.pos_center();
        if !(0..576).contains(&pos.1) || !(0..320).contains(&pos.0) {
            return;
        }
        let ramp = (self.physmaps[self.ball.layer][(pos.0 as usize, pos.1 as usize)] >> 4) as usize;
        if ramp == 0xf {
            return;
        }
        let ramp = &self.assets.ramps[ramp];
        self.ball.accel = if self.hifps {
            ramp.accel_hires
        } else {
            ramp.accel
        };
        if !self.options.angle_high {
            self.ball.accel.1 -= 3;
        }
    }

    pub fn ball_center(&self) -> (u16, u16) {
        let mut pos = self.ball.pos();
        pos.0 += 8;
        pos.1 += 8;
        pos.1 += self.push.offset();
        pos
    }

    pub fn check_transitions(&mut self) {
        let pos = self.ball_center();
        match self.ball.layer {
            Layer::Ground => {
                if self.assets.transitions_up.iter().any(|x| x.contains(pos)) {
                    self.ball.layer = Layer::Overhead;
                }
            }
            Layer::Overhead => {
                if self.assets.transitions_down.iter().any(|x| x.contains(pos)) {
                    self.ball.layer = Layer::Ground;
                }
            }
        }
    }

    pub fn score_bumper(&mut self) {
        if let Some(bid) = self.hit_bumper {
            self.hit_bumper = None;
            let bumper = &self.assets.bumpers[bid];
            self.player.play_sfx(bumper.sfx, 0x40);
            self.score(bumper.score, Bcd::ZERO);
            self.mode_count_hit();
        }
    }
}
