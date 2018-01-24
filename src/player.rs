use std;

use ggez::{
    Context,
    GameResult,
    graphics,
};

use body::Body;
use constant::{
    PLAYER_LIFE,
    PLAYER_ACCELERATION,
    PLAYER_DECELERATION,
};
use util::vec_from_angle;
use proto::astero;


pub struct Player {
    body: Body,
    life: f32,
    nickname: String,
    nickname_display: graphics::Text,
    color: graphics::Color,
}

impl Player {
    fn accelerate(&mut self, dt: f32, direction: i32) {
        if direction == 0 {
            return;
        }

        let (angle, accel_value) = if direction < 0 {
            (self.body.rot, PLAYER_ACCELERATION)
        } else {
            (self.body.rot + std::f32::consts::PI, PLAYER_DECELERATION)
        };

        let dir_vec = vec_from_angle(angle);
        let acceleration = dir_vec * accel_value;
        self.body.vel += acceleration * dt;
    }
}
