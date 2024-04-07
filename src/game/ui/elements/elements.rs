use uuid::Uuid;

use crate::{
  measurements::Point,
  types::{
    tree::{Tree, TreeNodeInput},
    PrevAndCurrent,
  },
};

use super::{factories, Element};

// TODO - implement Iterator
#[derive(Debug, Clone)]
pub struct Elements {
  pub tree: Tree<Element>,

  pub hovered_element_id: PrevAndCurrent<Uuid>,
  pub clicked_element_id: PrevAndCurrent<Uuid>,
}

impl Elements {
  pub fn new() -> Self {
    let mut tree = Tree::new();

    // Add root node
    let root_element_id = tree.add_node(
      TreeNodeInput {
        data: factories::create_root_node_element(),
        children: Vec::new(),
      },
      None,
    );

    tree.root_node_id = Some(root_element_id);

    Self {
      tree,

      hovered_element_id: PrevAndCurrent::new_none(),
      clicked_element_id: PrevAndCurrent::new_none(),
    }
  }

  /// Returns the id of the first matching interactive ui element that screen_point is inside of
  /// Assumes outer_dimensions ond outer_position on every node is not None
  pub fn find_interactive_node_at_screen_point(self: &Self, screen_point: &Point) -> Option<Uuid> {
    // Check for overlap from leaf nodes -> up to root to correctly
    let node_ids = self
      .tree
      .get_node_ids_grouped_by_depth_bottom_up_flat();

    let matching_node = node_ids
      .into_iter()
      .map(|node_id| self.tree.find_node_by_id(node_id).unwrap())
      // filter out non-interactive elements
      .filter(|node| node.data.config.is_interactive)
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