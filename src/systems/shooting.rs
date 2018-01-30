use specs;

use components;
use constant;
use resources;
use util;


pub struct Shooting;

impl Shooting {
    fn create_shot_body(&self, from: components::Body) -> components::Body {
        let mut shot_body = from;
        shot_body.size = constant::shot::SIZE;

        let direction = util::vec_from_angle(shot_body.rot);
        shot_body.vel.x = constant::shot::SPEED * direction.x;
        shot_body.vel.y = constant::shot::SPEED * direction.y;

        shot_body
    }
}

impl<'a> specs::System<'a> for Shooting {
    type SystemData = (
        specs::Fetch<'a, resources::CurrentSystemRunMode>,
        specs::Fetch<'a, resources::Input>,
        specs::Fetch<'a, resources::UnconfirmedShotId>,
        specs::Entities<'a>,
        specs::WriteStorage<'a, components::Body>,
        specs::WriteStorage<'a, components::Cannon>,
        specs::WriteStorage<'a, components::Sprite>,
        specs::WriteStorage<'a, components::TimeToLive>,
        specs::WriteStorage<'a, components::ShotNetworkId>,
    );

    fn run(
        &mut self,
        (
            run_mode, input, unconfirmed_shot_id,
            entities,
            mut bodies, mut cannons, mut sprites, mut ttls, mut shot_network_ids,
        ): Self::SystemData
    ) {
        use specs::Join;

        if let resources::SystemRunMode::Prediction = run_mode.0 {
            for (cannon, entity) in (&mut cannons, &*entities).join() {
                cannon.update(constant::physics::DELTA_TIME);

                if cannon.ready_to_fire() && input.fire {
                    let shot = entities.create();
                    sprites.insert(shot, components::Sprite(resources::SpriteKind::Shot));
                    ttls.insert(shot, components::TimeToLive::new(constant::shot::TTL));

                    let body = {
                        bodies.get(entity)
                            .expect("Cannon attached to entity without body?!")
                            .clone()
                    };
                    bodies.insert(shot, self.create_shot_body(body));

                    if let Some(shot_id) = unconfirmed_shot_id.0 {
                        shot_network_ids.insert(shot, components::ShotNetworkId(shot_id, true));
                    }

                    cannon.reload();
                }
            }
        }
    }
}