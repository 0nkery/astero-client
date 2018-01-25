use std;

use ggez::graphics::{
    Point2,
    Vector2,
};

use constant::physics;
use proto::astero;
use util::{
    reflect_vector,
    vec_from_angle,
};


#[derive(Component, Debug, Clone)]
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

    pub fn update_position(&mut self, dt: f32) {
        let norm_sq = self.vel.norm_squared();
        if norm_sq > physics::MAX_VEL.powi(2) {
            self.vel = self.vel / norm_sq.sqrt() * physics::MAX_VEL;
        }

        let dv = self.vel * dt;
        self.pos += dv;
    }

    pub fn rotate(&mut self, dt: f32, direction: i32) {
        self.rot += self.rvel * dt * direction as f32;
    }

    pub fn wrap_position(&mut self, xb: f32, yb: f32) {
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

    pub fn accelerate(&mut self, dt: f32, direction: i32, accel: f32, decel: f32) {
        if direction == 0 {
            return;
        }

        let (angle, accel_value) = if direction < 0 {
            (self.rot, accel)
        } else {
            (self.rot + std::f32::consts::PI, decel)
        };

        let dir_vec = vec_from_angle(angle);
        let acceleration = dir_vec * accel_value;
        self.vel += acceleration * dt;
    }

    pub fn interpolate_to(&mut self, other: &Self, by_dt: f32) {
        self.pos.x += (other.pos.x - self.pos.x) * by_dt;
        self.pos.y += (other.pos.y - self.pos.y) * by_dt;
        self.rot += (other.rot - self.rot) * by_dt;
    }
}