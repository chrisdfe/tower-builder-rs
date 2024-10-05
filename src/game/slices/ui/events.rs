use crate::game::Game;

use crate::game::slices::ui::elements::interactivity::QueuedAction;

use super::actions::{perform_element_mutation, run_action_creator};

pub fn run_event_handlers(game: &mut Game) {
  while let Some(queued_action) = game.ui.elements.event_handler_queue.pop() {
    let QueuedAction {
      action_creator,
      node_id,
    } = queued_action;

    // TODO - don't unwrap
    let tree_node = game
      .ui
      .elements
      .tree
      .find_node_by_id(node_id)
      .unwrap();

    let action = run_action_creator(game, &tree_node.data, action_creator);
    perform_element_mutation(game, tree_node.id, action);
  }
}
