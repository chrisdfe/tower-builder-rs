use macroquad::input::mouse_position;

use crate::game::slices::world::tower::rooms::validation::RoomValidationContext;
use crate::game::Game;

use crate::{types::map::CoordinatesBox, types::measurements::Point, utils::screen_point_to_cell};

pub fn update(game: &mut Game) {
  update_selection(game);

  if game.tools.selection.selected_cell_has_changed() {
    game
      .tools
      .blueprint_room
      .calculate_coordinates_box(&game.tools.selection.selection_box);

    game
      .tools
      .blueprint_room
      .validate(RoomValidationContext {
        tower: &game.world.tower.tower,
        wallet: &game.world.wallet,
      });
  }
}

fn update_selection(game: &mut Game) {
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
  game.tools.selection.previous_selected_cell = game.tools.selection.current_selected_cell.clone();
  game.tools.selection.current_selected_cell = {
    let (mouse_x, mouse_y) = mouse_position();
    screen_point_to_cell(
      &Point {
        x: mouse_x,
        y: mouse_y,
      },
      game,
    )
  };

  if game.tools.selection.selected_cell_has_changed() {
    if game.input.left_mouse_is_down {
      game.tools.selection.selection_box_end = game.tools.selection.current_selected_cell.clone();
      game.tools.selection.selection_box = CoordinatesBox::from_start_and_end_coords(
        &game.tools.selection.selection_box_start,
        &game.tools.selection.selection_box_end,
      );
    } else {
      game.tools.selection.selection_box =
        CoordinatesBox::at_coords(&game.tools.selection.current_selected_cell)
    };
  }
}
