use uuid::Uuid;

use crate::{
  measurements::Point,
  types::{tree::TreeNode, PrevAndCurrent},
};

use super::{
  interactivity::EventHandlerQueue,
  layers::{LayerTitle, Layers},
  Element,
};

// TODO - implement Iterator
#[derive(Debug, Clone)]
pub struct Elements {
  pub layers: Layers,

  pub event_handler_queue: EventHandlerQueue,

  pub hovered_element_id: PrevAndCurrent<Uuid>,
  pub clicked_element_id: PrevAndCurrent<Uuid>,
}

impl Elements {
  pub fn new() -> Self {
    Self {
      layers: Layers::new(),

      event_handler_queue: EventHandlerQueue::new(),

      hovered_element_id: PrevAndCurrent::new_none(),
      clicked_element_id: PrevAndCurrent::new_none(),
    }
  }

  /// Returns the id of the first matching interactive ui element that screen_point is inside of
  /// Assumes outer_dimensions ond outer_position of every node is not None
  pub fn find_interactive_node_at_screen_point(self: &Self, screen_point: &Point) -> Option<Uuid> {
    // TODO - allow other layers to be interactable
    //        just the front layer for now
    let layer = self.layers.front();
    if layer.is_none() {
      return None;
    }
    let layer = layer.unwrap();

    // Check for overlap from leaf nodes -> up to root to correctly
    let node_ids = layer
      .tree
      .get_node_ids_grouped_by_depth_bottom_up_flat();

    let matching_node = node_ids
      .into_iter()
      .map(|node_id| layer.tree.find_node_by_id(node_id).unwrap())
      // filter out non-interactive elements
      .filter(|node| node.data.config.is_interactive())
      // filter out elements that have not been precalculated yet
      .filter(|node| {
        node.data.calculated.outer_position.is_some()
          && node.data.calculated.outer_dimensions.is_some()
      })
      // find element that intersects point
      .find(|node| {
        node
          .data
          .calculated
          .outer_as_rect()
          .contains_point(&screen_point)
      });

    if let Some(matching_node) = matching_node {
      Some(matching_node.id)
    } else {
      None
    }
  }

  pub fn clear_all_calculated_properties(&mut self) {
    for layer in self.layers.iter() {
      for node in layer.tree.iter_mut() {
        node.data.calculated.clear();
      }
    }
  }
}
