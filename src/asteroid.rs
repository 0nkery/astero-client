use ggez::{Context, graphics, GameResult};

use client::proto::{
    Body,
    ProtoBody,
    Asteroid as ProtoAsteroid,
};

use health_bar;
use constant::ROCK_LIFE;
use util::world_to_screen_coords;

use ::Movable;


pub struct Asteroid {
    body: Body,
    life: f32,
}

impl ::Destroyable for Asteroid {
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

impl Asteroid {
    pub fn new(inner: ProtoAsteroid) -> Self {
        Asteroid {
            body: Body::new(inner.body),
            life: inner.life,
        }
    }

    pub fn update_body(&mut self, body: &ProtoBody) {
        self.body.update(body);
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut ::Assets, world_coords: (u32, u32)) -> GameResult<()> {
        let (screen_w, screen_h) = world_coords;
        let pos = self.body.pos;
        let pos = world_to_screen_coords(screen_w, screen_h, pos);
        let dest_point = graphics::Point::new(pos.x as f32, pos.y as f32);
        let image = assets.asteroid_image();
        graphics::draw(ctx, image, dest_point, self.body.rot)?;

        let x = pos.x;
        let y = pos.y + self.body.size / 2.0 + 4.0;

        health_bar::StickyHealthBar::draw(
            ctx, x, y,
            self.body.size, self.life, ROCK_LIFE,
            None
        )?;

        Ok(())
    }
}


impl Movable for Asteroid {
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