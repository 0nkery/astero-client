use specs;

use constant::physics;
use components;
use resources;


pub struct KinematicsPrediction;

impl<'a> specs::System<'a> for KinematicsPrediction {
    #[allow(type_complexity)]
    type SystemData = (
        specs::Fetch<'a, resources::CurrentSystemRunMode>,
        specs::Fetch<'a, resources::Input>,
        specs::WriteStorage<'a, components::Body>,
        specs::WriteStorage<'a, components::BlenderBody>,
        specs::ReadStorage<'a, components::Controllable>,
        specs::ReadStorage<'a, components::Accelerator>,
    );

    fn run(&mut self, (run_mode, input, mut bodies, mut blend_bodies, controllable, accelerators): Self::SystemData) {
        if let resources::SystemRunMode::Interpolation(..) = run_mode.0 {
            return;
        }

        use specs::Join;

        for (_cntrl, body, blend_body, accel) in (&controllable, &mut bodies, &mut blend_bodies, &accelerators).join() {
            blend_body.save(body.clone());

            body.accelerate(physics::DELTA_TIME, input.accel, accel.accel, accel.decel);

            body.update_position(physics::DELTA_TIME);
            body.rotate(physics::DELTA_TIME, input.turn);
            // TODO: move these literals to resource
            body.wrap_position(400.0, 300.0);
        }
    }
}
