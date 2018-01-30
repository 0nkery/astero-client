#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![cfg_attr(feature="clippy", warn(use_self))]
#![cfg_attr(feature="clippy", warn(wrong_pub_self_convention))]
#![cfg_attr(feature="clippy", warn(single_match_else))]
#![cfg_attr(feature="clippy", warn(similar_names))]
#![cfg_attr(feature="clippy", warn(result_unwrap_used))]
#![cfg_attr(feature="clippy", warn(result_map_unwrap_or_else))]
#![cfg_attr(feature="clippy", warn(replace_consts))]
#![cfg_attr(feature="clippy", warn(range_plus_one))]
#![cfg_attr(feature="clippy", warn(pub_enum_variant_names))]
#![cfg_attr(feature="clippy", warn(option_unwrap_used))]
#![cfg_attr(feature="clippy", warn(option_map_unwrap_or_else))]
#![cfg_attr(feature="clippy", warn(option_map_unwrap_or))]
#![cfg_attr(feature="clippy", warn(mutex_integer))]
#![cfg_attr(feature="clippy", warn(mut_mut))]
#![cfg_attr(feature="clippy", warn(int_plus_one))]
#![cfg_attr(feature="clippy", warn(if_not_else))]
#![cfg_attr(feature="clippy", warn(float_cmp_const))]
#![cfg_attr(feature="clippy", warn(filter_map))]
#![cfg_attr(feature="clippy", warn(fallible_impl_from))]
#![cfg_attr(feature="clippy", warn(enum_glob_use))]

#![feature(ip_constructors)]
#![feature(use_nested_groups)]
#![feature(entry_and_modify)]
#![feature(conservative_impl_trait)]
#![feature(match_default_bindings)]

extern crate ggez;
extern crate specs;
#[macro_use] extern crate specs_derive;

extern crate rand;
extern crate time;

extern crate futures;
extern crate tokio_core;

extern crate bytes;
extern crate prost;
#[macro_use] extern crate prost_derive;

use std::path;

use ggez::{
    Context, ContextBuilder, GameResult,
    conf,
    graphics,
    timer,
    event::*,
};

use specs::{
    World,
    DispatcherBuilder,
    Dispatcher,
};

mod components;
mod resources;
mod systems;

mod constant;
mod msg;
mod proto;
mod util;

use proto::astero;

// TODO: move to the server
//    fn collided<M: Movable>(&mut self, other: &M) -> bool {
//        let self_body = self.get_body();
//        let other_body = other.get_body();
//
//        let distance = ggez::nalgebra::distance(&self_body.pos, &other_body.pos);
//
//        distance < (self_body.size / 2.0 + other_body.size / 2.0)
//    }


struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,

    assets: resources::Assets,
    client: resources::Client,
    pending_inputs: resources::InputBuffer,

    time_acc: f32,
    last_server_update_timestamp: u64,
    player_id: i64,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        print_instructions();

        let mut world = World::new();

        world.add_resource(resources::Input::new());
        world.add_resource(resources::ServerClock::new());
        world.add_resource(resources::CurrentSystemRunMode);
        world.add_resource(resources::UnconfirmedShotId(None));

        world.register::<components::Sprite>();
        world.register::<components::Body>();
        world.register::<components::Nickname>();
        world.register::<components::Color>();
        world.register::<components::Life>();
        world.register::<components::StickyHealthBar>();
        world.register::<components::StaticHealthBar>();
        world.register::<components::Controllable>();
        world.register::<components::NetworkId>();
        world.register::<components::TimeToLive>();
        world.register::<components::Accelerator>();
        world.register::<components::InterpolationBuffer>();
        world.register::<components::BlenderBody>();
        world.register::<components::Cannon>();
        world.register::<components::ShotNetworkId>();

        let dispatcher = DispatcherBuilder::new()
            .add(systems::KinematicsPrediction, "KinematicsPrediction", &[])
            .add(systems::Interpolation, "Interpolation", &[])
            .add(systems::Shooting, "Shooting", &[])
            .build();

        let nickname = util::cur_user_name();

        let client = resources::Client::start();
        client.send(msg::Msg::JoinGame(nickname));
        println!("Connecting to server...");

        let s = Self {
            world,
            dispatcher,

            assets: resources::Assets::new(ctx)?,
            client,
            pending_inputs: resources::InputBuffer::new(),

            time_acc: 0.0,
            last_server_update_timestamp: 0,
            player_id: -1,
        };

        Ok(s)
    }

    fn world_to_screen_coords(&self, ctx: &Context, point: graphics::Point2) -> graphics::Point2 {
        let width = ctx.conf.window_mode.width as f32;
        let height = ctx.conf.window_mode.height as f32;
        let x = point.x + width / 2.0;
        let y = height - (point.y + height / 2.0);

        graphics::Point2::new(x, y)
    }

    fn update_input(&mut self, cur_input: resources::Input, maybe_update: Option<proto::astero::Input>) {
        use specs::Join;

        if let Some(mut update) = maybe_update {
            update.sequence_num = self.pending_inputs.add(cur_input);

            let cannons = self.world.read::<components::Cannon>();
            let bodies = self.world.read::<components::Body>();
            let maybe_player = (&cannons, &bodies).join().next();

            if let Some((cannon, body)) = maybe_player {
                if cannon.ready_to_fire() {
                    update.body_then = Some(body.clone().into());
                }

                {
                    let mut unconfirmed_shot_id = self.world.write_resource::<resources::UnconfirmedShotId>();
                    unconfirmed_shot_id.0 = Some(update.sequence_num);
                }
            }

            self.client.send(msg::Msg::ToServer(update.into()));
        }
    }

    fn handle_message(&mut self, ctx: &mut Context, msg: msg::Msg) -> GameResult<()> {
        use specs::Join;

        match msg {
            msg::Msg::JoinAck(cur_player) => {
                self.player_id = i64::from(cur_player.id);

                self.world.create_entity()
                    .with(components::Body::new(&cur_player.body))
                    .with(components::BlenderBody::new())
                    .with(components::Accelerator::new(
                        constant::physics::PLAYER_ACCELERATION,
                        constant::physics::PLAYER_DECELERATION
                    ))
                    .with(components::Cannon::new(constant::FIRE_TIMEOUT))
                    .with(components::Color(constant::colors::GREEN))
                    .with(components::Life::new(cur_player.life.expect("Got empty life from server")))
                    .with(components::StaticHealthBar::new(
                        10 as f32,
                        ctx.conf.window_mode.height as f32 - constant::hud::HEALTH_BAR_SIZE - 5.0,
                        (ctx.conf.window_mode.width / 2) as f32,
                        constant::hud::HEALTH_BAR_SIZE))
                    .with(components::StickyHealthBar {})
                    .with(components::Sprite(resources::SpriteKind::Player))
                    .with(components::Nickname::new(
                        ctx,
                        &cur_player.nickname.expect("Got empty nickname from server"),
                        &self.assets.small_font)?)
                    .with(components::Controllable {})
                    .with(components::NetworkId(cur_player.id))
                    .build();
            },

            msg::Msg::FromServer(msg) => {
                match msg {
                    astero::server::Msg::Create(create) => {
                        let entity = create.entity.expect("Got empty create entity from server");

                        match entity {
                            astero::create::Entity::Player(other) => {
                                self.world.create_entity()
                                    .with(components::Body::new(&other.body))
                                    .with(components::Color(constant::colors::RED))
                                    .with(components::Life::new(other.life.expect("Got empty life from server")))
                                    .with(components::StickyHealthBar {})
                                    .with(components::Sprite(resources::SpriteKind::Player))
                                    .with(components::Nickname::new (
                                        ctx,
                                        &other.nickname.expect("Got empty nickname from server"),
                                        &self.assets.small_font
                                    )?)
                                    .with(components::NetworkId(other.id))
                                    .with(components::InterpolationBuffer::new())
                                    .build();
                            }
                            astero::create::Entity::Asteroid(ref asteroid) => {
                                self.world.create_entity()
                                    .with(components::Body::new(&asteroid.body))
                                    .with(components::Color(constant::colors::RED))
                                    .with(components::Life::new(asteroid.life.expect("Got empty life from server")))
                                    .with(components::StickyHealthBar {})
                                    .with(components::Sprite(resources::SpriteKind::Asteroid))
                                    .with(components::NetworkId(asteroid.id))
                                    .with(components::InterpolationBuffer::new())
                                    .build();
                            }
                            astero::create::Entity::Shot(ref shot) => {
                                self.world.create_entity()
                                    .with(components::Body::new(&shot.body))
                                    .with(components::Sprite(resources::SpriteKind::Shot))
                                    .with(components::TimeToLive::new(shot.ttl))
                                    .build();
                            }
                        }
                    }
                    astero::server::Msg::Destroy(entity_to_destroy) => {
                        let entity = {
                            let entities = self.world.entities();
                            let network_ids = self.world.read::<components::NetworkId>();

                            (&*entities, &network_ids).join()
                                .find(|&(_entity, network_id)| entity_to_destroy.id == network_id.0)
                                .map(|(entity, _network_id)| entity)
                        };

                        if let Some(entity) = entity {
                            self.world.delete_entity(entity)
                                .expect("Deleting already deleted entity?!");
                        }
                    },
                    astero::server::Msg::List(updates) => {
                        if updates.timestamp < self.last_server_update_timestamp {
                            return Ok(());
                        }
                        self.last_server_update_timestamp = updates.timestamp;

                        let entities = self.world.entities();
                        let network_ids = self.world.read::<components::NetworkId>();
                        let mut bodies = self.world.write::<components::Body>();
                        let mut interp_buffers = self.world.write::<components::InterpolationBuffer>();

                        for (ent, network_id, ) in (&*entities, &network_ids, ).join() {
                            let maybe_update = updates.updates.get(&network_id.0);

                            if let Some(update) = maybe_update {
                                let entity = update.entity.as_ref().expect("Got empty entity update from server");

                                match entity {
                                    astero::update::Entity::Player(player)
                                    if self.player_id == i64::from(player.id) => {
                                        let maybe_body = bodies.get_mut(ent);
                                        if let Some(body) = maybe_body {
                                            *body = components::Body::new(&player.body);

                                            {
                                                let mut cur_sys_run_mode = self.world.write_resource::<resources::CurrentSystemRunMode>();
                                                cur_sys_run_mode.0 = resources::SystemRunMode::Reconciliation;
                                            }

                                            if let Some(current_fire_timeout) = player.current_fire_timeout {
                                                let server_clock = self.world.read_resource::<resources::ServerClock>();
                                                let corrected_timeout = current_fire_timeout - server_clock.compensation() as f32;

                                                let mut cannons = self.world.write::<components::Cannon>();
                                                for (cannon, ) in (&mut cannons, ).join() {
                                                    cannon.set_current_timeout(corrected_timeout);
                                                }
                                            }

                                            let last_handled_input = player.last_handled_input
                                                .expect("Got empty last handled input from server");

                                            if !player.shot_confirmed.unwrap_or(true) {
                                                let mut unconfirmed_shots = self.world.write::<components::ShotNetworkId>();
                                                (&mut unconfirmed_shots, ).join()
                                                    .filter(|(shot_id, )| shot_id.0 == last_handled_input)
                                                    .for_each(|(shot_id, )| shot_id.1 = false);
                                            }

                                            for pending in self.pending_inputs.get_state_after(last_handled_input) {
                                                {
                                                    let mut input = self.world.write_resource::<resources::Input>();
                                                    *input = pending.input.clone();
                                                }

                                                for _ in 0..pending.full_update_steps {
                                                    self.dispatcher.dispatch(&self.world.res);
                                                }
                                            }
                                        }
                                    }

                                    astero::update::Entity::Player(player) => {
                                        let interp_buf = interp_buffers.get_mut(ent)
                                            .expect("No interpolation buffer attached to remote player?!");
                                        interp_buf.add(&player.body);
                                    }
                                    astero::update::Entity::Asteroid(asteroid) => {
                                        let interp_buf = interp_buffers.get_mut(ent)
                                            .expect("No interpolation buffer attached to remote asteroid?!");
                                        interp_buf.add(&asteroid.body);
                                    }
                                }
                            }
                        }
                    },
                }
            }

            msg::Msg::ServerNotResponding => {
                println!("Server is not available! Closing game...");
                ctx.quit()?;
            }

            msg::Msg::Latency(ref measure) => {
                let mut server_clock = self.world.write_resource::<resources::ServerClock>();
                server_clock.update(
                    measure.timestamp,
                    measure.server_timestamp.expect("Got empty server timestamp")
                );
            },

            msg::Msg::Unknown | msg::Msg::JoinGame(..) | msg::Msg::LeaveGame |
            msg::Msg::Heartbeat | msg::Msg::ToServer(..) => unreachable!(),
        }

        Ok(())
    }
}

