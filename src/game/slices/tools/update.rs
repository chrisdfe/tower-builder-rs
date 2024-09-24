use macroquad::input::mouse_position;

use crate::game::slices::world::tower::rooms::validation::RoomValidationContext;
use crate::game::Game;

use crate::{types::map::CoordinatesBox, types::measurements::Point, utils::screen_point_to_cell};

pub fn update(game: &mut Game) {
  if game.tools.selection.selected_cell_has_changed() {
    game
      .tools
      .blueprint_room
      .calculate_coordinates_box(&game.tools.selection.selection_box());

    game
      .tools
      .blueprint_room
      .validate(RoomValidationContext {
        tower: &game.world.tower.tower,
        wallet: &game.world.wallet,
      });
  }

  super::selection::update(game);
}
