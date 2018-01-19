use std::collections::BTreeMap;

use ggez::{
    self,
    graphics,
};


#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SpriteKind {
    Player = 0,
    Asteroid = 1,
    Shot = 2,
}


pub struct Assets {
    sprites: BTreeMap<SpriteKind, graphics::Image>,
    font: graphics::Font,
    small_font: graphics::Font,
}

impl Assets {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let mut sprites = BTreeMap::new();

        sprites.insert(SpriteKind::Player, graphics::Image::new(ctx, "/player.png")?);
        sprites.insert(SpriteKind::Asteroid, graphics::Image::new(ctx, "/asteroid.png")?);
        sprites.insert(SpriteKind::Shot, graphics::Image::new(ctx, "/shot.png")?);

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;
        let small_font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 12)?;

        Ok(Self {
            sprites,
            font,
            small_font,
        })
    }

    pub fn get_sprite(&self, kind: &SpriteKind) -> &graphics::Image {
        self.sprites.get(kind).expect("Sprite not found")
    }
}