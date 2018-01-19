use util::cur_time_in_millis;


pub struct ServerClock {
    latency: u64,
    delta_time: u64,
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

        self.delta_time = now - server_timestamp;
        self.latency = (now - then) / 2;
    }

    pub fn server_time(&self) -> u64 {
        self.latency + self.delta_time
    }
}