use uuid::Uuid;

use crate::types::tree::{Tree, TreeNodeInput};

use super::{factories, Element};

// TODO - implement Iterator
#[derive(Debug, Clone)]
pub struct Elements {
  pub tree: Tree<Element>,

  pub hovered_element_id: Option<Uuid>,
  pub clicked_element_id: Option<Uuid>,
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

      hovered_element_id: None,
      clicked_element_id: None,
    }
  }

  pub fn add_node(element: Element, parent_id: Option<Uuid>) {}

  pub fn clear_all_calculated_properties(&mut self) {
    for node in self.tree.nodes.iter_mut() {
      node.data.calculated.clear();
    }
  }
}
