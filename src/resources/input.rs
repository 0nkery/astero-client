use std::collections::VecDeque;

use ggez::event::Keycode;

use proto::astero;


#[derive(Clone)]
pub struct Input {
    turn: i32,
    accel: i32,
    fire: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            turn: 0,
            accel: 0,
            fire: false,
        }
    }

    pub fn key_down(&mut self, btn: Keycode, repeat: bool) -> Option<astero::Input> {
        if repeat {
            return None;
        }

        let old_input = self.clone();

        match btn {
            Keycode::W | Keycode::Up => {
                self.accel = 1;
            },
            Keycode::S | Keycode::Down => {
                self.accel = -1;
            },
            Keycode::A | Keycode::Left => {
                self.turn = -1;
            },
            Keycode::D | Keycode::Right => {
                self.turn = 1;
            },
            Keycode::Space => {
                self.fire = true;
            },

            _ => {
                return None;
            }
        }

        Some(self.diff(old_input))
    }

    pub fn key_up(&mut self, btn: Keycode) -> Option<astero::Input> {
        let old_input = self.clone();

        match btn {
            Keycode::W | Keycode::Up | Keycode::S | Keycode::Down => {
                self.accel = 0;
            },
            Keycode::A | Keycode::Left | Keycode::D | Keycode::Right=> {
                self.turn = 0;
            },
            Keycode::Space => {
                self.fire = false;
            },
            _ => {
                return None;
            }
        }

        Some(self.diff(old_input))
    }

    fn diff(
        &self,
        Self {
            turn,
            accel,
            fire
        }: Self
    ) -> astero::Input {
        let mut msg = astero::Input::default();

        if self.turn != turn {
            msg.turn = Some(self.turn);
        }
        if self.accel != accel {
            msg.accel = Some(self.accel);
        }
        if self.fire != fire {
            msg.fire = Some(self.fire);
        }

        msg
    }
}


pub struct PendingInput {
    sequence_number: u32,
    pub full_update_steps: u32,
    pub input: Input,
}

pub struct InputBuffer {
    buf: VecDeque<PendingInput>,
    sequence_number: u32,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buf: VecDeque::new(),
            sequence_number: 0,
        }
    }

    pub fn add(&mut self, input: Input) -> u32 {
        self.sequence_number = self.sequence_number.wrapping_add(1);

        self.buf.push_back(PendingInput {
            sequence_number: self.sequence_number,
            full_update_steps: 0,
            input,
        });

        self.sequence_number
    }

    pub fn increase_update_step(&mut self) {
        self.buf.back_mut().and_then(|pending| {
            pending.full_update_steps += 1;

            Some(())
        });
    }

    pub fn get_state_after(&mut self, sequence_number: u32) -> impl Iterator<Item=&PendingInput> {
        while self.buf[0].sequence_number <= sequence_number {
            self.buf.pop_front();
        }

        self.buf.iter()
    }
}
