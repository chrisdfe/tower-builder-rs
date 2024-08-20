use macroquad::prelude::Color;
use macroquad::rand::RandomRange;
use macroquad::window::{screen_height, screen_width};

use crate::constants::{CELL_HEIGHT, CELL_WIDTH};
use crate::game::Game;
use crate::types::map::Coordinates;
use crate::types::measurements::Point;

// TODO - put in point or coordinates
pub fn screen_point_to_cell(point: &Point, game: &Game) -> Coordinates {
  let x = (
      (
        // base point
        point.x as i32
        // center 0,0 to the middle of the screen
        - (screen_width() as i32 / 2)
        // 
        - (CELL_WIDTH as i32 / 2)
      )
      / CELL_WIDTH as i32
    )
    // account for camera position
    + game.world.camera.camera_position.x;
  let y = (
      ((
        // base point
        point.y as i32
        // center 0,0 to the middle of the screen
        - (screen_height() as i32 / 2)
        // 
        + (CELL_HEIGHT as i32 / 2)
      )
      / CELL_HEIGHT as i32)
      // floors go in the opposite direction to screen coordinates
      * -1
    )
    // account for camera position
    + game.world.camera.camera_position.y;

  Coordinates { x, y }
}

// TODO - this game param can probably just be Camera
// TODO - this should be a fn on impl Coordinates and/or Point
pub fn coordinates_to_screen_point(cell: &Coordinates, game: &Game) -> Point {
  let x =
    //-
    (cell.x as f32 * CELL_WIDTH as f32)
    //-
    - (game.world.camera.camera_position.x as f32 * CELL_WIDTH as f32)
    //-
    + (screen_width() / 2.)
    //-
    - (CELL_WIDTH as f32);

  let y =
    //-
    (cell.y as f32 * -1. * CELL_HEIGHT as f32)
    //-
    + (game.world.camera.camera_position.y as f32 * CELL_HEIGHT as f32)
    //-
    + (screen_height() / 2.)
    //-
    - (CELL_HEIGHT as f32 / 2.);

  Point { x, y }
}

// TODO - figure out how to accept other number types
// final_connector_word = "or", "and", etc
pub fn comma_seperate_number_vec(numbers: Vec<u32>, final_connector_word: String) -> String {
  let mut result = String::from("");

  let len = numbers.len();
  for (index, number) in numbers.into_iter().enumerate() {
    if index < len - 1 {
      result += format!("{}, ", number).as_str();
    } else {
      result += format!(" {} {}", final_connector_word, number).as_str();
    }
  }

  result
}

pub fn round_to_nearest(i: u32, multiple: u32) -> u32 {
  ((i + (multiple - 1)) / multiple) * multiple
}

pub fn get_random_color() -> Color {
  let r: f32 = RandomRange::gen_range(0., 1.);
  let g: f32 = RandomRange::gen_range(0., 1.);
  let b: f32 = RandomRange::gen_range(0., 1.);
  Color::new(r, g, b, 1.)
}
