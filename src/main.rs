extern crate ggez;

use ggez::event;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Increase or decrease 'pos_x' if keyboard pressed
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.pos_x += 4.5;
            }
            self.pos_x += 0.5;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.pos_x -= 4.5;
            }
            self.pos_x -= 0.5;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());

        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            [10.0, 10.0, 75.0, 80.0].into(),
            graphics::BLACK
        )?;
        graphics::draw(ctx, &square, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
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
