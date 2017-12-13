use std;

use ggez::{
    Context,
    GameResult,
    graphics,
};
use nalgebra;

use super::{
    Assets,
    InputState,
    Movable,
};
use constant::{
    PLAYER_BBOX,
    PLAYER_LIFE,
    PLAYER_TURN_RATE,
    PLAYER_ACCELERATION,
    PLAYER_DECELERATION,
    SPRITE_SIZE,
    SPRITE_HALF_SIZE,
};
use health_bar::StickyHealthBar;
use util::{
    Point2,
    Vector2,
    vec_from_angle,
    world_to_screen_coords,
};


pub struct Player {
    pos: Option<Point2>,
    facing: f32,
    velocity: Vector2,
    rvel: f32,
    bbox_size: f32,
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
            pos: None,
            facing: 0.0,
            velocity: nalgebra::zero(),
            rvel: 0.0,
            bbox_size: PLAYER_BBOX,
            life: PLAYER_LIFE,
            nickname,
            nickname_display,
            color,
        })
    }

    pub fn handle_input(&mut self, input: &InputState, dt: f32) {
        if self.pos.is_none() {
            return;
        }

        self.facing += dt * PLAYER_TURN_RATE * input.xaxis;

        if input.yaxis > 0.0 {
            self.accelerate(dt);
        } else if input.yaxis < 0.0 {
            self.decelerate(dt);
        }
    }

    fn accelerate(&mut self, dt: f32) {
        let direction_vector = vec_from_angle(self.facing);
        let acceleration = direction_vector * PLAYER_ACCELERATION;
        self.velocity += acceleration * dt;
    }

    fn decelerate(&mut self, dt: f32) {
        let direction_vector = vec_from_angle(self.facing + std::f32::consts::PI);
        let deceleration_vector = direction_vector * PLAYER_DECELERATION;
        self.velocity += deceleration_vector * dt;
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets, coords: (u32, u32)) -> GameResult<()> {
        if let Some(ref pos) = self.pos {
            let (screen_w, screen_h) = coords;
            let pos = world_to_screen_coords(
                screen_w, screen_h, *pos
            );
            let dest_point = graphics::Point::new(pos.x as f32, pos.y as f32);
            let image = assets.player_image();

            graphics::draw(ctx, image, dest_point, self.facing)?;

            StickyHealthBar::draw(
                ctx, pos.x, pos.y + SPRITE_HALF_SIZE + 6.0,
                SPRITE_SIZE as f32, self.cur_life(), self.max_life(),
                Some(self.color)
            )?;

            let old_color = graphics::get_color(ctx);
            graphics::set_color(ctx, self.color)?;

            let nickname_dest = graphics::Point::new(pos.x, pos.y - SPRITE_HALF_SIZE - 7.0);
            graphics::draw(ctx, &self.nickname_display, nickname_dest, 0.0)?;

            graphics::set_color(ctx, old_color)?;
        }

        Ok(())
    }
}

impl Player {
    pub fn cur_life(&self) -> f32 {
        self.life
    }

    pub fn max_life(&self) -> f32 {
        PLAYER_LIFE
    }

    pub fn bbox_size(&self) -> f32 {
        self.bbox_size
    }

    pub fn damage(&mut self, dmg: f32) {
        self.life -= dmg;
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }
}

impl Movable for Player {
    fn velocity(&self) -> Vector2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Vector2) {
        self.velocity = velocity;
    }

    fn pos(&self) -> Option<Point2> {
        self.pos
    }

    fn set_pos(&mut self, pos: Point2) {
        self.pos = Some(pos);
    }

    fn facing(&self) -> f32 {
        self.facing
    }

    fn set_facing(&mut self, facing: f32) {
        self.facing = facing;
    }

    fn rvel(&self) -> f32 {
        self.rvel
    }

    fn set_rvel(&mut self, rvel: f32) {
        self.rvel = rvel;
    }
}