use body::Body;
use proto::astero;

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
    pub fn new(inner: &astero::Asteroid) -> Self {
        Self {
            body: Body::new(&inner.body),
            life: inner.life.unwrap_or(0.0),
        }
    }

    pub fn update_body(&mut self, body: &astero::Body) {
        self.body.update(body);
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