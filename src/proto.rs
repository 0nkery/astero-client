pub mod mmob {
    include!(concat!(env!("OUT_DIR"), "/mmob.rs"));
}

pub mod astero {
    include!(concat!(env!("OUT_DIR"), "/astero.rs"));
}

impl From<astero::Input> for astero::client::Msg {
    fn from(input: astero::Input) -> Self {
        astero::client::Msg::Input(input)
    }
}

impl astero::Input {
    pub fn is_empty(&self) -> bool {
        self.turn.is_none()
        && self.accel.is_none()
        && self.fire.is_none()
    }
}
