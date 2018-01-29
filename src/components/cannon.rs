#[derive(Component, Debug)]
pub struct Cannon {
    cur_timeout: f32,
    max_timeout: f32,
}

impl Cannon {
    pub fn new(timeout: f32) -> Self {
        Self {
            cur_timeout: timeout,
            max_timeout: timeout,
        }
    }

    pub fn set_current_timeout(&mut self, timeout: f32) {
        self.cur_timeout = timeout;
    }

    pub fn update(&mut self, dt: f32) {
        self.cur_timeout -= dt;
    }

    pub fn ready_to_fire(&self) -> bool {
        self.cur_timeout <= 0.0
    }

    pub fn reload(&mut self) {
        self.cur_timeout = self.max_timeout;
    }
}
