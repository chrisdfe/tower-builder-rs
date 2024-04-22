use macroquad::color::RED;
use uuid::Uuid;

use crate::{
  game::{
    ui::elements::{BackgroundColorKind, Elements},
    Game,
  },
  utils::get_random_color,
};

pub fn prerender(game: &mut Game, mut elements_replica: &mut Elements) {
  // Background color
  for layer in game.ui.elements.layers.iter() {
    for node_id in layer
      .tree
      .get_node_ids_grouped_by_depth_bottom_up_flat()
    {
      if needs_prerender(&game, node_id) {
        let node = layer.tree.find_node_by_id_mut(node_id).unwrap();

        node.data.calculated.background_color = match node.data.config.background_color {
          // TODO - transparent
          BackgroundColorKind::None => Some(RED),
          BackgroundColorKind::Fixed(color) => Some(color),
          BackgroundColorKind::Randomized => Some(get_random_color()),
        };
      }
    }
  }
}

fn needs_prerender(game: &Game, node_id: Uuid) -> bool {
  let (_, node) = game
    .ui
    .elements
    .layers
    .find_node_by_id(node_id)
    .unwrap();

  if node.data.calculated.background_color.is_none() {
    return true;
  };

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
