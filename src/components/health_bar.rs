use ggez::{
    graphics,
    Context,
    GameResult,
};


const STICKY_HEIGHT: f32 = 3.0;

pub struct Sticky;

impl Sticky {
    pub fn draw(
        &self,
        ctx: &mut Context,
        pos: graphics::Point2, size: f32, fraction: f32, color: graphics::Color
    ) -> GameResult<()> {
        let old_color = graphics::get_color(ctx);
        graphics::set_color(ctx, color)?;

        let width = size * fraction;

        graphics::rectangle(
            ctx, graphics::DrawMode::Fill,
            graphics::Rect::new(
                pos.x - width / 2.0,
                pos.y + size / 2.0 + STICKY_HEIGHT,
                width, STICKY_HEIGHT
            )
        )?;

        graphics::set_color(ctx, old_color)?;

        Ok(())
    }
}