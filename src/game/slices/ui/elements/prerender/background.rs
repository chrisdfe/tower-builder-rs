use macroquad::color::{Color, RED};
use uuid::Uuid;

use crate::{
  game::{
    slices::ui::elements::{BackgroundColorKind, Element, Elements},
    Game,
  },
  types::tree::TreeNode,
  utils::get_random_color,
};

pub fn prerender(game: &mut Game, mut elements_replica: &mut Elements) {
  // Background color
  for node_id in game
    .ui
    .elements
    .tree
    .get_node_ids_grouped_by_depth_bottom_up_flat()
  {
    let needs_prerender = needs_prerender(&game, node_id);

    if needs_prerender {
      let (base_background_color, is_interactive) = {
        let element = &game
          .ui
          .elements
          .tree
          .find_node_by_id_mut(node_id)
          .unwrap()
          .data;

        (element.background_color, element.interactivity.is_some())
      };

      let background_color = if is_interactive {
        get_interactive_color(game, node_id)
      } else {
        base_background_color
      };

      let node = game
        .ui
        .elements
        .tree
        .find_node_by_id_mut(node_id)
        .unwrap();

      node.data.calculated.background_color = match background_color {
        BackgroundColorKind::None => Some(Color::new(0., 0., 0., 0.)),
        BackgroundColorKind::Fixed(color) => Some(color),
        BackgroundColorKind::Randomized => Some(get_random_color()),
      }
    };
  }
}

fn needs_prerender(game: &Game, node_id: Uuid) -> bool {
  let node = game
    .ui
    .elements
    .tree
    .find_node_by_id(node_id)
    .unwrap();

  if node.data.calculated.background_color.is_none() {
    return true;
  };

  if let Some(interactivity) = &node.data.interactivity {
    if interactivity.state.is_active.has_changed() {
      return true;
    }
  }

  // hover state
  if game.ui.elements.hovered_element_id.has_changed() {
    if let Some(hovered_element_id) = game.ui.elements.hovered_element_id.current {
      if hovered_element_id == node.id {
        return true;
      }
    }

    if let Some(unhovered_element_id) = game.ui.elements.hovered_element_id.prev {
      if unhovered_element_id == node.id {
        return true;
      }
    }
  }

  // click state
  if game.ui.elements.clicked_element_id.has_changed() {
    if let Some(clicked_element_id) = game.ui.elements.clicked_element_id.current {
      if clicked_element_id == node.id {
        return true;
      }
    }

    if let Some(unclicked_element_id) = game.ui.elements.clicked_element_id.prev {
      if unclicked_element_id == node.id {
        return true;
      }
    }
  }

  false
}

fn get_interactive_color(game: &Game, node_id: Uuid) -> BackgroundColorKind {
  let node = game
    .ui
    .elements
    .tree
    .find_node_by_id(node_id)
    .unwrap();

  let interactivity = &node.data.interactivity.as_ref().unwrap();

  if interactivity.state.is_active.current {
    return interactivity
      .config
      .background_color_active
      .clone();
  }

  // hover state
  if game.ui.elements.hovered_element_id.has_changed() {
    if let Some(hovered_element_id) = game.ui.elements.hovered_element_id.current {
      if hovered_element_id == node.id {
        return interactivity.config.background_color_over.clone();
      }
    }

    if let Some(unhovered_element_id) = game.ui.elements.hovered_element_id.prev {
      if unhovered_element_id == node.id {
        return node.data.background_color.clone();
      }
    }
  }

  if game.ui.elements.clicked_element_id.has_changed() {
    if let Some(clicked_element_id) = game.ui.elements.clicked_element_id.current {
      if clicked_element_id == node.id {
        return interactivity.config.background_color_down.clone();
      }
    }

    if let Some(unclicked_element_id) = game.ui.elements.clicked_element_id.prev {
      if unclicked_element_id == node.id {
        return interactivity.config.background_color_up.clone();
      }
    }
  }

  BackgroundColorKind::Fixed(Color::new(1., 1., 1., 0.))
}
