use specs;

use components;
use resources;


pub struct UnconfirmedShotCleanup;

impl<'a> specs::System<'a> for UnconfirmedShotCleanup {
    type SystemData = (
        specs::Fetch<'a, resources::CurrentSystemRunMode>,
        specs::Entities<'a>,
        specs::ReadStorage<'a, components::ShotNetworkId>,
    );

    fn run(&mut self, (run_mode, entities, shot_network_ids): Self::SystemData) {
        use specs::Join;

        if let resources::SystemRunMode::Reconciliation = run_mode.0 {
            for (entity, shot_network_id) in (&*entities, &shot_network_ids).join() {
                if !shot_network_id.1 {
                    entities.delete(entity)
                        .expect("Deleting old shot?!");
                }
            }
        }
    }
}
