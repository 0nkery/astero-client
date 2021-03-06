use std;
use ggez::graphics::{Vector2, Point2};
use time;

use proto::astero::Coord;


impl Copy for Coord {}

impl Into<Point2> for Coord {
    fn into(self) -> Point2 {
        Point2::new(self.x, self.y)
    }
}

impl From<Point2> for Coord {
    fn from(point: Point2) -> Self {
        Self { x: point.x, y: point.y }
    }
}

impl Into<Vector2> for Coord {
    fn into(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl From<Vector2> for Coord {
    fn from(vec: Vector2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

pub fn vec_from_angle(angle: f32) -> Vector2 {
    Vector2::new(angle.sin(), angle.cos())
}

pub fn reflect_vector(vec: Vector2, normal: Vector2) -> Vector2 {
    // |normal| ^ 2 == 1.0
    assert!(
        (normal.norm_squared() == 0.0) ||
        (normal.norm_squared() - 1.0).abs() < std::f32::EPSILON
    );

    vec - 2.0 * normal * vec.dot(&normal)
}

pub fn cur_time_in_millis() -> u64 {
    let timespec = time::get_time();
    let millis = timespec.sec * 1000 + (i64::from(timespec.nsec) / 1000 / 1000);

    millis as u64
}

pub fn cur_user_name() -> String {
    std::env::home_dir()
        .expect("Failed to retrieve home dir")
        .as_path()
        .file_name()
        .expect("Failed to retrieve username")
        .to_str()
        .expect("Failed to convert username to Unicode")
        .to_string()
}
