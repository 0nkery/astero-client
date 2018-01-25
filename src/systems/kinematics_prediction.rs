use specs;

use constant::physics;
use components;
use resources;


pub struct KinematicsPrediction;

impl<'a> specs::System<'a> for KinematicsPrediction {
    type SystemData = (
        specs::Fetch<'a, resources::Input>,
        specs::WriteStorage<'a, components::Body>,
        specs::ReadStorage<'a, components::Controllable>,
        specs::ReadStorage<'a, components::Accelerator>,
    );

    fn run(&mut self, (input, mut bodies, controllable, accelerators): Self::SystemData) {
        use specs::Join;

        for (_cntrl, body, accel) in (&controllable, &mut bodies, &accelerators).join() {
            body.accelerate(physics::DELTA_TIME, input.accel, accel.accel, accel.decel);

            body.update_position(physics::DELTA_TIME);
            body.rotate(physics::DELTA_TIME, input.turn);
            // TODO: move these literals to resource
            body.wrap_position(400.0, 300.0);
        }
    }
}
