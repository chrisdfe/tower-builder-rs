use macroquad::{color::Color, window::clear_background};

use crate::game::Game;

// Helpful JS to convert 0-255 rgb to what we need here
// "118, 16, 70, 255".split(',').map(n => parseInt(n, 10)).map(n => n/255).map(n => Math.round(n * 1000) / 1000).join(', ')

// Helpful JS to convert hex to what we need here
// "2d0d03".match(/.{1,2}/g).map(n => parseInt(n, 16)).map(n => n/255).map(n => Math.round(n * 1000) / 1000).join(', ')
pub const BG_COLOR: Color = Color::new(0.075, 0.094, 0.067, 1.);

mod ground;
mod rooms;
mod sky;
mod tools;
mod ui;

pub mod text;

pub async fn initialize() {
  text::initialize().await
}

pub fn render(game: &Game) {
  clear_background(BG_COLOR);

  // World
  sky::render(game);
  ground::render(game);
  rooms::render(game);
  tools::render(game);
  // draw_room_blueprint(game);

  // UI
  ui::render(game);
}
