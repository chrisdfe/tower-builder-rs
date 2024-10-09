use macroquad::{
  color::Color,
  shapes::draw_rectangle,
  window::{screen_height, screen_width},
};

use crate::{game::Game, types::map::Coordinates, utils::coordinates_to_screen_point};

pub const GROUND_COLOR: Color = Color::new(0.357, 0.055, 0.082, 1.);

pub fn render(game: &Game) {
  //
  let x = 0.;
  let y = coordinates_to_screen_point(&Coordinates::zero(), game).y;
  let w = screen_width();

  let h = screen_height() - y;
  let color = GROUND_COLOR;
  draw_rectangle(x, y, w, h, color);
}
