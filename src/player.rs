use std;

use ggez::{
    Context,
    GameResult,
    graphics,
};

use ::Assets;
use constant::{
    PLAYER_LIFE,
    PLAYER_ACCELERATION,
    PLAYER_DECELERATION,
};
use health_bar::StickyHealthBar;
use util::{
    vec_from_angle,
    world_to_screen_coords,
};
use proto;

use ::Movable;
use ::Destroyable;


pub struct Player {
    body: Body,
    life: f32,
    nickname: String,
    nickname_display: graphics::Text,
    color: graphics::Color,
    input: Input,
    body_error: BodyError,
}

impl Player {
    pub fn new(
        ctx: &mut Context,
        nickname: String,
        font: &graphics::Font,
        color: graphics::Color
    ) -> GameResult<Self> {

        let nickname_display = graphics::Text::new(ctx, &nickname, font)?;

        Ok(Player {
            body: Body::default(),
            life: PLAYER_LIFE,
            nickname,
            nickname_display,
            color,
            input: Input::default(),
            body_error: BodyError::default(),
        })
    }

    pub fn set_body(&mut self, body: ProtoBody) {
        self.body = Body::new(body);
    }

    pub fn update_body(&mut self, update: &ProtoBody) {
        let error = self.body.update(update);
        self.body_error.add(error);
    }

    pub fn update_input(&mut self, update: &Input) -> bool {
        self.input.update(&update)
    }

    fn accelerate(&mut self, dt: f32, direction: i32) {
        if direction == 0 {
            return;
        }

        let mut angle = self.body.rot;
        let mut accel_value = PLAYER_ACCELERATION;
        if direction < 0 {
            angle += std::f32::consts::PI;
            accel_value = PLAYER_DECELERATION;
        }

        let dir_vec = vec_from_angle(angle);
        let acceleration = dir_vec * accel_value;
        self.body.vel += acceleration * dt;
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets, coords: (u32, u32)) -> GameResult<()> {
        if self.is_ready() {
            let (sw, sh) = coords;
            let pos = graphics::Point2::new(
                self.body.pos.x + self.body_error.pos_error.x,
                self.body.pos.y + self.body_error.pos_error.y
            );
            let pos = world_to_screen_coords(sw, sh, pos);

            let image = assets.player_image();

            graphics::draw_ex(
                ctx,
                image,
                graphics::DrawParam {
                    dest: pos,
                    rotation: self.body.rot + self.body_error.rot_error,
                    offset: graphics::Point2::new(0.5, 0.5),
                    scale: graphics::Point2::new(
                        self.body.size / image.width() as f32,
                        self.body.size / image.height() as f32
                    ),
                    .. Default::default()
                }
            )?;

            StickyHealthBar::draw(
                ctx, pos, self.body.size,
                self.life() / self.max_life(),
                Some(self.color)
            )?;

            let dest = graphics::Point2::new(
                pos.x - (self.nickname_display.width() / 2) as f32,
                pos.y - self.body.size / 2.0 - self.nickname_display.height() as f32,
            );
            graphics::draw_ex(
                ctx,
                &self.nickname_display,
                graphics::DrawParam {
                    dest,
                    color: Some(self.color),
                    .. Default::default()
                }
            )?;
        }

        Ok(())
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
    fn update_position(&mut self, dt: f32) {
        let acceleration = self.input.accel.unwrap_or(0);
        self.accelerate(dt, acceleration);
        self.body.rotate(dt, self.input.turn.unwrap_or(0) as f32);
        self.body.update_position(dt);

        self.body_error.reduce();
    }

    fn wrap_position(&mut self, sx: f32, sy: f32) {
        self.body.wrap_position(sx, sy);
    }

    fn get_body(&self) -> &Body {
        &self.body
    }
}
