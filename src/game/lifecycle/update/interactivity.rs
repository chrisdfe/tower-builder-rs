use crate::{
  game::{
    ui::elements::{
      interactivity::{self, ActionCreator, ActionCreatorCtx, QueuedAction},
      Element,
    },
    Game,
  },
  types::tree::TreeNode,
};
use macroquad::input::mouse_position;
use uuid::Uuid;

use crate::measurements::Point;

pub fn update(game: &mut Game) {
  calculate_hovered_ui_element(game);
  calculate_clicked_ui_element(game);

  let hovered_element_id = game.ui.elements.hovered_element_id.clone();
  let clicked_element_id = game.ui.elements.clicked_element_id.clone();

  // Hover actions
  if hovered_element_id.has_changed() {
    match hovered_element_id.as_tuple() {
      (None, Some(current_id)) => {
        // on mouse over
        let action_creator = {
          let node = find_node(game, *current_id);
          node
            .data
            .interactivity
            .as_ref()
            .unwrap()
            .on_mouse_over
        };

        maybe_enqueue_action(game, action_creator, *current_id);
      }
      (Some(prev_id), None) => {
        // on mouse out
        let action_creator = {
          let node = find_node(game, *prev_id);
          node
            .data
            .interactivity
            .as_ref()
            .unwrap()
            .on_mouse_out
        };

        maybe_enqueue_action(game, action_creator, *prev_id);
      }
      _ => (),
    }
  }

  // Click actions
  if clicked_element_id.has_changed() {
    match clicked_element_id.as_tuple() {
      (None, Some(current_id)) => {
        // on mouse down
        let action_creator = {
          let node = find_node(game, *current_id);
          node
            .data
            .interactivity
            .as_ref()
            .unwrap()
            .on_mouse_down
        };

        maybe_enqueue_action(game, action_creator, *current_id);
      }
      (Some(prev_id), None) => {
        // on mouse up
        let action_creator = {
          let node = find_node(game, *prev_id);
          node
            .data
            .interactivity
            .as_ref()
            .unwrap()
            .on_mouse_up
        };

        maybe_enqueue_action(game, action_creator, *prev_id);
      }
      _ => (),
    }
  }
}

pub fn run_event_handlers(game: &mut Game) {
  while let Some(queued_action) = game.ui.elements.event_handler_queue.pop() {
    let QueuedAction {
      action_creator,
      node_id,
    } = queued_action;

    let action = action_creator(ActionCreatorCtx { node_id });
    use interactivity::Action::*;
    match action {
      None => {
        // no-op
      }
      PrintDebugStatement => {
        println!("debug statement. {}", node_id);
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

fn calculate_hovered_ui_element(game: &mut Game) {
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
    .set_current(hovered_button_id);
}

fn calculate_clicked_ui_element(game: &mut Game) {
  let new_clicked_id = if let Some(current_id) = game.ui.elements.clicked_element_id.current {
    // Check if we should transition to unclicked
    if game.left_mouse_is_down {
      // remain clicked
      Some(current_id)
    } else {
      // Transition to unclicked
      None
    }
  } else {
    // Check if we should transition to clicked
    if let Some(hovered_ui_element) = game.ui.elements.hovered_element_id.current {
      if game.left_mouse_is_down {
        // Transition to clicked
        Some(hovered_ui_element)
      } else {
        // remain unclicked
        None
      }
    } else {
      // remain unclicked
      None
    }
  };

  game
    .ui
    .elements
    .clicked_element_id
    .set_current(new_clicked_id);
}

/*
  Local helpers
*/
fn find_node(game: &Game, node_id: Uuid) -> &TreeNode<Element> {
  game
    .ui
    .elements
    .tree
    .find_node_by_id(node_id)
    .unwrap()
}

fn maybe_enqueue_action(game: &mut Game, action_creator: Option<ActionCreator>, node_id: Uuid) {
  if let Some(action_creator) = action_creator {
    game
      .ui
      .elements
      .event_handler_queue
      .push(QueuedAction {
        action_creator,
        node_id,
      });
  }
}
