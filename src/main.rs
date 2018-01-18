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

use ggez::conf;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::timer;


mod client;
use client::Msg;

mod constant;
use constant::gui::HEALTH_BAR_SIZE;

mod health_bar;

mod body;
use body::Body;

mod player;
use player::Player;

mod asteroid;
use asteroid::Asteroid;

mod shot;
use shot::Shot;

mod util;

mod proto;
use proto::{
    mmob,
    astero,
    astero::Entity,
    astero::Input,
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

#[derive(Debug)]
pub struct InputState {
    turn: i32,
    accel: i32,
    fire: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            turn: 0,
            accel: 0,
            fire: false,
        }
    }
}

impl InputState {
    fn update(&mut self, update: &Input) -> bool {
        let new_turn = update.turn.unwrap_or(self.turn);
        let new_accel = update.accel.unwrap_or(self.accel);
        let new_fire = update.fire.unwrap_or(self.fire);

        let updated =
            new_turn != self.turn
            || new_accel != self.accel
            || new_fire != self.fire;

        if updated {
            self.turn = new_turn;
            self.accel = new_accel;
            self.fire = new_fire;

            return true;
        }

        false
    }
}

struct MainState {
    player_id: u32,
    player: Player,
    asteroids: HashMap<u32, Asteroid>,
    score: i32,
    others: HashMap<u32, Player>,

    shots: Vec<Shot>,
    unconfirmed_shots: HashMap<u32, Shot>,

    input: InputState,

    assets: Assets,
    screen_width: u32,
    screen_height: u32,

    gui_dirty: bool,
    score_display: graphics::Text,
    health_bar: health_bar::Static,

    client: client::Handle,
    latency: u64,
    server_time_delta: u64,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        print_instructions();

        let assets = Assets::new(ctx)?;
        let score_display = graphics::Text::new(ctx, "score", &assets.font)?;

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

        let client = client::Handle::start();
        client.send(Msg::JoinGame(nickname));
        println!("Connecting to server...");

        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;

        let health_bar = health_bar::Static::new(
            10 as f32,
            screen_height as f32 - HEALTH_BAR_SIZE - 5.0,
            (screen_width / 2) as f32,
            HEALTH_BAR_SIZE
        );

        let s = Self {
            player_id: 0,
            player,
            asteroids: HashMap::new(),
            score: 0,
            others: HashMap::new(),

            shots: Vec::new(),
            unconfirmed_shots: HashMap::new(),

            input: InputState::default(),

            assets,
            screen_width,
            screen_height,

            gui_dirty: true,
            score_display,
            health_bar,

            client,
            latency: 0,
            server_time_delta: 0,
        };

        Ok(s)
    }

    fn clear_dead_stuff(&mut self) {
        self.shots.retain(| s| s.is_alive());
        self.asteroids.retain(|_, r| r.is_alive());
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
                    if rock.is_dead() {
                        self.score += 1;
                    }
                    self.gui_dirty = true;
                }
            }
        }
    }

    fn update_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let score_str = format!("Score: {}", self.score);
        self.score_display = graphics::Text::new(ctx, &score_str, &self.assets.font)?;

        Ok(())
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

    fn update_latency(&mut self, latency_measure: &mmob::LatencyMeasure) {
        let now = util::cur_time_in_millis();
        self.server_time_delta = now - latency_measure.server_timestamp.unwrap_or(now);
        self.latency = (now - latency_measure.timestamp) / 2;
    }

    fn server_time(&self) -> u64 {
        self.latency as u64 + self.server_time_delta
    }

    fn handle_message(&mut self, ctx: &mut Context, msg: Msg) -> GameResult<()> {
        match msg {
            Msg::JoinAck(ref this_player) => self.init_player(this_player),

            Msg::FromServer(msg) => {
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

            Msg::ServerNotResponding => {
                println!("Server is not available! Closing game...");
                ctx.quit()?;
            }

            Msg::Latency(ref measure) => self.update_latency(measure),

            Msg::Unknown | Msg::JoinGame(..) | Msg::LeaveGame |
            Msg::Heartbeat | Msg::ToServer(..) => unreachable!(),
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

impl EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            let x_bound = self.screen_width as f32 / 2.0;
            let y_bound = self.screen_height as f32 / 2.0;

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

            self.clear_dead_stuff();

            if self.gui_dirty {
                self.update_ui(ctx)?;
                self.gui_dirty = false;
            }

            if self.player.is_dead() {
                println!("Game over!");
                ctx.quit()?;
            }
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

         let score_dest = graphics::Point2::new(
             (self.score_display.width() / 2) as f32 + 200.0,
             (self.score_display.height() / 2) as f32 + 10.0
         );

         graphics::draw(ctx, &self.score_display, score_dest, 0.0)?;

         self.health_bar.draw(ctx, self.player.life() / self.player.max_life())?;

         graphics::present(ctx);

         timer::yield_now();

         Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool
    ) {
        let mut update = Input::default();

        let update = match keycode {
            Keycode::W | Keycode::Up => {
                update.accel = Some(1);
                Some(update)
            }
            Keycode::S | Keycode::Down => {
                update.accel = Some(-1);
                Some(update)
            }
            Keycode::A | Keycode::Left => {
                update.turn = Some(-1);
                Some(update)
            }
            Keycode::D | Keycode::Right => {
                update.turn = Some(1);
                Some(update)
            }
            Keycode::Space => {
                update.fire = Some(true);
                Some(update)
            }
            Keycode::Escape => {
                ctx.quit().expect("Failed to quit the game");

                None
            },
            _ => None,
        };

        if let Some(update) = update {
            if self.input.update(&update) {
                self.client.send(Msg::ToServer(update.into()));
            }
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool
    ) {
        let mut update = Input::default();

        let update = match keycode {
            Keycode::W | Keycode::S | Keycode::Up | Keycode::Down => {
                update.accel = Some(0);
                Some(update)
            }
            Keycode::A | Keycode::D | Keycode::Left | Keycode::Right => {
                update.turn = Some(0);
                Some(update)
            }
            Keycode::Space => {
                update.fire = Some(false);
                Some(update)
            }
            _ => None,
        };

        if let Some(update) = update {
            if self.input.update(&update) {
                self.client.send(Msg::ToServer(update.into()));
            }
        }
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        self.client.stop();

        false
    }
}

fn main() {
    let mut cb = ContextBuilder::new("Astero", "herald-it")
        .window_setup(conf::WindowSetup::default().title("Astero"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600));

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
