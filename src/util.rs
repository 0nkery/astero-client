use ggez::graphics::{Vector2, Point2};

use proto::astero::Coord;


impl Copy for Coord {}

impl Into<Point2> for Coord {
    fn into(self) -> Point2 {
        Point2::new(self.x, self.y)
    }
}

impl From<Point2> for Coord {
    fn from(point: Point2) -> Self {
        Coord { x: point.x, y: point.y }
    }
}

impl Into<Vector2> for Coord {
    fn into(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl From<Vector2> for Coord {
    fn from(vec: Vector2) -> Self {
        Coord { x: vec.x, y: vec.y }
    }
}


pub fn vec_from_angle(angle: f32) -> Vector2 {
    Vector2::new(angle.sin(), angle.cos())
}

pub fn world_to_screen_coords(screen_width: u32, screen_height: u32, point: Point2) -> Point2 {
    let width = screen_width as f32;
    let height = screen_height as f32;
    let x = point.x + width / 2.0;
    let y = height - (point.y + height / 2.0);
    Point2::new(x, y)
}

pub fn reflect_vector(vec: Vector2, normal: Vector2) -> Vector2 {
    vec - 2.0 * normal * vec.dot(&normal)
}