fn print_instructions() {
    println!();
    println!("Welcome to Astero!");
    println!();
    println!("How to play:");
    println!("L/R arrow keys rotate ship, up thrusts, down slows down, space bar fires");
    println!();
}

impl<'a, 'b> EventHandler for MainState<'a, 'b> {

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while let Ok(msg) = self.client.try_recv() {
            self.handle_message(ctx, msg)?;
        }

        let frame_time = timer::get_delta(ctx);
        let frame_time = timer::duration_to_f64(frame_time) as f32;

        {
            let mut cur_system_run_mode = self.world.write_resource::<resources::CurrentSystemRunMode>();
            cur_system_run_mode.0 = resources::SystemRunMode::Prediction;
        }

        self.time_acc += frame_time;

        while self.time_acc > constant::physics::DELTA_TIME {
            self.dispatcher.dispatch(&self.world.res);
            self.pending_inputs.increase_update_step();
            self.time_acc -= constant::physics::DELTA_TIME;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;

        graphics::clear(ctx);

        {
            let mut cur_sys_run_mode = self.world.write_resource::<resources::CurrentSystemRunMode>();
            cur_sys_run_mode.0 = resources::SystemRunMode::Interpolation(
                util::cur_time_in_millis() - 1000 / 30,
                self.time_acc / constant::physics::DELTA_TIME
            );
        }

        self.dispatcher.dispatch(&self.world.res);

        let entities = self.world.entities();
        let bodies = self.world.read::<components::Body>();
        let blend_bodies = self.world.read::<components::BlenderBody>();
        let sprites = self.world.read::<components::Sprite>();

        for (ent, body, sprite) in (&*entities, &bodies, &sprites).join() {
            let sprite = self.assets.get_sprite(&sprite.0);

            let pos = blend_bodies.get(ent)
                .and_then(|bb| bb.get_blended())
                .and_then(|b| Some(b.pos))
                .unwrap_or(body.pos);
            let pos = self.world_to_screen_coords(ctx,pos);

            graphics::draw_ex(ctx, sprite, graphics::DrawParam {
                dest: pos,
                rotation: body.rot,
                offset: graphics::Point2::new(0.5, 0.5),
                scale: graphics::Point2::new(
                    body.size / sprite.width() as f32,
                    body.size / sprite.height() as f32
                ),
                ..Default::default()
            })?;
        }

        let nicknames = self.world.read::<components::Nickname>();
        let colors = self.world.read::<components::Color>();

        for (body, nickname, color) in (&bodies, &nicknames, &colors).join() {
            let pos = self.world_to_screen_coords(ctx, body.pos);
            nickname.draw(ctx, pos, body.size, color.0)?;
        }

        let lives = self.world.read::<components::Life>();
        let sticky_health_bars = self.world.read::<components::StickyHealthBar>();

        for (body, life, color, sticky_health_bar) in (&bodies, &lives, &colors, &sticky_health_bars).join() {
            let pos = self.world_to_screen_coords(ctx, body.pos);
            sticky_health_bar.draw(ctx, pos, body.size, life.fraction(), color.0)?;
        }

        let static_health_bars = self.world.read::<components::StaticHealthBar>();

        for (life, static_health_bar) in (&lives, &static_health_bars).join() {
            static_health_bar.draw(ctx, life.fraction())?;
        }

        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if let Keycode::Escape = keycode {
            ctx.quit().expect("Failed to quit the game");
            return;
        }

        let (input, maybe_update) = {
            let mut input = self.world.write_resource::<resources::Input>();
            let maybe_update = input.key_down(keycode, repeat);

            (input.clone(), maybe_update)
        };

        self.update_input(input, maybe_update);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let (input, maybe_update) = {
            let mut input = self.world.write_resource::<resources::Input>();
            let maybe_update = input.key_up(keycode);

            (input.clone(), maybe_update)
        };

        self.update_input(input, maybe_update);
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        self.client.stop();

        false
    }
}

fn main() {
    let mut cb = ContextBuilder::new("Astero", "onkery")
        .window_setup(conf::WindowSetup::default().title("Astero"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600).vsync(true));

    let mut path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets");
    cb = cb.add_resource_path(path);

    let ctx = &mut cb.build().expect("Failed to build game context");

    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load the game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
    }
}
