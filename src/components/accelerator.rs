#[derive(Component, Debug)]
pub struct Accelerator {
    pub accel: f32,
    pub decel: f32,
}

impl Accelerator {
    pub fn new(accel: f32, decel: f32) -> Self {
        Self {
            accel,
            decel
        }
    }
}