use crate::game::slices::world::tower::rooms::validation::RoomValidationContext;
use crate::game::Game;

use crate::game::slices::ui::elements::interactivity::{self, ActionCreatorCtx, QueuedAction};
use crate::types::map::CoordinatesBox;

pub fn run_event_handlers(game: &mut Game) {
  while let Some(queued_action) = game.ui.elements.event_handler_queue.pop() {
    let QueuedAction {
      action_creator,
      node_id,
    } = queued_action;

    let tree_node = game.ui.elements.tree.find_node_by_id(node_id);
    // TODO - don't unwrap
    let element = &tree_node.unwrap().data;

    let action = action_creator(ActionCreatorCtx { element: &element });
    use interactivity::Action::*;
    match action {
      None => {
        // no-op
      }
      PrintDebugStatement => {
        println!("debug statement. {}", node_id);
      }
      SetSelectedRoomDefinition(room_definition_id) => {
        // game.tools.selected_room_definition_id = definition_id;
        // TODO
        game.tools.set_selected_room_definition(
          room_definition_id,
          // TODO - work this out

          // Is this even neccessary???
          // game.tools.selection_box,
          &CoordinatesBox::at_coords(&game.tools.selection.current_selected_cell),
          RoomValidationContext {
            tower: &game.world.tower.tower,
            wallet: &game.world.wallet,
          },
        );
        println!(
          "selected_room_definition is now: {:?}",
          game.tools.selected_room_definition_id
        );
      }
    }
    // RemoveAllRootNodeChildren => {
    //   let ids = game
    //     .ui
    //     .elements
    //     .tree
    //     .get_children_ids_for_node_id(game.ui.elements.tree.root_node_id.unwrap());
    //   game.ui.elements.tree.remove_nodes_by_ids(ids);
    //   println!("removing all root node children");
    //   game.ui.elements.clear_all_calculated_properties();
    // }
  }
}
