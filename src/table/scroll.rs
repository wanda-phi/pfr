use crate::config::{Options, Resolution, ScrollSpeed};

pub struct ScrollState {
    pos: u16,
    raw_pos_f4: i16,
    speed: i16,
    window_height: u16,
    target_special: Option<u16>,
    ball_target: u16,
    attract_up: bool,
}

impl ScrollState {
    pub fn new(options: &Options) -> Self {
        let window_height = match options.resolution {
            Resolution::Normal => 240 - 33,
            Resolution::High => 350 - 33,
            Resolution::Full => 576,
        };
        Self {
            pos: 576 - window_height,
            raw_pos_f4: 0,
            speed: match options.scroll_speed {
                ScrollSpeed::Hard => 20,
                ScrollSpeed::Medium => 11,
                ScrollSpeed::Soft => 9,
            },
            window_height,
            target_special: None,
            ball_target: match options.resolution {
                Resolution::Normal => 75,
                Resolution::High => 130,
                Resolution::Full => 0,
            },
            attract_up: true,
        }
    }

    pub fn update(&mut self, ball_y: u16) {
        if self.window_height == 576 {
            self.pos = 0;
            return;
        }
        let target = self.target_special.unwrap_or(if ball_y < self.ball_target {
            0
        } else {
            (ball_y - self.ball_target).min(576 - self.window_height)
        });
        let delta = (target as i16) - (self.raw_pos_f4 >> 4);
        let diff = (delta * self.speed) >> 2;
        self.raw_pos_f4 += diff;
        let delta = (target as i16) - (self.raw_pos_f4 >> 4);
        if delta <= -(self.ball_target as i16) {
            self.raw_pos_f4 += (delta + self.ball_target as i16) << 4;
        } else if delta >= self.ball_target as i16 + 40 {
            self.raw_pos_f4 += (delta - self.ball_target as i16 - 40) << 4;
        }
        self.pos = (self.raw_pos_f4 >> 4) as u16;
    }

    pub fn attract_frame(&mut self) {
        if self.window_height == 576 {
            self.pos = 0;
            return;
        }
        if self.pos == 0 {
            self.attract_up = false;
        } else if self.pos == 576 - self.window_height {
            self.attract_up = true;
        }
        if self.attract_up {
            self.pos -= 1;
        } else {
            self.pos += 1;
        }
        self.raw_pos_f4 = (self.pos << 4) as i16;
    }

    pub fn pos(&self) -> u16 {
        self.pos
    }

    pub fn set_speed(&mut self, speed: i16) {
        self.speed = speed;
    }

    pub fn set_special_target(&mut self, target: u16) {
        self.target_special = Some(target);
    }

    pub fn set_special_target_now(&mut self, target: u16) {
        self.target_special = Some(target);
        if self.window_height != 576 {
            self.raw_pos_f4 = (target << 4) as i16;
            self.pos = target;
        }
    }

    pub fn reset_special_target(&mut self) {
        self.target_special = None;
    }
}
