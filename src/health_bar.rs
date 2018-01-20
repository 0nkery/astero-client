use ggez::{
    Context,
    GameResult,
    graphics
};

use constant::colors;
use constant::hud::STATIC_HEALTH_BAR_LINE_WIDTH;


pub struct Static {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl Static {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {x, y, width, height}
    }

    pub fn draw(&self, ctx: &mut Context, fraction: f32) -> GameResult<()> {
        let old_color = graphics::get_color(ctx);

        graphics::set_color(ctx, colors::LIGHT_BLUE)?;

        graphics::rectangle(
            ctx, graphics::DrawMode::Line(STATIC_HEALTH_BAR_LINE_WIDTH),
            graphics::Rect::new(self.x, self.y, self.width, self.height)
        )?;

        graphics::set_color(ctx, colors::RED)?;

        let width = (self.width - STATIC_HEALTH_BAR_LINE_WIDTH) * fraction;
        let height = self.height - STATIC_HEALTH_BAR_LINE_WIDTH;

        let half_line_width = STATIC_HEALTH_BAR_LINE_WIDTH / 2.0;

        graphics::rectangle(
            ctx, graphics::DrawMode::Fill,
            graphics::Rect::new(
                self.x + self.width / 2.0 - width / 2.0,
                self.y + half_line_width,
                width,
                height
            )
        )?;

        graphics::set_color(ctx, old_color)?;

        Ok(())
    }
}


pub struct Sticky;

const STICKY_HEIGHT: f32 = 3.0;

impl Sticky {
    pub fn draw(
        ctx: &mut Context,
        pos: graphics::Point2,
        size: f32,
        fraction: f32,
        color: Option<graphics::Color>
    ) -> GameResult<()> {

        let old_color = graphics::get_color(ctx);
        graphics::set_color(ctx, color.unwrap_or(colors::RED))?;

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