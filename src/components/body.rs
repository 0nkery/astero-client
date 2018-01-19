use ggez::graphics::{
    Point2,
    Vector2,
};


#[derive(Component, Debug)]
pub struct Body {
    pub pos: Point2,
    pub vel: Vector2,
    pub rot: f32,
    pub rvel: f32,
    pub size: f32,
}