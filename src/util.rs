use ggez::graphics::{Vector2, Point2};


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
