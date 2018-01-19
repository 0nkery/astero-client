use ggez::{
    self,
    graphics,
};


pub struct Assets {
    player_image: graphics::Image,
    shot_image: graphics::Image,
    rock_image: graphics::Image,
    font: graphics::Font,
    small_font: graphics::Font,
}

impl Assets {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;
        let rock_image = graphics::Image::new(ctx, "/rock.png")?;

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;
        let small_font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 12)?;

        Ok(Self {
            player_image,
            shot_image,
            rock_image,
            font,
            small_font,
        })
    }

    fn shot_image(&self) -> &graphics::Image {
        &self.shot_image
    }

    fn asteroid_image(&self) -> &graphics::Image {
        &self.rock_image
    }

    fn player_image(&self) -> &graphics::Image {
        &self.player_image
    }
}