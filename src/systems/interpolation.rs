use specs;

use resources;
use components;


pub struct Interpolation;

impl<'a> specs::System<'a> for Interpolation {
    type SystemData = (
        specs::Fetch<'a, resources::CurrentSystemRunMode>,
        specs::WriteStorage<'a, components::Body>,
        specs::WriteStorage<'a, components::InterpolationBuffer>,
        specs::WriteStorage<'a, components::BlenderBody>,
    );

    fn run(&mut self, (run_mode, mut bodies, mut interp_buffers, mut blend_bodies): Self::SystemData) {
        use specs::Join;

        if let resources::SystemRunMode::Interpolation(render_timestamp, blending_factor) = run_mode.0 {
            for (body, interp_buf) in (&mut bodies, &mut interp_buffers).join() {
                *body = interp_buf.interpolate(render_timestamp);
            }

            for (body, blend_body) in (&bodies, &mut blend_bodies).join() {
                blend_body.blend(body, blending_factor);
            }
        }
    }
}
