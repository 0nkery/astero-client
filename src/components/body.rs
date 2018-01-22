use ggez::graphics::{
    Point2,
    Vector2,
};

use proto::astero;


#[derive(Component, Debug)]
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
}