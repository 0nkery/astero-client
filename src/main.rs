#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", feature(clippy))]

#![feature(ip_constructors)]

extern crate ggez;
extern crate rand;
extern crate nalgebra;

extern crate byteorder;
extern crate futures;
extern crate tokio_core;

use std::collections::HashMap;
use std::env;
use std::process;
use std::thread;
use std::time::Duration;

use ggez::conf;
use ggez::event::*;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::timer;


mod client;
use client::Msg;

mod constant;
use constant::{
    PLAYER_SHOT_TIME,
    ROCK_BBOX,
    ROCK_LIFE,
    MAX_ROCK_VEL,
    SHOT_RVEL,
    SHOT_BBOX,
    SHOT_LIFE,
    SHOT_SPEED,
    MAX_PHYSICS_VEL,
    SPRITE_HALF_SIZE,
    SPRITE_SIZE,
};
use constant::gui::HEALTH_BAR_SIZE;

mod health_bar;

mod player;
use player::Player;


mod util;
use util::{
    Point2,
    Vector2,
    vec_from_angle,
    random_vec,
    world_to_screen_coords
};


trait Movable {
    #[inline]
    fn velocity(&self) -> Vector2;
    #[inline]
    fn set_velocity(&mut self, velocity: Vector2);

    #[inline]
    fn pos(&self) -> Point2;
    #[inline]
    fn set_pos(&mut self, pos: Point2);

    #[inline]
    fn facing(&self) -> f32;
    #[inline]
    fn set_facing(&mut self, facing: f32);

    #[inline]
    fn rvel(&self) -> f32;
    #[inline]
    fn set_rvel(&mut self, rvel: f32);

    fn update_position(&mut self, dt: f32) {
        let mut velocity = self.velocity();
        let norm_sq = velocity.norm_squared();
        if norm_sq > MAX_PHYSICS_VEL.powi(2) {
            velocity = velocity / norm_sq.sqrt() * MAX_PHYSICS_VEL;
        }
        self.set_velocity(velocity);

        let dv = velocity * dt;
        let pos = self.pos() + dv;
        self.set_pos(pos);

        let facing = self.facing() + self.rvel();
        self.set_facing(facing);
    }

    fn wrap_position(&mut self, sx: f32, sy: f32) {
        let screen_x_bounds = sx / 2.0;
        let screen_y_bounds = sy / 2.0;
        let mut pos = self.pos();

        let center = pos - Vector2::new(-SPRITE_HALF_SIZE, SPRITE_HALF_SIZE);

        if center.x > screen_x_bounds {
            pos.x -= sx;
        } else if center.x < -screen_x_bounds {
            pos.x += sx;
        }
        if center.y > screen_y_bounds {
            pos.y -= sy;
        } else if center.y < -screen_y_bounds {
            pos.y += sy;
        }

        self.set_pos(pos);
    }
}


#[derive(Debug)]
enum ActorType {
    Rock,
    Shot
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    pos: Point2,
    facing: f32,
    velocity: Vector2,
    rvel: f32,
    bbox_size: f32,
    life: f32
}

impl Actor {
    fn create_rock() -> Self {
        Actor {
            tag: ActorType::Rock,
            pos: Point2::origin(),
            facing: 0.,
            velocity: nalgebra::zero(),
            rvel: 0.,
            bbox_size: ROCK_BBOX,
            life: ROCK_LIFE
        }
    }

    fn create_rocks(num: usize, exclusion: Point2, min_radius: f32, max_radius: f32) -> Vec<Actor> {
        assert!(max_radius > min_radius);
        let mut rocks = Vec::with_capacity(num);
        for _ in 0..num {
            let mut rock = Self::create_rock();
            let r_angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
            let r_distance = rand::random::<f32>() * (max_radius - min_radius) + min_radius;
            rock.pos = exclusion + vec_from_angle(r_angle) * r_distance;
            rock.velocity = random_vec(MAX_ROCK_VEL);
            rocks.push(rock);
        }
        rocks
    }

    fn create_shot() -> Self {
        Actor {
            tag: ActorType::Shot,
            pos: Point2::origin(),
            facing: 0.,
            velocity: nalgebra::zero(),
            rvel: SHOT_RVEL,
            bbox_size: SHOT_BBOX,
            life: SHOT_LIFE
        }
    }

