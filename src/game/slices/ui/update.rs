use macroquad::input::mouse_position;

use crate::{game::Game, types::measurements::Point};

use crate::{
  game::slices::ui::elements::{interactivity::QueuedAction, Element},
  types::tree::TreeNode,
};
use uuid::Uuid;

use super::actions::{
  perform_element_mutation, ElementAction, ElementActionCreator, ElementActionCreatorCtx,
};

pub fn update(game: &mut Game) {
  run_update_handlers(game);
  update_interactivity(game);
  // run_event_handlers(game);
}

fn run_update_handlers(game: &mut Game) {
  let mut mutations: Vec<(Uuid, ElementAction)> = Vec::new();

  // run on_updates
  {
    for node_id in game
      .ui
      .elements
      .tree
      .get_node_ids_grouped_by_depth_top_down_flat()
    {
      let element = game
        .ui
        .elements
        .tree
        .find_node_by_id(node_id)
        .unwrap();

      if let Some(on_update) = element.data.on_update {
        let ctx = ElementActionCreatorCtx {
          world: &game.world,
          tools: &game.tools,
          ui: &game.ui,
        };

        let action = on_update(ctx, &element.data);

        match action {
          ElementAction::None => (),
          _ => mutations.push((node_id, action)),
        };
      }
    }
  }

  // Run mutations
  for (node_id, action) in mutations.into_iter() {
    perform_element_mutation(game, node_id, action);
  }
}

fn update_interactivity(game: &mut Game) {
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
            .config
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
            .config
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
            .config
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
            .config
            .on_mouse_up
        };

        maybe_enqueue_action(game, action_creator, *prev_id);
      }
      _ => (),
    }
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
    if game.input.left_mouse_is_down {
      // remain clicked
      Some(current_id)
    } else {
      // Transition to unclicked
      None
    }
  } else {
    // Check if we should transition to clicked
    if let Some(hovered_ui_element) = game.ui.elements.hovered_element_id.current {
      if game.input.left_mouse_is_down {
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

pub fn post_update(game: &mut Game) {
  for node_id in game
    .ui
    .elements
    .tree
    .get_node_ids_grouped_by_depth_top_down_flat()
  {
    let node = game
      .ui
      .elements
      .tree
      .find_node_by_id_mut(node_id)
      .unwrap();

    if let Some(interactivity) = &mut node.data.interactivity {
      interactivity.state.is_active.tick();
    }
  }

  //
  if game.ui.state.status_text.has_changed() {
    game.ui.state.status_text.tick();
  }
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

fn maybe_enqueue_action(
  game: &mut Game,
  action_creator: Option<ElementActionCreator>,
  node_id: Uuid,
) {
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
