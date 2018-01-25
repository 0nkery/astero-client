use specs;

use resources;
use components;


pub struct Interpolation;

impl<'a> specs::System<'a> for Interpolation {
    type SystemData = (
        specs::Fetch<'a, resources::CurrentSystemRunMode>,
        specs::WriteStorage<'a, components::Body>,
        specs::WriteStorage<'a, components::InterpolationBuffer>,
    );

    fn run(&mut self, (run_mode, mut bodies, mut interp_buffers): Self::SystemData) {
        use specs::Join;

        if let resources::SystemRunMode::Interpolation(render_timestamp, _blending_factor) = run_mode.0 {
            for (body, interp_buf) in (&mut bodies, &mut interp_buffers).join() {
                *body = interp_buf.interpolate(render_timestamp);
            }
        }
    }
}
