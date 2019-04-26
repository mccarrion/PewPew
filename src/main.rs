extern crate ggez;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};
use ggez::nalgebra as na;
use ggez::input::keyboard;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

#[derive(Debug)]
enum ActorType {
    Player,
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    pos: Point2,
    velocity: Vector2,
}

fn create_player() -> Actor {
    Actor {
        tag: ActorType::Player,
        pos: Point2::origin(),
        velocity: na::zero(),
    }
}

struct MainState {
    player: Actor,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let player = create_player();
        let s = MainState { 
            player,
        };
        Ok(s)
    }
}

fn draw_actor(
    ctx: &mut Context,
    actor: &Actor,
) -> GameResult {
    // Need to fill this out
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
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        

        let p = &self.player;
        draw_actor(ctx, p)?;

        /*
        This will have to be removed soonish

        let color = [0.0, 0.0, 1.0, 1.0].into();

        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos.into(),
            color
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        */

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())

    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        match key {
            // Quit if Shift+Ctrl+Q is pressed
            KeyCode::Q => {
                if mods.contains(KeyMods::SHIFT & KeyMods::CTRL) {
                    println!("Terminating!");
                    ggez::quit(ctx);
                } else if mods.contains(KeyMods::SHIFT) || mods.contains(KeyMods::CTRL) {
                    println!("You need to hold both Shift and Control to quit.");
                } else {
                    println!("Now you're not even trying!");
                }
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
