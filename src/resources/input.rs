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

            _ => {}
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
            _ => {}
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