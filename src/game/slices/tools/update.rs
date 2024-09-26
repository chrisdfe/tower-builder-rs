use macroquad::input::mouse_position;

use crate::game::slices::world::tower::rooms::validation::RoomValidationContext;
use crate::game::Game;

use super::slice::Tool;

pub fn update(game: &mut Game) {
  if game.tools.selection.selected_cell_has_changed() {
    match &mut game.tools.tool {
      Tool::Build(build_tool) => {
        build_tool
          .blueprint_room
          .calculate_coordinates_box(&game.tools.selection.selection_box());

        build_tool
          .blueprint_room
          .validate(RoomValidationContext {
            tower: &game.world.tower.tower,
            wallet: &game.world.wallet,
          });
      }
      _ => (),
    }
  }

  super::selection::update(game);
}
