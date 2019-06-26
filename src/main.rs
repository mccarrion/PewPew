extern crate ggez;

use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}

#[derive(Debug)]
enum ActorType {
    Player,
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    pos: Point2,
    facing: f32,
    velocity: Vector2,
    ang_vel: f32,
    bbox: f32,
    life: f32,
}

const PLAYER_LIFE: f32 = 1.0;

const PLAYER_BBOX: f32 = 12.0;

fn create_player() -> Actor {
    Actor {
        tag: ActorType::Player,
        pos: Point2::origin(),
        facing: 0.0,
        velocity: na::zero(),
        ang_vel: 0.0,
        bbox: PLAYER_BBOX,
        life: PLAYER_LIFE,
    }
}

// Acceleration in pixels per second.
const PLAYER_THRUST: f32 = 100.0;

fn player_handle_input(actor: &mut Actor, input: &InputState, dt: f32) {
    actor.velocity += Vector2::new(input.xaxis, input.yaxis) * (PLAYER_THRUST) * (dt);
}

const MAX_PHYSICS_VEL: f32 = 250.0;

fn update_actor_position(actor: &mut Actor, dt: f32) {
    // Tie velocity to max effectively
    let norm_sq = actor.velocity.norm_squared();
    if norm_sq > MAX_PHYSICS_VEL.powi(2) {
        actor.velocity = actor.velocity / norm_sq.sqrt() * MAX_PHYSICS_VEL;
    }
    let dv = actor.velocity * (dt);
    actor.pos += dv;
    actor.facing += actor.ang_vel;
}

/**
 * Wraps actor position to the bounds of the screen
 */
fn wrap_actor_position(actor: &mut Actor, sx: f32, sy: f32) {
    // wrap screen
    let screen_x_bounds = sx / 2.0;
    let screen_y_bounds = sy / 2.0;
    if actor.pos.x > screen_x_bounds {
        actor.pos.x -= sx;
    } else if actor.pos.x < -screen_x_bounds {
        actor.pos.x += sx;
    };
    if actor.pos.y > screen_y_bounds {
        actor.pos.y -= sy;
    } else if actor.pos.y < -screen_y_bounds {
        actor.pos.y += sy;
    }
}

fn handle_timed_life(actor: &mut Actor, dt: f32) {
    actor.life -= dt;
}

fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Point2) -> Point2 {
    let x = point.x + screen_width / 2.0;
    let y = screen_height - (point.y + screen_height / 2.0);
    Point2::new(x, y)
}

struct Assets {
    player_image: graphics::Image,
    font: graphics::Font,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/survivor.png")?;
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;

        Ok(Assets {
            player_image,
            font,
        })
    }

    fn actor_image(&mut self, actor: &Actor) -> &mut graphics::Image {
        match actor.tag {
            ActorType::Player => &mut self.player_image,
        }
    }
}

#[derive(Debug)]
struct InputState {
    xaxis: f32,
    yaxis: f32,
    xclick: f32,
    yclick: f32,
    fire: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
            xclick: 0.0,
            yclick: 0.0,
            fire: false,
        }
    }
}

struct MainState {
    player: Actor,
    level: i32,
    score: i32,
    assets: Assets,
    screen_width: f32,
    screen_height: f32,
    input: InputState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        print_instructions();

        let assets = Assets::new(ctx)?;

        let player = create_player();

        let s = MainState {
            player,
            //shots: Vec::new(),
            level: 0,
            score: 0,
            assets,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
            input: InputState::default(),
        };

        Ok(s)
    }
}

fn print_instructions() {
    println!();
    println!("Welcome to Zombie Survival!");
    println!();
    println!("How to play:");
    println!("WASD to move and mouse to aim and shoot.");
    println!();
}
fn draw_actor(
    assets: &mut Assets,
    ctx: &mut Context,
    actor: &Actor,
    world_coords: (f32, f32),
) -> GameResult {
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, actor.pos);
    let image = assets.actor_image(actor);
    let drawparams = graphics::DrawParam::new()
        .dest(pos)
        .offset(Point2::new(0.5, 0.5));
    graphics::draw(ctx, image, drawparams)
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            // Handle keyboard presses
            player_handle_input(&mut self.player, &self.input, seconds);

            // Update position of square
            update_actor_position(&mut self.player, seconds);
            wrap_actor_position(
                &mut self.player,
                self.screen_width as f32,
                self.screen_height as f32,
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        

        {
            let assets = &mut self.assets;
            let coords = (self.screen_width, self.screen_height);

            let p = &self.player;
            draw_actor(assets, ctx, p, coords)?;
        }

        let level_dest = Point2::new(10.0, 10.0);
        let score_dest = Point2::new(200.0, 10.0);

        let level_str = format!("Level: {}", self.level);
        let score_str = format!("Score: {}", self.score);
        let level_display = graphics::Text::new((level_str, self.assets.font, 32.0));
        let score_display = graphics::Text::new((score_str, self.assets.font, 32.0));
        graphics::draw(ctx, &level_display, (level_dest, 0.0, graphics::WHITE))?;
        graphics::draw(ctx, &score_display, (score_dest, 0.0, graphics::WHITE))?;

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())

    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _btn: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.input.xclick = x;
        self.input.yclick = y;
        self.input.fire = true;
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.input.xclick = 0.0;
        self.input.yclick = 0.0;
        self.input.fire = false;
    }

    fn key_down_event(
        &mut self, 
        ctx: &mut Context, 
        keycode: KeyCode, 
        _keymod: KeyMods, 
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::W => {
                self.input.yaxis = 1.0;
            }
            KeyCode::S => {
                self.input.yaxis = -1.0;
            }
            KeyCode::A => {
                self.input.xaxis = -1.0;
            }
            KeyCode::D => {
                self.input.xaxis = 1.0;
            }
            KeyCode::Escape => ggez::quit(ctx),
            _ => (),
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        match keycode {
            KeyCode::W => {
                self.input.yaxis = 0.0;
            }
            KeyCode::S => {
                self.input.yaxis = 0.0;
            }
            KeyCode::A => {
                self.input.xaxis = 0.0;
            }
            KeyCode::D => {
                self.input.xaxis = 0.0;
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("pewpew", "ggez")
        .window_setup(conf::WindowSetup::default().title("PewPew die Zombies!"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
