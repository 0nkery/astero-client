use ggez::{
    graphics,
    Context,
    GameResult,
};

use body::Body;
use proto::astero;

use ::Movable;
use ::Destroyable;


pub struct Shot {
    body: Body,
    ttl: f32,
}


impl Shot {
    pub fn new(shot: &astero::Shot) -> Self {
        Self {
            body: Body::new(&shot.body),
            ttl: shot.ttl,
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets, world_coords: (u32, u32)) -> GameResult<()> {
        let (sw, sh) = world_coords;

        graphics::draw_ex(
            ctx,
            assets.shot_image(),
            graphics::DrawParam {
                dest: world_to_screen_coords(sw, sh, self.body.pos),
                offset: graphics::Point2::new(0.5, 0.5),
                .. Default::default()
            }
        )?;

        Ok(())
    }
}

impl Movable for Shot {
    fn update_position(&mut self, dt: f32) {
        self.ttl -= dt;
        self.body.update_position(dt);
    }

    fn wrap_position(&mut self, _xb: f32, _yb: f32) {}

    fn get_body(&self) -> &Body {
        &self.body
    }
}

impl Destroyable for Shot {
    fn life(&self) -> f32 {
        self.ttl
    }

    fn damage(&mut self, _amount: f32) {}

    fn destroy(&mut self) {
        self.ttl = 0.0;
    }
}