use ggez::{
    graphics,
    Context,
    GameResult,
};

use constant::hud::STICKY_HEALTH_BAR_HEIGHT;


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
                pos.y + size / 2.0 + STICKY_HEALTH_BAR_HEIGHT,
                width, STICKY_HEALTH_BAR_HEIGHT
            )
        )?;

        graphics::set_color(ctx, old_color)?;

        Ok(())
    }
}