use body::Body;
use proto::astero;

use ::Movable;
use ::Destroyable;


pub struct Shot {
    body: Body,
    ttl: f32,
}


impl Shot {
    pub fn new(shot: &astero::Shot) -> Self {
        Self {
            body: Body::new(&shot.body),
            ttl: shot.ttl,
        }
    }
}

impl Movable for Shot {
    fn update_position(&mut self, dt: f32) {
        self.ttl -= dt;
        self.body.update_position(dt);
    }

    fn wrap_position(&mut self, _xb: f32, _yb: f32) {}

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