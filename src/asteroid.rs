use ggez::{Context, graphics, GameResult};

use client::proto;

use health_bar;
use constant::{
    ROCK_BBOX,
    SPRITE_HALF_SIZE,
    SPRITE_SIZE,
    ROCK_LIFE,
};
use util::{world_to_screen_coords, Vector2, Point2};

use ::Movable;


pub struct Asteroid {
    inner: proto::Asteroid,
    pub bbox_size: f32,
}

impl ::Destroyable for Asteroid {
    fn life(&self) -> f32 {
        self.inner.life
    }

    fn damage(&mut self, amount: f32) {
        self.inner.life -= amount;
    }
    fn destroy(&mut self) {
        self.inner.life = 0.0;
    }
}

impl Asteroid {
    pub fn new(inner: proto::Asteroid) -> Self {
        Asteroid {
            inner,
            bbox_size: ROCK_BBOX
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut ::Assets, world_coords: (u32, u32)) -> GameResult<()> {
        let (screen_w, screen_h) = world_coords;
        let pos = self.inner.pos.into();
        let pos = world_to_screen_coords(screen_w, screen_h, pos);
        let dest_point = graphics::Point::new(pos.x as f32, pos.y as f32);
        let image = assets.asteroid_image();
        graphics::draw(ctx, image, dest_point, self.facing())?;

        let x = pos.x;
        let y = pos.y + SPRITE_HALF_SIZE + 4.0;

        health_bar::StickyHealthBar::draw(
            ctx, x, y,
            SPRITE_SIZE as f32, self.inner.life, ROCK_LIFE,
            None
        )?;

        Ok(())
    }
}


impl ::Movable for Asteroid {
    fn velocity(&self) -> Vector2 {
        self.inner.velocity.into()
    }

    fn set_velocity(&mut self, velocity: Vector2) {
        self.inner.velocity = velocity.into();
    }

    fn pos(&self) -> Option<Point2> {
        Some(self.inner.pos.into())
    }

    fn set_pos(&mut self, pos: Point2) {
        self.inner.pos = pos.into();
    }

    fn facing(&self) -> f32 {
        0.0
    }

    fn set_facing(&mut self, _facing: f32) {}

    fn rvel(&self) -> f32 {
        0.0
    }

    fn set_rvel(&mut self, _rvel: f32) {}
}