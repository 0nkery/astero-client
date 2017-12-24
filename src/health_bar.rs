use ggez::{
    Context,
    GameResult,
    graphics
};

use constant::colors;
use constant::gui::STATIC_HEALTH_BAR_LINE_WIDTH;


pub struct StaticHealthBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl StaticHealthBar {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        StaticHealthBar {x, y, width, height}
    }

    pub fn draw(&self, ctx: &mut Context, cur: f32, max: f32) -> GameResult<()> {
        let old_color = graphics::get_color(ctx);

        graphics::set_color(ctx, colors::LIGHT_BLUE)?;

        graphics::rectangle(
            ctx, graphics::DrawMode::Line(STATIC_HEALTH_BAR_LINE_WIDTH),
            graphics::Rect::new(self.x, self.y, self.width, self.height)
        )?;

        graphics::set_color(ctx, colors::RED)?;

        let health_bar_width = (self.width - STATIC_HEALTH_BAR_LINE_WIDTH) * (cur / max);
        let health_bar_height = self.height - STATIC_HEALTH_BAR_LINE_WIDTH;

        graphics::rectangle(
            ctx, graphics::DrawMode::Fill,
            graphics::Rect::new(self.x, self.y, health_bar_width, health_bar_height)
        )?;

        graphics::set_color(ctx, old_color)?;

        Ok(())
    }
}


pub struct StickyHealthBar;

const STICKY_HEIGHT: f32 = 3.0;

impl StickyHealthBar {
    pub fn draw(
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        cur: f32,
        max: f32,
        color: Option<graphics::Color>
    ) -> GameResult<()> {

        let old_color = graphics::get_color(ctx);
        graphics::set_color(ctx, color.or(Some(colors::RED)).unwrap())?;

        let width = width * (cur / max);

        graphics::rectangle(
            ctx, graphics::DrawMode::Fill,
            graphics::Rect::new(x, y, width, STICKY_HEIGHT)
        )?;

        graphics::set_color(ctx, old_color)?;

        Ok(())
    }
}