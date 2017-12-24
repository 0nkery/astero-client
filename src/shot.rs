use ggez::{
    graphics,
    Context,
    GameResult,
};

use ::Assets;
use constant::{
    SHOT_LIFE,
    SHOT_SPEED,
};
use client::proto::Body;
use util::{
    Point2,
    vec_from_angle,
    world_to_screen_coords,
};

use ::Movable;
use ::Destroyable;


pub struct Shot {
    body: Body,
    ttl: f32,
}

impl Shot {
    pub fn new() -> Self {
        let mut body = Body::default();
        body.size = 6.0;

        Shot {
            body,
            ttl: SHOT_LIFE,
        }
    }

    pub fn with_coord(mut self, pos: Point2) -> Self {
        self.body.pos = pos;

        self
    }

    pub fn with_rot(mut self, rot: f32) -> Self {
        self.body.rot = rot;
        let direction = vec_from_angle(rot);
        self.body.vel = direction * SHOT_SPEED;

        self
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets, world_coords: (u32, u32)) -> GameResult<()> {
        let (screen_w, screen_h) = world_coords;
        let pos = world_to_screen_coords(screen_w, screen_h, self.body.pos);
        let dest_point = graphics::Point2::new(pos.x as f32, pos.y as f32);
        let image = assets.shot_image();
        graphics::draw(ctx, image, dest_point, self.body.rot)?;

        Ok(())
    }
}

impl Movable for Shot {
    fn update_position(&mut self, dt: f32) {
        self.ttl -= dt;
        self.body.update_position(dt);
    }

    fn wrap_position(&mut self, sx: f32, sy: f32) {
        self.body.wrap_position(sx, sy);
    }

    fn get_body(&self) -> &Body {
        &self.body
    }
}

impl Destroyable for Shot {
    fn life(&self) -> f32 {
        self.ttl
    }

    fn damage(&mut self, _amount: f32) {}

    fn destroy(&mut self) {
        self.ttl = 0.0;
    }
}