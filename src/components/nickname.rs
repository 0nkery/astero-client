use ggez::{
    graphics,
    Context,
    GameResult,
};

use super::Body;


#[derive(Component, Debug)]
pub struct Nickname {
    display: graphics::Text,
}

impl Nickname {
    pub fn new(ctx: &mut Context, nickname: &str, font: &graphics::Font) -> GameResult<Self> {
        let new = Self {
            display: graphics::Text::new(ctx, nickname, font)?
        };

        Ok(new)
    }

    pub fn draw(&self, ctx: &mut Context, body: &Body, color: graphics::Color) -> GameResult<()> {
        let dest = graphics::Point2::new(
            pos.x - (self.display.width() / 2) as f32,
            pos.y - body.size / 2.0 - self.display.height() as f32,
        );

        graphics::draw_ex(
            ctx,
            &self.display,
            graphics::DrawParam {
                dest,
                color: Some(color),
                .. Default::default()
            }
        )?;

        Ok(())
    }
}
