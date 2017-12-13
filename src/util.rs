use std;

use nalgebra;
use rand;


pub type Point2 = nalgebra::Point2<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;


pub fn vec_from_angle(angle: f32) -> Vector2 {
    Vector2::new(angle.sin(), angle.cos())
}

pub fn random_vec(max_magnitude: f32) -> Vector2 {
    let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    let mag = rand::random::<f32>() * max_magnitude;
    vec_from_angle(angle) * (mag)
}