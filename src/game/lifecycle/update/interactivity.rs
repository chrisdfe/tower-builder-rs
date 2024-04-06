use crate::game::{ui::elements, Game};
use macroquad::input::mouse_position;

use crate::measurements::Point;

pub fn update(game: &mut Game) {
  calculate_hovered_ui_element(game);
}

pub fn calculate_hovered_ui_element(game: &mut Game) {
  let (mouse_x, mouse_y) = mouse_position();

  let mouse_point = Point {
    x: mouse_x,
    y: mouse_y,
  };

  let hovered_button_id = game
    .ui
    .elements
    .find_interactive_node_at_screen_point(&mouse_point);

  game
    .ui
    .elements
    .hovered_element_id
    .set_maybe_current(hovered_button_id);
}
