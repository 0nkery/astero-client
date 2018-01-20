mod body;
mod color;
mod health_bar;
mod life;
mod nickname;
mod sprite;

pub use self::body::Body;
pub use self::color::Color;
pub use self::{
    health_bar::Sticky as StickyHealthBar,
    health_bar::Static as StaticHealthBar,
};
pub use self::life::Life;
pub use self::nickname::Nickname;
pub use self::sprite::Sprite;
