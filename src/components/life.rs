#[derive(Component, Debug)]
pub struct Life {
    cur: f32,
    max: f32,
}

impl Life {
    pub fn new(max: f32) -> Self {
        Self {
            cur: max,
            max,
        }
    }

    pub fn fraction(&self) -> f32 {
        self.cur / self.max
    }
}
