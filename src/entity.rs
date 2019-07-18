use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::input::InputState;

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
pub struct Actor {
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

pub fn create_player() -> Actor {
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

pub fn player_handle_input(actor: &mut Actor, input: &InputState, dt: f32) {
    actor.velocity += Vector2::new(input.xaxis, input.yaxis) * (PLAYER_THRUST) * (dt);
}

const MAX_PHYSICS_VEL: f32 = 250.0;

pub fn update_actor_position(actor: &mut Actor, dt: f32) {
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
pub fn wrap_actor_position(actor: &mut Actor, sx: f32, sy: f32) {
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

pub struct Assets {
    pub player_image: graphics::Image,
    pub font: graphics::Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
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

pub fn draw_actor(
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