    fn handle_timed_life(&mut self, dt: f32) {
        self.life -= dt;
    }
}

impl Movable for Actor {
    fn velocity(&self) -> Vector2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Vector2) {
        self.velocity = velocity;
    }

    fn pos(&self) -> Point2 {
        self.pos
    }

    fn set_pos(&mut self, pos: Point2) {
        self.pos = pos;
    }

    fn facing(&self) -> f32 {
        self.facing
    }

    fn set_facing(&mut self, facing: f32) {
        self.facing = facing;
    }

    fn rvel(&self) -> f32 {
        self.rvel
    }

    fn set_rvel(&mut self, rvel: f32) {
        self.rvel = rvel;
    }
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

    fn actor_image(&mut self, actor: &Actor) -> &mut graphics::Image {
        match actor.tag {
            ActorType::Rock => &mut self.rock_image,
            ActorType::Shot => &mut self.shot_image,
        }
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
    shots: Vec<Actor>,
    rocks: Vec<Actor>,
    level: usize,
    score: i32,
    others: HashMap<u16, String>,

    assets: Assets,
    screen_width: u32,
    screen_height: u32,

    input: InputState,
    player_shot_timeout: f32,

    gui_dirty: bool,
    score_display: graphics::Text,
    level_display: graphics::Text,
    health_bar: health_bar::StaticHealthBar,

    client: client::Client,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        print_instructions();

        let assets = Assets::new(ctx)?;
        let score_display = graphics::Text::new(ctx, "score", &assets.font)?;
        let level_display = graphics::Text::new(ctx, "level", &assets.font)?;

        let rocks = Actor::create_rocks(5, Point2::origin(), 100.0, 250.0);

        let home_dir = env::home_dir().expect("Failed to retrieve home dir");
        let nickname =
            home_dir
                .as_path()
                .file_name()
                .expect("Failed to retrieve username")
                .to_str()
                .expect("Failed to convert username to Unicode")
                .to_string();

        let player = Player::new(ctx, &nickname, &assets.small_font)?;

        let client = client::Client::start();
        client.send(Msg::Join(nickname));

        let screen_width = ctx.conf.window_width;
        let screen_height = ctx.conf.window_height;

        let health_bar = health_bar::StaticHealthBar::new(
            (screen_width / 4 + 10) as f32,
            (screen_height - 30) as f32,
            (screen_width / 2) as f32,
            HEALTH_BAR_SIZE
        );

        let s = MainState {
            player,
            shots: Vec::new(),
            rocks,
            level: 0,
            score: 0,
            others: HashMap::new(),

            assets,
            screen_width,
            screen_height,

            input: InputState::default(),
            player_shot_timeout: 0.0,

            gui_dirty: true,
            score_display,
            level_display,
            health_bar,

            client
        };

        Ok(s)
    }

    fn fire_player_shot(&mut self) {
        self.player_shot_timeout = PLAYER_SHOT_TIME;
        let mut shot = Actor::create_shot();
        shot.pos = self.player.pos();
        shot.facing = self.player.facing();
        let direction = vec_from_angle(shot.facing);
        shot.velocity = direction * SHOT_SPEED;
        self.shots.push(shot);
    }

    fn clear_dead_stuff(&mut self) {
        self.shots.retain(|s| s.life > 0.0);
        self.rocks.retain(|r| r.life > 0.0);
    }

    fn handle_collisions(&mut self) {
        for rock in &mut self.rocks {
            let distance = rock.pos - self.player.pos();
            if distance.norm() < (self.player.bbox_size() + rock.bbox_size) {
                self.player.damage(1.0);
                rock.life = 0.0;
                continue;
            }
            for shot in &mut self.shots {
                let distance = shot.pos - rock.pos;
                if distance.norm() < (shot.bbox_size + rock.bbox_size) {
                    shot.life = 0.0;
                    rock.life -= 1.0;
                    if rock.life <= 0.0 {
                        self.score += 1;
                    }
                    self.gui_dirty = true;
                }
            }
        }
    }

    fn check_for_level_respawn(&mut self) {
        if self.rocks.is_empty() {
            self.level += 1;
            self.gui_dirty = true;
            let r = Actor::create_rocks(self.level + 5, self.player.pos(), 100.0, 250.0);
            self.rocks.extend(r);
        }
    }

    fn update_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let score_str = format!("Score: {}", self.score);
        let level_str = format!("Level: {}", self.level);
        self.score_display = graphics::Text::new(ctx, &score_str, &self.assets.font)?;
        self.level_display = graphics::Text::new(ctx, &level_str, &self.assets.font)?;

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

