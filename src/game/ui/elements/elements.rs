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

  // TODO - should this be on Layers?
  pub fn find_node_by_id(&self, id: Uuid) -> Option<(LayerTitle, &TreeNode<Element>)> {
    for layer in self.layers.iter() {
      if let Some(node) = layer.tree.find_node_by_id(id) {
        return Some((layer.title, node));
      }
    }

    None
  }

  // TODO - should this be on Layers?
  pub fn get_children_ids_for_node_id(&self, id: Uuid) -> Option<(LayerTitle, Vec<Uuid>)> {
    for layer in self.layers.iter() {
      if let Some(_) = layer.tree.find_node_by_id(id) {
        let ids = layer.tree.get_children_ids_for_node_id(id);
        return Some((layer.title, ids));
      }
    }

    None
  }

  /// Returns the id of the first matching interactive ui element that screen_point is inside of
  /// Assumes outer_dimensions ond outer_position of every node is not None
  pub fn find_interactive_node_at_screen_point(self: &Self, screen_point: &Point) -> Option<Uuid> {
    // TODO - allow other layers to be interactable
    //        just the front layer for now
    let front_layer = self.layers.front();
    if front_layer.is_none() {
      return None;
    }
    let front_layer = front_layer.unwrap();

    // Check for overlap from leaf nodes -> up to root to correctly
    let node_ids = front_layer
      .tree
      .get_node_ids_grouped_by_depth_bottom_up_flat();

    let matching_node = node_ids
      .into_iter()
      .map(|node_id| self.find_node_by_id(node_id).unwrap())
      // discared unused layer title
      .map(|(_, node)| node)
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
