use util::cur_time_in_millis;


pub struct ServerClock {
    latency: i32,
    delta_time: i32,
}

impl ServerClock {
    pub fn new() -> Self {
        Self {
            latency: 0,
            delta_time: 0,
        }
    }

    pub fn update(&mut self, then: u64, server_timestamp: u64) {
        let now = cur_time_in_millis();

        self.delta_time = (server_timestamp - now) as i32;
        self.latency = ((now - then) / 2) as i32;
    }

    pub fn compensation(&self) -> i32 {
        self.latency + self.delta_time
    }
}