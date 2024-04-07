use crate::game::{
  ui::elements::{
    interactivity::{Action, QueuedAction},
    Elements,
  },
  Game,
};
use macroquad::input::mouse_position;

use crate::measurements::Point;

pub fn update(game: &mut Game) {
  calculate_hovered_ui_element(game);
  calculate_clicked_ui_element(game);

  let hovered_element_id = game.ui.elements.hovered_element_id.clone();
  let clicked_element_id = game.ui.elements.clicked_element_id.clone();

  if hovered_element_id.has_changed() {
    if hovered_element_id.prev.is_none() && hovered_element_id.current.is_some() {
      // on mouse over
      let node_id = hovered_element_id.current.unwrap();

      let action = {
        let node = game
          .ui
          .elements
          .tree
          .find_node_by_id(node_id)
          .unwrap();
        node.data.config.event_handlers.on_mouse_over
      };

      game
        .ui
        .elements
        .event_handler_queue
        .push(QueuedAction { action, node_id });
    } else if hovered_element_id.current.is_none() && hovered_element_id.prev.is_some() {
      // on mouse up
      let node_id = hovered_element_id.prev.unwrap();
      let action = {
        let node = game
          .ui
          .elements
          .tree
          .find_node_by_id(node_id)
          .unwrap();
        node.data.config.event_handlers.on_mouse_out
      };

      // on mouse over
      game
        .ui
        .elements
        .event_handler_queue
        .push(QueuedAction { action, node_id });
    }
  }

  if clicked_element_id.has_changed() {
    if clicked_element_id.prev.is_none() && clicked_element_id.current.is_some() {
      // on mouse over
      let node_id = hovered_element_id.current.unwrap();

      let action = {
        let node = game
          .ui
          .elements
          .tree
          .find_node_by_id(node_id)
          .unwrap();
        node.data.config.event_handlers.on_mouse_down
      };

      game
        .ui
        .elements
        .event_handler_queue
        .push(QueuedAction { action, node_id });
    } else if clicked_element_id.current.is_none() && clicked_element_id.prev.is_some() {
      // on mouse up
      let node_id = hovered_element_id.prev.unwrap();

      let action = {
        let node = game
          .ui
          .elements
          .tree
          .find_node_by_id(node_id)
          .unwrap();
        node.data.config.event_handlers.on_mouse_up
      };

      game
        .ui
        .elements
        .event_handler_queue
        .push(QueuedAction { action, node_id });
    }
  }
}

pub fn run_event_handlers(game: &mut Game) {
  while let Some(queued_action) = game.ui.elements.event_handler_queue.pop() {
    let QueuedAction { action, node_id } = queued_action;
    println!("action_handler, {:?}, {}", action, node_id);

    // match action {}
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
    .set_maybe_current(hovered_button_id);
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
    .set_maybe_current(new_clicked_id);
}
