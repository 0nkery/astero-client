use std::collections::VecDeque;

use components::Body;
use proto::astero;
use util::cur_time_in_millis;


#[derive(Debug)]
pub struct InterpolationPosition {
    pub timestamp: u64,
    pub body: Body,
}


#[derive(Component, Debug)]
pub struct InterpolationBuffer {
    buf: VecDeque<InterpolationPosition>,
}

impl InterpolationBuffer {
    pub fn new() -> Self {
        Self {
            buf: VecDeque::new(),
        }
    }

    pub fn add(&mut self, body: &astero::Body) {
        self.buf.push_back(InterpolationPosition {
            timestamp: cur_time_in_millis(),
            body: Body::new(body),
        })
    }

    pub fn interpolate(&mut self, timestamp: u64) -> Body {
        while self.buf.len() >= 2 && self.buf[1].timestamp <= timestamp {
            self.buf.pop_front();
        }

        let dt = (timestamp - self.buf[0].timestamp) as f32 / (self.buf[1].timestamp - self.buf[0].timestamp) as f32;

        let mut body = self.buf[0].body.clone();
        body.interpolate_to(&self.buf[1].body, dt);

        body
    }
}
