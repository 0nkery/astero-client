use ggez::graphics::{Point2, Vector2};
use ggez::nalgebra;

use util::reflect_vector;

use constant::MAX_PHYSICS_VEL;

use proto::astero;


#[derive(Debug)]
pub struct Body {
    pub pos: Point2,
    pub vel: Vector2,
    pub rot: f32,
    pub rvel: f32,
    pub size: f32,
}

impl Body {
    pub fn new(body: &astero::Body) -> Self {
        Self {
            pos: body.pos.into(),
            vel: body.vel.into(),
            rot: body.rot.unwrap_or(0.0),
            rvel: body.rvel.unwrap_or(0.0),
            size: body.size.unwrap_or(0.0),
        }
    }

    pub fn update(&mut self, body: &astero::Body) {
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

    pub fn wrap_position(&mut self, xb: f32, yb: f32) {
        if self.size <= 0.0 {
            return;
        }

        let center = self.pos + Vector2::new(self.pos.x.signum() * self.size / 2.0, self.pos.y.signum() * self.size / 2.0);

        let (nx, ny) = match center {
            _ if center.x > xb => (-1.0, 0.0),
            _ if center.x < -xb => (1.0, 0.0),
            _ if center.y > yb => (0.0, -1.0),
            _ if center.y < -yb => (0.0, 1.0),
            _ => (0.0, 0.0)
        };
        let normal = Vector2::new(nx, ny);
        if normal.dot(&self.vel) <= 0.0 {
            self.vel = reflect_vector(self.vel, normal);
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self {
            pos: Point2::origin(),
            vel: nalgebra::zero(),
            rot: 0.0,
            rvel: 0.0,
            size: 0.0,
        }
    }
}