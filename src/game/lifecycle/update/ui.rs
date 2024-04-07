use macroquad::input::mouse_position;

use crate::{game::Game, map::CoordinatesBox, measurements::Point, utils::screen_point_to_cell};

pub fn update(game: &mut Game) {
  // game.ui.debug_text = format!(
  //   "{},{} | {} {} | {}",
  //   game.ui.current_selected_cell.x,
  //   game.ui.current_selected_cell.y,
  //   game.camera_position.x,
  //   game.camera_position.y,
  //   get_fps()
  // );

  // TODO - only if !ui.mouse_is_hover_ui()
  // Mouse position
  game.ui.selection.previous_selected_cell = game.ui.selection.current_selected_cell.clone();
  game.ui.selection.current_selected_cell = {
    let (mouse_x, mouse_y) = mouse_position();
    screen_point_to_cell(
      &Point {
        x: mouse_x,
        y: mouse_y,
      },
      game,
    )
  };

  if game.ui.selection.selected_cell_has_changed() {
    if game.left_mouse_is_down {
      game.ui.selection.selection_box_end = game.ui.selection.current_selected_cell.clone();
      game.ui.selection.selection_box = CoordinatesBox::from_start_and_end_coords(
        &game.ui.selection.selection_box_start,
        &game.ui.selection.selection_box_end,
      );
    } else {
      game.ui.selection.selection_box =
        CoordinatesBox::at_coords(&game.ui.selection.current_selected_cell)
    };
  }
}
