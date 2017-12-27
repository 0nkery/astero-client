use ggez::graphics::{Point2, Vector2};
use ggez::nalgebra;

use util::reflect_vector;

use constant::MAX_PHYSICS_VEL;

use super::proto_defs::astero::{
    Body as ProtoBody,
};


#[derive(Debug)]
pub struct BodyError {
    pub pos_error: Point2,
    pub rot_error: f32,
}

impl BodyError {
    pub fn add(&mut self, other: BodyError) {
        self.pos_error.x += other.pos_error.x;
        self.pos_error.y += other.pos_error.y;
        self.rot_error += other.rot_error;
    }

    pub fn reduce(&mut self) {
        self.rot_error *= 0.9;
        if self.rot_error < 0.00001 {
            self.rot_error = 0.0;
        }
        if self.pos_error.x > 3.0 {
            self.pos_error.x *= 0.85;
        } else {
            self.pos_error.x *= 0.95;
        }

        if self.pos_error.y > 3.0 {
            self.pos_error.y *= 0.85;
        } else {
            self.pos_error.y *= 0.95;
        }
    }
}

impl Default for BodyError {
    fn default() -> Self {
        BodyError {
            pos_error: Point2::origin(),
            rot_error: 0.0,
        }
    }
}


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

    pub fn update(&mut self, body: &ProtoBody) -> BodyError {
        let mut error = BodyError::default();

        let new_pos: Point2 = body.pos.into();
        error.pos_error.x = self.pos.x - new_pos.x;
        error.pos_error.y = self.pos.y - new_pos.y;

        self.pos = new_pos;

        self.vel = body.vel.into();
        self.rvel = body.rvel.unwrap_or(self.rvel);
        self.size = body.size.unwrap_or(self.size);

        if let Some(new_rot) = body.rot {
            error.rot_error = self.rot - new_rot;
            self.rot = new_rot;
        }

        error
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

        let (nx, ny) = match center {
            _ if center.x > screen_x_bounds => (-1.0, 0.0),
            _ if center.x < -screen_x_bounds => (1.0, 0.0),
            _ if center.y > screen_y_bounds => (0.0, -1.0),
            _ if center.y < -screen_y_bounds => (0.0, 1.0),
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
        Body {
            pos: Point2::origin(),
            vel: nalgebra::zero(),
            rot: 0.0,
            rvel: 0.0,
            size: 0.0,
        }
    }
}