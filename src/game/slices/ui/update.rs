use macroquad::input::mouse_position;

use crate::{
  game::{slices::ui::elements::UpdateCtx, Game},
  types::map::CoordinatesBox,
  types::measurements::Point,
  utils::screen_point_to_cell,
};

use crate::{
  game::slices::ui::elements::{
    interactivity::{ActionCreator, QueuedAction},
    Element,
  },
  types::tree::TreeNode,
};
use uuid::Uuid;

pub fn update(game: &mut Game) {
  update_selection(game);
  run_update_handlers(game);
  update_interactivity(game);
  // run_event_handlers(game);
}

fn run_update_handlers(game: &mut Game) {
  let ctx = UpdateCtx {
    world: &game.world,
    tools: &game.tools,
    camera_position: &game.world.camera.camera_position,
  };

  for element_id in game
    .ui
    .elements
    .tree
    .get_node_ids_grouped_by_depth_top_down_flat()
  {
    let element = game
      .ui
      .elements
      .tree
      .find_node_by_id_mut(element_id)
      .unwrap();

    if let Some(on_update) = element.data.on_update {
      on_update(&ctx, &mut element.data);
    }
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
  game.ui.selection.previous_selected_cell = game.ui.selection.current_selected_cell.clone();
  game.ui.selection.current_selected_cell = {
    let (mouse_x, mouse_y) = mouse_position();
    screen_point_to_cell(
      &Point {
        x: mouse_x,
        y: mouse_y,
      },
      game,
    )
  };

  if game.ui.selection.selected_cell_has_changed() {
    if game.input.left_mouse_is_down {
      game.ui.selection.selection_box_end = game.ui.selection.current_selected_cell.clone();
      game.ui.selection.selection_box = CoordinatesBox::from_start_and_end_coords(
        &game.ui.selection.selection_box_start,
        &game.ui.selection.selection_box_end,
      );
    } else {
      game.ui.selection.selection_box =
        CoordinatesBox::at_coords(&game.ui.selection.current_selected_cell)
    };
  }
}

pub fn update_interactivity(game: &mut Game) {
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
