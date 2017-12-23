#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", feature(clippy))]

#![feature(ip_constructors)]
#![feature(use_nested_groups)]

extern crate ggez;
extern crate rand;
extern crate nalgebra;

extern crate futures;
extern crate tokio_core;
extern crate quick_protobuf;

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
use client::proto::Entity;

mod constant;
use constant::{
    PLAYER_SHOT_TIME,
    SHOT_RVEL,
    SHOT_BBOX,
    SHOT_LIFE,
    SHOT_SPEED,
    MAX_PHYSICS_VEL,
    SPRITE_HALF_SIZE,
};
use constant::gui::HEALTH_BAR_SIZE;

mod health_bar;

mod player;
use player::Player;

mod asteroid;
use asteroid::Asteroid;

mod util;
use util::{
    Point2,
    Vector2,
    vec_from_angle,
    world_to_screen_coords,
    reflect_vector,
};


trait Movable {
    #[inline]
    fn velocity(&self) -> Vector2;
    #[inline]
    fn set_velocity(&mut self, velocity: Vector2);

    #[inline]
    fn pos(&self) -> Option<Point2>;
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
        if self.pos().is_none() {
            return;
        }

        let mut velocity = self.velocity();
        let norm_sq = velocity.norm_squared();
        if norm_sq > MAX_PHYSICS_VEL.powi(2) {
            velocity = velocity / norm_sq.sqrt() * MAX_PHYSICS_VEL;
        }
        self.set_velocity(velocity);

        let dv = velocity * dt;
        let pos = self.pos().unwrap() + dv;
        self.set_pos(pos);

        let facing = self.facing() + self.rvel();
        self.set_facing(facing);
    }

    fn wrap_position(&mut self, sx: f32, sy: f32) {
        if self.pos().is_none() {
            return;
        }

        let screen_x_bounds = sx / 2.0;
        let screen_y_bounds = sy / 2.0;
        let pos = self.pos().unwrap();

        let center = pos - Vector2::new(-SPRITE_HALF_SIZE, SPRITE_HALF_SIZE);

        if center.x > screen_x_bounds || center.x < -screen_x_bounds {
            let normal = Vector2::new(sy, 0.0);
            let v = reflect_vector(self.velocity(), normal);
            self.set_velocity(v);
        } else if center.y > screen_y_bounds || center.y < -screen_y_bounds {
            let normal = Vector2::new(0.0, sx);
            let v = reflect_vector(self.velocity(), normal);
            self.set_velocity(v);
        };
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


#[derive(Debug)]
enum ActorType {
    Shot
}

#[derive(Debug)]
pub struct Actor {
    tag: ActorType,
    pos: Point2,
    facing: f32,
    velocity: Vector2,
    rvel: f32,
    bbox_size: f32,
    life: f32
}

impl Actor {
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

    fn pos(&self) -> Option<Point2> {
        Some(self.pos)
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
            ActorType::Shot => &mut self.shot_image,
        }
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
    shots: Vec<Actor>,
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
        if self.player.pos().is_none() {
            return;
        }

        self.player_shot_timeout = PLAYER_SHOT_TIME;
        let mut shot = Actor::create_shot();
        shot.pos = self.player.pos().unwrap();
        shot.facing = self.player.facing();
        let direction = vec_from_angle(shot.facing);
        shot.velocity = direction * SHOT_SPEED;
        self.shots.push(shot);
    }

    fn clear_dead_stuff(&mut self) {
        self.shots.retain(|s| s.life > 0.0);
        self.asteroids.retain(|_, r| r.is_alive());
    }

    fn handle_collisions(&mut self) {
        for (_, rock) in &mut self.asteroids {

            if let Some(ref pos) = self.player.pos() {
                let distance = rock.pos().unwrap() - pos;
                if distance.norm() < (self.player.bbox_size() + rock.bbox_size) {
                    self.player.damage(1.0);
                    rock.destroy();
                    continue;
                }
            }

            for shot in &mut self.shots {
                let distance = shot.pos - rock.pos().unwrap();
                if distance.norm() < (shot.bbox_size + rock.bbox_size) {
                    shot.life = 0.0;
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
                self.player.set_pos(ack.pos.into());
            }
            Msg::OtherJoined(other) => {
                println!("Player connected. ID - {}, nickname - {}, coord - ({})", other.id, other.nickname, other.pos);
                let mut player = Player::new(
                    ctx, other.nickname, &self.assets.small_font, constant::colors::RED
                )?;
                player.set_pos(other.pos);
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
                    Entity::asteroids(asteroids) => {
                        self.asteroids.extend(asteroids.entities.into_iter()
                            .map(|(id, a)| (id, Asteroid::new(a))));
                    }

                    Entity::None => {}
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

    Ok(())
}

impl EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        const DESIRED_FPS: u64 = 60;
        if !timer::check_update_time(ctx, DESIRED_FPS) {
            return Ok(())
        }

        if let Ok(msg) = self.client.try_recv() {
            self.handle_message(ctx, msg)?;
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

        for (_id, rock) in &mut self.asteroids {
            rock.update_position(seconds);
            rock.wrap_position(self.screen_width as f32, self.screen_height as f32);
        }

        self.handle_collisions();
        self.clear_dead_stuff();

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

            for (_id, asteroid) in &self.asteroids {
                asteroid.draw(ctx, &mut self.assets, coords)?;
            }

            for other in self.others.values() {
                other.draw(ctx, &mut self.assets, coords)?;
            }
        }

         let score_dest = graphics::Point::new(
             (self.score_display.width() / 2) as f32 + 200.0,
             (self.score_display.height() / 2) as f32 + 10.0
         );

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
