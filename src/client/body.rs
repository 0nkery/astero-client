use util::{
    Point2,
    Vector2,
    reflect_vector,
};

use constant::MAX_PHYSICS_VEL;

use super::proto_defs::astero::{
    Body as ProtoBody,
};


#[derive(Debug)]
pub struct Body {
    pub pos: Point2,
    pub vel: Vector2,
    pub rot: f32,
    pub rvel: f32,
    pub size: f32,
}

impl Body {
    pub fn new(body: ProtoBody) -> Self {
        Body {
            pos: body.pos.into(),
            vel: body.vel.into(),
            rot: body.rot.unwrap_or(0.0),
            rvel: body.rvel.unwrap_or(0.0),
            size: body.size.unwrap_or(0.0),
        }
    }

    pub fn update(&mut self, body: &ProtoBody) {
        self.pos = body.pos.into();
        self.vel = body.vel.into();
        self.rot = body.rot.unwrap_or(self.rot);
        self.rvel = body.rvel.unwrap_or(self.rvel);
        self.size = body.size.unwrap_or(self.size);
    }

    pub fn update_position(&mut self, dt: f32) {
        if self.size <= 0.0 {
            return;
        }

        let norm_sq = self.vel.norm_squared();
        if norm_sq > MAX_PHYSICS_VEL.powi(2) {
            self.vel = self.vel / norm_sq.sqrt() * MAX_PHYSICS_VEL;
        }

        let dv = self.vel * dt;
        self.pos += dv;
    }

    pub fn rotate(&mut self, dt: f32, direction: f32) {
        self.rot += self.rvel * dt * direction;
    }

    pub fn wrap_position(&mut self, sx: f32, sy: f32) {
        if self.size <= 0.0 {
            return;
        }

        let screen_x_bounds = sx / 2.0;
        let screen_y_bounds = sy / 2.0;

        let center = self.pos + Vector2::new(self.pos.x.signum() * self.size / 2.0, self.pos.y.signum() * self.size / 2.0);

        if center.x > screen_x_bounds || center.x < -screen_x_bounds {
            let normal = Vector2::new(sy, 0.0);
            self.vel = reflect_vector(self.vel, normal);
        } else if center.y > screen_y_bounds || center.y < -screen_y_bounds {
            let normal = Vector2::new(0.0, sx);
            self.vel = reflect_vector(self.vel, normal);
        };
    }
}

impl Default for Body {
    fn default() -> Self {
        use nalgebra;

        Body {
            pos: Point2::origin(),
            vel: nalgebra::zero(),
            rot: 0.0,
            rvel: 0.0,
            size: 0.0,
        }
    }
}