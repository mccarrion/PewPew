extern crate ggez;

use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;

use entity::*;
use helpers::print_instructions;
use input::InputState;

mod entity;
mod helpers;
mod input;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

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
