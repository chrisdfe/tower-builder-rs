use crate::game::slices::tools::Tool;
use crate::game::slices::ui::ElementHandle;
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
        if let Tool::Build = &mut game.tools.tool.current {
          // game.tools.selected_room_definition_id = definition_id;
          // TODO
          game
            .tools
            .build_tool
            .set_selected_room_definition(
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
        }
      }
      SetCurrentTool(tool) => {
        use crate::game::slices::ui::elements::components::tools_panel::room_definitions_button_wrapper;
        game.tools.tool.set_current(tool);

        let parent_id = {
          if let Some(tools_panel_element) = game
            .ui
            .elements
            .find_node_by_handle(ElementHandle::ToolsPanel)
          {
            Some(tools_panel_element.id)
          } else {
            Option::None
          }
        };

        // TODO - might want to move elsewhere at some point to decouple UI from other logic
        // Add/remove room definitions buttons
        if tool == Tool::Build {
          if let Some(parent_id) = parent_id {
            game
              .ui
              .elements
              .tree
              .prepend_node(room_definitions_button_wrapper::create(), Some(parent_id));
          }
        } else {
          game
            .ui
            .elements
            .remove_node_by_handle(ElementHandle::RoomDefinitionButtonsWrapper);
        }
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
