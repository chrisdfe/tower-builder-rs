use crate::game::slices::world::tower::rooms::validation::RoomValidationContext;
use crate::game::Game;

pub fn update(game: &mut Game) {
  if game.ui.selection.selected_cell_has_changed() {
    game
      .tools
      .blueprint_room
      .calculate_coordinates_box(&game.ui.selection.selection_box);

    game
      .tools
      .blueprint_room
      .validate(RoomValidationContext {
        tower: &game.world.tower.tower,
        wallet: &game.world.wallet,
      });
  }
}
