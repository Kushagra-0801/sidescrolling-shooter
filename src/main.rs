#![allow(unused)]

use ggez;
use ggez::conf;
use ggez::{mint, GameResult};

use rand::prelude::*;

mod helpers;

mod state;
use state::State;

mod player;

mod enemy;

const SCREEN_SIZE: (f32, f32) = (800.0, 500.0);

type Point = mint::Point2<f32>;
type Vector = mint::Vector2<f32>;

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Shooter", "Kushagra")
        .window_setup(conf::WindowSetup::default().title("Shooter"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    let state = &mut State::new();
    ggez::event::run(&mut ctx, &mut event_loop, state)
}
