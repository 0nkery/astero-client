pub mod colors {
    use ggez::graphics::Color;

    pub const RED: Color = Color {
        r: 253.0 / 255.0,
        g: 112.0 / 255.0,
        b: 119.0 / 255.0,
        a: 200.0 / 255.0,
    };

    pub const LIGHT_BLUE: Color = Color {
        r: 126.0 / 255.0,
        g: 203.0 / 255.0,
        b: 210.0 / 255.0,
        a: 127.0 / 255.0
    };

    pub const GREEN: Color = Color {
        r: 162.0 / 255.0,
        g: 215.0 / 255.0,
        b: 41.0 / 255.0,
        a: 200.0 / 255.0,
    };
}

// TODO: move to server
//pub const PLAYER_LIFE: f32 = 3.0;
//pub const ROCK_LIFE: f32 = 2.0;

pub const PLAYER_ACCELERATION: f32 = 60.0;
pub const PLAYER_DECELERATION: f32 = 10.0;

pub mod physics {
    pub const MAX_VEL: f32 = 250.0;
    pub const DELTA_TIME: f64 = 0.01;
}

pub mod hud {
    pub const HEALTH_BAR_SIZE: f32 = 30.0;
    pub const STATIC_HEALTH_BAR_LINE_WIDTH: f32 = 4.0;
    pub const STICKY_HEALTH_BAR_HEIGHT: f32 = 3.0;
}
