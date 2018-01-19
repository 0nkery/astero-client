#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", feature(clippy))]

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

mod health_bar;

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


pub struct Assets {
    player_image: graphics::Image,
    shot_image: graphics::Image,
    rock_image: graphics::Image,
    font: graphics::Font,
    small_font: graphics::Font,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;
        let rock_image = graphics::Image::new(ctx, "/rock.png")?;

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;
        let small_font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 12)?;

        Ok(Self {
            player_image,
            shot_image,
            rock_image,
            font,
            small_font,
        })
    }

    fn shot_image(&mut self) -> &mut graphics::Image {
        &mut self.shot_image
    }

    fn asteroid_image(&mut self) -> &mut graphics::Image {
        &mut self.rock_image
    }

    fn player_image(&mut self) -> &mut graphics::Image {
        &mut self.player_image
    }
}


struct MainState<'a, 'b> {
    player_id: u32,
    player: Player,
    asteroids: HashMap<u32, Asteroid>,
    others: HashMap<u32, Player>,

    shots: Vec<Shot>,
    unconfirmed_shots: HashMap<u32, Shot>,

    assets: Assets,
    screen_width: u32,
    screen_height: u32,

    health_bar: health_bar::Static,

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
        let mut dispatcher_builder = DispatcherBuilder::new();

        world.add_resource(resources::Input::new());
        world.add_resource(resources::ServerClock::new());

        let dispatcher = dispatcher_builder.build();

        let assets = Assets::new(ctx)?;

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

        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;

        let health_bar = health_bar::Static::new(
            10 as f32,
            screen_height as f32 - constant::gui::HEALTH_BAR_SIZE - 5.0,
            (screen_width / 2) as f32,
            constant::gui::HEALTH_BAR_SIZE
        );

        let s = Self {
            player_id: 0,
            player,
            asteroids: HashMap::new(),
            others: HashMap::new(),

            shots: Vec::new(),
            unconfirmed_shots: HashMap::new(),

            assets,
            screen_width,
            screen_height,

            health_bar,

            client,

            world,
            dispatcher,
            time_acc: 0.0,
        };

        Ok(s)
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
        println!("Connected to server. Conn ID - {}", this.id);
        self.player_id = this.id;
        self.player.set_body(&this.body);
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
        let frame_time = timer::get_delta(ctx);
        let frame_time = timer::duration_to_f64(frame_time);

        self.time_acc += frame_time;

        while self.time_acc > constant::physics::DELTA_TIME {
            let x_bound = self.screen_width as f32 / 2.0;
            let y_bound = self.screen_height as f32 / 2.0;

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

            self.handle_collisions();

            let mut handled_messages = 0;
            while let Ok(msg) = self.client.try_recv() {
                self.handle_message(ctx, msg)?;
                handled_messages += 1;
                if handled_messages >= 3 {
                    break;
                }
            }

            if self.player.is_dead() {
                println!("Game over!");
                ctx.quit()?;
            }

            self.dispatcher.dispatch(&self.world.res);
            self.time_acc -= constant::physics::DELTA_TIME;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            let coords = (self.screen_width, self.screen_height);

            self.player.draw(ctx, &mut self.assets, coords)?;

            for shot in &self.shots {
                shot.draw(ctx, &mut self.assets, coords)?;
            }

            for asteroid in self.asteroids.values() {
                asteroid.draw(ctx, &mut self.assets, coords)?;
            }

            for other in self.others.values() {
                other.draw(ctx, &mut self.assets, coords)?;
            }
        }

         self.health_bar.draw(ctx, self.player.life() / self.player.max_life())?;

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
    path.push("resources");
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
