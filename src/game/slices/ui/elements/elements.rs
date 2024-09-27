use uuid::Uuid;

use crate::types::{
  measurements::Point,
  tree::{Tree, TreeNode},
  PrevAndCurrent,
};

use super::{components, interactivity::EventHandlerQueue, Element, ElementHandle, ElementTag};

// TODO - implement Iterator
#[derive(Debug, Clone)]
pub struct Elements {
  pub tree: Tree<Element>,

  pub event_handler_queue: EventHandlerQueue,

  pub hovered_element_id: PrevAndCurrent<Option<Uuid>>,
  pub clicked_element_id: PrevAndCurrent<Option<Uuid>>,
}

impl Elements {
  pub fn new() -> Self {
    let mut tree = Tree::new();

    let root_element_id = tree.add_node(components::root_node::create(), None);
    // TODO - should this be the default behavior in Tree if parent_id is None?
    tree.root_node_id = Some(root_element_id);

    Self {
      tree,

      event_handler_queue: EventHandlerQueue::new(),

      hovered_element_id: PrevAndCurrent::new(None),
      clicked_element_id: PrevAndCurrent::new(None),
    }
  }

  pub fn get_current_hovered_element(&self) -> Option<&Element> {
    if let Some(id) = self.hovered_element_id.current {
      if let Some(node) = self.tree.find_node_by_id(id) {
        Some(&node.data)
      } else {
        None
      }
    } else {
      None
    }
  }

  pub fn get_current_clicked_element(&self) -> Option<&Element> {
    if let Some(id) = self.hovered_element_id.current {
      if let Some(node) = self.tree.find_node_by_id(id) {
        Some(&node.data)
      } else {
        None
      }
    } else {
      None
    }
  }

  pub fn find_node_by_handle(&self, handle: ElementHandle) -> Option<&TreeNode<Element>> {
    self
      .tree
      .nodes
      .iter()
      .find(|node| node.data.handle == handle)
  }

  pub fn find_nodes_by_tag(&self, tag: ElementTag) -> Vec<&TreeNode<Element>> {
    self
      .tree
      .nodes
      .iter()
      .filter(|node| node.data.tags.contains(&tag))
      .collect::<_>()
  }

  pub fn remove_node_by_handle(&mut self, handle: ElementHandle) {
    let element = self.find_node_by_handle(handle);

    if let Some(element) = element {
      let mut node_ids_to_remove = vec![element.id];

      let mut descendent_ids = self.tree.get_descendant_ids(element.id);

      node_ids_to_remove.append(&mut descendent_ids);

      self.tree.remove_nodes_by_ids(node_ids_to_remove);
    }
  }

  /// Returns the id of the first matching interactive ui element that screen_point is inside of
  /// Assumes outer_dimensions ond outer_position on every node is not None
  pub fn find_interactive_node_at_screen_point(self: &Self, screen_point: &Point) -> Option<Uuid> {
    // Check for overlap from leaf nodes -> up to root
    let node_ids = self
      .tree
      .get_node_ids_grouped_by_depth_bottom_up_flat();

    let matching_node = node_ids
      .into_iter()
      .map(|node_id| self.tree.find_node_by_id(node_id).unwrap())
      // filter out non-interactive elements
      .filter(|node| node.data.interactivity.is_some())
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
    for node in self.tree.nodes.iter_mut() {
      node.data.calculated.clear();
    }
  }
}
