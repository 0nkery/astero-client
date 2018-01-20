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

use ::Movable;
use ::Destroyable;


pub struct Player {
    body: Body,
    life: f32,
    nickname: String,
    nickname_display: graphics::Text,
    color: graphics::Color,
}

impl Player {
    pub fn new(
        ctx: &mut Context,
        nickname: String,
        font: &graphics::Font,
        color: graphics::Color
    ) -> GameResult<Self> {

        let nickname_display = graphics::Text::new(ctx, &nickname, font)?;

        Ok(Self {
            body: Body::default(),
            life: PLAYER_LIFE,
            nickname,
            nickname_display,
            color,
        })
    }

    pub fn set_body(&mut self, body: &astero::Body) {
        self.body = Body::new(body);
    }

    pub fn update_body(&mut self, update: &astero::Body) {
        self.body.update(update);
    }

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

    pub fn is_ready(&self) -> bool {
        self.body.size > 0.0
    }
}

impl Player {
    pub fn max_life(&self) -> f32 {
        PLAYER_LIFE
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }
}

impl Destroyable for Player {
    fn life(&self) -> f32 {
        self.life
    }

    fn damage(&mut self, amount: f32) {
        self.life -= amount;
    }

    fn destroy(&mut self) {
        self.life = 0.0;
    }
}

impl Movable for Player {
    fn update_position(&mut self, dt: f32) {}

    fn wrap_position(&mut self, sx: f32, sy: f32) {}

    fn get_body(&self) -> &Body {
        &self.body
    }
}