fn draw_actor(
    assets: &mut Assets,
    ctx: &mut Context,
    actor: &Actor,
    world_coords: (u32, u32)) -> GameResult<()>
{
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, actor.pos);
    let dest_point = graphics::Point::new(pos.x as f32, pos.y as f32);
    let image = assets.actor_image(actor);
    graphics::draw(ctx, image, dest_point, actor.facing as f32)?;

    if let ActorType::Rock = actor.tag {
        let x = pos.x;
        let y = pos.y + SPRITE_HALF_SIZE + 4.0;

        health_bar::StickyHealthBar::draw(ctx, x, y, SPRITE_SIZE as f32, actor.life, ROCK_LIFE)?;
    }

    Ok(())
}

impl EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        const DESIRED_FPS: u64 = 60;
        if !timer::check_update_time(ctx, DESIRED_FPS) {
            return Ok(())
        }

        if let Ok(msg) = self.client.try_recv() {
            match msg {
                Msg::JoinAck(conn_id, x, y) => {
                    println!("Connected to server. Conn ID - {}", conn_id);
                }
                Msg::OtherJoined(conn_id, nickname) => {
                    println!("Player connected. ID - {}, nickname - {}", conn_id, nickname);
                    self.others.insert(conn_id, nickname);
                }
                Msg::OtherLeft(conn_id) => {
                    let nickname = self.others.remove(&conn_id).unwrap();
                    println!("Player disconnected. ID - {}, nickname - {}", conn_id, nickname);
                }
                Msg::ServerNotResponding => {
                    println!("Server is not available! Closing game...");
                    ctx.quit()?;
                }

                _ => {}
            }
        }

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
            shot.handle_timed_life(seconds);
        }

        for rock in &mut self.rocks {
            rock.update_position(seconds);
            rock.wrap_position(self.screen_width as f32, self.screen_height as f32);
        }

        self.handle_collisions();
        self.clear_dead_stuff();
        self.check_for_level_respawn();

        if self.gui_dirty {
            self.update_ui(ctx)?;
            self.gui_dirty = false;
        }

        if self.player.cur_life() <= 0.0 {
            println!("Game over!");
            ctx.quit()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            let coords = (self.screen_width, self.screen_height);

            self.player.draw(ctx, &mut self.assets, coords)?;

            for shot in &self.shots {
                draw_actor(&mut self.assets, ctx, shot, coords)?;
            }

            for rock in &self.rocks {
                draw_actor(&mut self.assets, ctx, rock, coords)?;
            }
        }

         let level_dest = graphics::Point::new(
             (self.level_display.width() / 2) as f32 + 10.0,
             (self.level_display.height() / 2) as f32 + 10.0
         );
         let score_dest = graphics::Point::new(
             (self.score_display.width() / 2) as f32 + 200.0,
             (self.score_display.height() / 2) as f32 + 10.0
         );

         graphics::draw(ctx, &self.level_display, level_dest, 0.0)?;
         graphics::draw(ctx, &self.score_display, score_dest, 0.0)?;

         self.health_bar.draw(ctx, self.player.cur_life(), self.player.max_life())?;

         graphics::present(ctx);

         thread::yield_now();
         Ok(())
    }

    fn key_down_event(
        &mut self,
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
            Keycode::Escape => process::exit(0),
            _ => (),
        }
    }

    fn key_up_event(
        &mut self,
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

    fn quit_event(&mut self) -> bool {
        self.client.stop();

        false
    }
}

fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "Astero".to_string();
    c.window_width = 800;
    c.window_height = 600;

    let ctx = &mut Context::load_from_conf("astero", "ggez", c).expect("Failed to load configuration");

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
