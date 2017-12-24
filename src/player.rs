use std;

use ggez::{
    Context,
    GameResult,
    graphics,
};

use ::{
    Assets,
    InputState,
};
use client::proto::{Body, ProtoBody};
use constant::{
    PLAYER_LIFE,
    PLAYER_ACCELERATION,
    PLAYER_DECELERATION,
};
use shot::Shot;
use health_bar::StickyHealthBar;
use util::{
    vec_from_angle,
    world_to_screen_coords,
};

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

        Ok(Player {
            body: Body::default(),
            life: PLAYER_LIFE,
            nickname,
            nickname_display,
            color,
        })
    }

    pub fn set_body(&mut self, body: ProtoBody) {
        self.body = Body::new(body);
    }

    pub fn update_body(&mut self, body: &ProtoBody) {
        self.body.update(body);
    }

    pub fn handle_input(&mut self, input: &InputState, dt: f32) {
        if !self.is_ready() {
            return;
        }

        self.body.rotate(dt, input.xaxis);

        if input.yaxis > 0.0 {
            self.accelerate(dt);
        } else if input.yaxis < 0.0 {
            self.decelerate(dt);
        }
    }

    fn accelerate(&mut self, dt: f32) {
        let direction_vector = vec_from_angle(self.body.rot);
        let acceleration = direction_vector * PLAYER_ACCELERATION;
        self.body.vel += acceleration * dt;
    }

    fn decelerate(&mut self, dt: f32) {
        let direction_vector = vec_from_angle(self.body.rot + std::f32::consts::PI);
        let deceleration_vector = direction_vector * PLAYER_DECELERATION;
        self.body.vel += deceleration_vector * dt;
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets, coords: (u32, u32)) -> GameResult<()> {
        if self.is_ready() {
            let (sw, sh) = coords;
            let pos = world_to_screen_coords(sw, sh, self.body.pos);

            graphics::draw_ex(
                ctx,
                assets.player_image(),
                graphics::DrawParam {
                    dest: pos,
                    rotation: self.body.rot,
                    offset: graphics::Point2::new(0.5, 0.5),
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

    pub fn fire(&self) -> Shot {
        Shot::new()
            .with_coord(self.body.pos)
            .with_rot(self.body.rot)
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
        self.body.update_position(dt);
    }

    fn wrap_position(&mut self, sx: f32, sy: f32) {
        self.body.wrap_position(sx, sy);
    }

    fn get_body(&self) -> &Body {
        &self.body
    }
}