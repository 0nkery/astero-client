#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", feature(clippy))]

#![feature(ip_constructors)]
#![feature(use_nested_groups)]
#![feature(entry_and_modify)]

extern crate ggez;
extern crate rand;

extern crate futures;
extern crate tokio_core;
extern crate quick_protobuf;

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
use client::proto::{
    SpawnEntity,
    Entity,
    Body,
};

mod constant;
use constant::PLAYER_SHOT_TIME;
use constant::gui::HEALTH_BAR_SIZE;

mod health_bar;

mod player;
use player::Player;

mod asteroid;
use asteroid::Asteroid;

mod shot;
use shot::Shot;

mod util;


trait Movable {
    fn update_position(&mut self, dt: f32);
    fn wrap_position(&mut self, sx: f32, sy: f32);
    fn get_body(&self) -> &Body;

    fn collided<M: Movable>(&mut self, other: &M) -> bool {
        let self_body = self.get_body();
        let other_body = other.get_body();

        let distance = ggez::nalgebra::distance(&self_body.pos, &other_body.pos);

        distance < (self_body.size + other_body.size)
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

        Ok(Assets {
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
    xaxis: f32,
    yaxis: f32,
    fire: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            fire: false,
            xaxis: 0.0,
            yaxis: 0.0,
        }
    }
}

struct MainState {
    player: Player,
    shots: Vec<Shot>,
    asteroids: HashMap<u32, Asteroid>,
    score: i32,
    others: HashMap<u32, Player>,

    assets: Assets,
    screen_width: u32,
    screen_height: u32,

    input: InputState,
    player_shot_timeout: f32,

    gui_dirty: bool,
    score_display: graphics::Text,
    health_bar: health_bar::StaticHealthBar,

    client: client::ClientHandle,
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

        let client = client::ClientHandle::start();
        client.send(Msg::Join(nickname));

        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;

        let health_bar = health_bar::StaticHealthBar::new(
            (screen_width / 4 + 10) as f32,
            (screen_height - 30) as f32,
            (screen_width / 2) as f32,
            HEALTH_BAR_SIZE
        );

        let s = MainState {
            player,
            shots: Vec::new(),
            asteroids: HashMap::new(),
            score: 0,
            others: HashMap::new(),

            assets,
            screen_width,
            screen_height,

            input: InputState::default(),
            player_shot_timeout: 0.0,

            gui_dirty: true,
            score_display,
            health_bar,

            client
        };

        Ok(s)
    }

    fn fire_player_shot(&mut self) {
        if !self.player.is_ready() {
            return;
        }

        self.player_shot_timeout = PLAYER_SHOT_TIME;
        let shot = self.player.fire();
        self.shots.push(shot);
    }

    fn clear_dead_stuff(&mut self) {
        self.shots.retain(|s| s.is_alive());
        self.asteroids.retain(|_, r| r.is_alive());
    }

    fn handle_collisions(&mut self) {
        for (_, rock) in &mut self.asteroids {

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

    fn handle_message(&mut self, ctx: &mut Context, msg: Msg) -> GameResult<()> {
        match msg {
            Msg::JoinAck(ack) => {
                println!("Connected to server. Conn ID - {}", ack.id);
                self.player.set_body(ack.body);
            }
            Msg::OtherJoined(other) => {
                println!(
                    "Player connected. ID - {}, nickname - {}, coord - ({}, {})",
                    other.id, other.nickname, other.body.pos.x, other.body.pos.y
                );

                let mut player = Player::new(
                    ctx, other.nickname, &self.assets.small_font, constant::colors::RED
                )?;
                player.set_body(other.body);
                self.others.insert(other.id, player);
            }
            Msg::OtherLeft(other) => {
                let player = self.others.remove(&other.id);
                if let Some(player) = player {
                    println!("Player disconnected. ID - {}, nickname - {}", other.id, player.nickname());
                }
            }
            Msg::ServerNotResponding => {
                println!("Server is not available! Closing game...");
                ctx.quit()?;
            }
            Msg::Spawn(spawn) => {
                match spawn.entity {
                    SpawnEntity::asteroids(asteroids) => {
                        self.asteroids.extend(asteroids.entities.into_iter()
                            .map(|(id, a)| (id, Asteroid::new(a))));
                    }

                    SpawnEntity::None => {}
                }
            }
            Msg::SimUpdates(updates) => {
                for update in updates {
                    match update.entity {
                        Entity::ASTEROID => {
                            if self.asteroids.contains_key(&update.id) {
                                self.asteroids
                                    .entry(update.id)
                                    .and_modify(move |a| a.update_body(&update.body));
                            }
                        }

                        _ => {}
                    }
                }
            }
            _ => {}
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

            self.player.handle_input(&self.input, seconds);
            self.player_shot_timeout -= seconds;
            if self.input.fire && self.player_shot_timeout < 0.0 {
                self.fire_player_shot();
            }

            self.player.update_position(seconds);
            self.player.wrap_position(self.screen_width as f32, self.screen_height as f32);

            for shot in &mut self.shots {
                shot.update_position(seconds);
                shot.wrap_position(self.screen_width as f32, self.screen_height as f32);
            }

            for (_id, rock) in &mut self.asteroids {
                rock.update_position(seconds);
                rock.wrap_position(self.screen_width as f32, self.screen_height as f32);
            }

            self.handle_collisions();

            if let Ok(msg) = self.client.try_recv() {
                self.handle_message(ctx, msg)?;
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

            for (_id, asteroid) in &self.asteroids {
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

         self.health_bar.draw(ctx, self.player.life(), self.player.max_life())?;

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
        match keycode {
            Keycode::W | Keycode::Up => {
                self.input.yaxis = 1.0;
            }
            Keycode::S | Keycode::Down => {
                self.input.yaxis = -1.0;
            }
            Keycode::A | Keycode::Left => {
                self.input.xaxis = -1.0;
            }
            Keycode::D | Keycode::Right => {
                self.input.xaxis = 1.0;
            }
            Keycode::Space => {
                self.input.fire = true;
            }
            Keycode::Escape => {
                ctx.quit().expect("Failed to quit the game");
            },
            _ => (),
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool
    ) {
        match keycode {
            Keycode::W | Keycode::S | Keycode::Up | Keycode::Down => {
                self.input.yaxis = 0.0;
            }
            Keycode::A | Keycode::D | Keycode::Left | Keycode::Right => {
                self.input.xaxis = 0.0;
            }
            Keycode::Space => {
                self.input.fire = false;
            }
            _ => (),
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
