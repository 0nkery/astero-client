#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![cfg_attr(feature="clippy", warn(use_self))]
#![cfg_attr(feature="clippy", warn(wrong_pub_self_convention))]
#![cfg_attr(feature="clippy", warn(stutter))]
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

use std::collections::HashMap;
use std::env;
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

mod body;
use body::Body;

mod player;
use player::Player;

mod asteroid;
use asteroid::Asteroid;

mod shot;
use shot::Shot;

mod components;
mod resources;
mod systems;

mod constant;
mod msg;
mod proto;
mod util;

use proto::{
    astero,
    astero::Entity,
};


trait Movable {
    fn update_position(&mut self, dt: f32);
    fn wrap_position(&mut self, sx: f32, sy: f32);
    fn get_body(&self) -> &Body;

    fn collided<M: Movable>(&mut self, other: &M) -> bool {
        let self_body = self.get_body();
        let other_body = other.get_body();

        let distance = ggez::nalgebra::distance(&self_body.pos, &other_body.pos);

        distance < (self_body.size / 2.0 + other_body.size / 2.0)
    }
}


trait Destroyable {
    fn is_dead(&self) -> bool {
        self.life() <= 0.0
    }

    fn is_alive(&self) -> bool {
        !self.is_dead()
    }

    fn life(&self) -> f32;
    fn damage(&mut self, amount: f32);
    fn destroy(&mut self);
}


struct MainState<'a, 'b> {
    player: Player,
    asteroids: HashMap<u32, Asteroid>,
    others: HashMap<u32, Player>,

    shots: Vec<Shot>,
    unconfirmed_shots: HashMap<u32, Shot>,

    client: resources::Client,

    world: World,
    dispatcher: Dispatcher<'a, 'b>,

    time_acc: f64,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        print_instructions();

        let mut world = World::new();

        world.add_resource(resources::Input::new());
        world.add_resource(resources::ServerClock::new());
        world.add_resource(resources::Assets::new(ctx)?);
        world.add_resource(resources::PlayerId(None));
        world.add_resource(resources::MsgQueue);

        world.register::<components::Sprite>();
        world.register::<components::Body>();
        world.register::<components::Nickname>();
        world.register::<components::Color>();
        world.register::<components::Life>();
        world.register::<components::StickyHealthBar>();
        world.register::<components::StaticHealthBar>();

        let dispatcher = DispatcherBuilder::new()
            .add(systems::Network, "Network", &[])
            .build();

        let home_dir = env::home_dir().expect("Failed to retrieve home dir");
        let nickname =
            home_dir
                .as_path()
                .file_name()
                .expect("Failed to retrieve username")
                .to_str()
                .expect("Failed to convert username to Unicode")
                .to_string();

        let player = Player::new(ctx, nickname.clone(), &assets.small_font, constant::colors::GREEN)?;

        let client = resources::Client::start();
        client.send(msg::Msg::JoinGame(nickname));
        println!("Connecting to server...");

        // TODO: move this code to init fn of current player
//        let health_bar = health_bar::Static::new(
//            10 as f32,
//            screen_height as f32 - constant::hud::HEALTH_BAR_SIZE - 5.0,
//            (screen_width / 2) as f32,
//            constant::hud::HEALTH_BAR_SIZE
//        );

