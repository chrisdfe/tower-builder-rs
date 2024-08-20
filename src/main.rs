use macroquad::prelude::*;
use macroquad::window::{next_frame, Conf};

mod constants;
mod game;
pub mod types;
mod utils;

use crate::game::{
  lifecycle::{render, update},
  Game,
};

fn window_conf() -> Conf {
  Conf {
    window_title: "towerbuilder-rs".to_owned(),
    high_dpi: true,
    window_width: 800,
    window_height: 600,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  // Seed rng
  rand::srand(macroquad::miniquad::date::now() as _);

  let mut game = Game::new();
  game.initialize();

  render::initialize().await;

  loop {
    update(&mut game);
    render(&game);

    next_frame().await
  }
}