        let s = Self {
            player,
            asteroids: HashMap::new(),
            others: HashMap::new(),

            shots: Vec::new(),
            unconfirmed_shots: HashMap::new(),

            client,

            world,
            dispatcher,
            time_acc: 0.0,
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

    fn handle_collisions(&mut self) {
        for rock in self.asteroids.values_mut() {

            if self.player.collided(rock) {
                self.player.damage(1.0);
                rock.destroy();
                continue;
            }

            for shot in &mut self.shots {
                if shot.collided(rock) {
                    shot.destroy();
                    rock.damage(1.0);
                }
            }
        }
    }

    fn init_player(&mut self, this: &proto::astero::Player) {
        // TODO: refactor this code into Network system
//        println!("Connected to server. Conn ID - {}", this.id);
//        self.player_id = this.id;
//        self.player.set_body(&this.body);
    }

    fn create_entity(&mut self, ctx: &mut Context, entity: astero::create::Entity) -> GameResult<()> {
        match entity {
            astero::create::Entity::Player(other) => {
                let nickname = other.nickname
                    .expect("Missing nickname on remote player");

                let mut player = Player::new(
                    ctx, nickname, &self.assets.small_font, constant::colors::RED
                )?;
                player.set_body(&other.body);
                self.others.insert(other.id, player);
            }
            astero::create::Entity::Asteroid(ref asteroid) => {
                self.asteroids.insert(asteroid.id, Asteroid::new(asteroid));
            }
            astero::create::Entity::Shot(ref shot) => {
                self.shots.push(Shot::new(shot));
            }
        }

        Ok(())
    }

    fn destroy_entity(&mut self, entity: &proto::astero::Destroy) {
        let kind = Entity::from_i32(entity.entity).expect("Missing entity on Destroy");
        match kind {
            Entity::Unknown => {},
            Entity::Player => {
                let player = self.others.remove(&entity.id);
                if let Some(player) = player {
                    println!("Player disconnected. ID - {}, nickname - {}", entity.id, player.nickname());
                }
            }
            Entity::Asteroid => {
                self.asteroids.remove(&entity.id);
            }
        }
    }

    fn update_entity(&mut self, entity: astero::update::Entity) {
        match entity {
            astero::update::Entity::Player(player) => {
                self.others
                    .entry(player.id)
                    .and_modify(|p| p.update_body(&player.body));
            }
            astero::update::Entity::Asteroid(asteroid) => {
                self.asteroids
                    .entry(asteroid.id)
                    .and_modify(|a| a.update_body(&asteroid.body));
            }
        }
    }

    fn handle_message(&mut self, ctx: &mut Context, msg: msg::Msg) -> GameResult<()> {
        match msg {
            msg::Msg::JoinAck(ref this_player) => self.init_player(this_player),

            msg::Msg::FromServer(msg) => {
                match msg {
                    astero::server::Msg::Create(create) => {
                        let entity = create.entity.expect("Got empty create entity from server");
                        self.create_entity(ctx, entity)?
                    }
                    astero::server::Msg::Destroy(ref entity) => self.destroy_entity(entity),
                    astero::server::Msg::List(updates) => {
                        for update in updates.update_list {
                            let entity = update.entity.expect("Got empty update entity from server");
                            self.update_entity(entity);
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
        let frame_time = timer::duration_to_f64(frame_time);

        self.time_acc += frame_time;

        while self.time_acc > constant::physics::DELTA_TIME {
            let x_bound = ctx.conf.window_mode.width as f32 / 2.0;
            let y_bound = ctx.conf.window_mode.height as f32 / 2.0;

            let seconds = constant::physics::DELTA_TIME as f32;

            self.player.update_position(seconds);
            self.player.wrap_position(x_bound, y_bound);

            for player in self.others.values_mut() {
                player.update_position(seconds);
                player.wrap_position(x_bound, y_bound);
            }

            for shot in &mut self.shots {
                shot.update_position(seconds);
                shot.wrap_position(x_bound, y_bound);
            }

            for rock in self.asteroids.values_mut() {
                rock.update_position(seconds);
                rock.wrap_position(x_bound, y_bound);
            }

            self.dispatcher.dispatch(&self.world.res);
            self.time_acc -= constant::physics::DELTA_TIME;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;

        graphics::clear(ctx);

        let assets = self.world.read_resource::<resources::Assets>();
        let bodies = self.world.read::<components::Body>();
        let sprites = self.world.read::<components::Sprite>();

        for (body, sprite) in (&bodies, &sprites).join() {
            let sprite = assets.get_sprite(&sprite.0);
            let pos = self.world_to_screen_coords(ctx, body.pos);

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
            nickname.draw(ctx, body.pos, body.size, color.0)?;
        }

        let lives = self.world.read::<components::Life>();
        let sticky_health_bars = self.world.read::<components::StickyHealthBar>();

        for (body, life, color, sticky_health_bar) in (&bodies, &lives, &colors, &sticky_health_bars).join() {
            let pos = self.world_to_screen_coords(ctx, body.pos);
            sticky_health_bar.draw(ctx, pos, body.size, life.fraction(), color.0)?;
        }

        let static_health_bars = self.world.read::<components::StaticHealthBar>();

        for (body, life, static_health_bar) in (&bodies, &lives, &static_health_bars).join() {
            let pos = self.world_to_screen_coords(ctx, body.pos);
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

        let mut input = self.world.write_resource::<resources::Input>();
        let maybe_update = input.key_down(keycode, repeat);

        if let Some(update) = maybe_update {
            self.client.send(msg::Msg::ToServer(update.into()));
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut input = self.world.write_resource::<resources::Input>();
        let maybe_update = input.key_up(keycode);

        if let Some(update) = maybe_update {
            self.client.send(msg::Msg::ToServer(update.into()));
        }
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
