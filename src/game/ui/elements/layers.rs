use crate::types::tree::{Tree, TreeNodeInput};

use super::{factories, Element};

#[derive(Debug, Clone)]
pub struct Layers {
  // I might want to make this a Tree later, but keeping it simple for now
  pub layers: Vec<Layer>,
}

impl Layers {
  pub fn new() -> Self {
    Self { layers: Vec::new() }
  }
}

#[derive(Debug, Clone)]
pub struct Layer {
  title: LayerTitle,
  element_tree: Tree<Element>,
}

impl Layer {
  pub fn new(title: LayerTitle) -> Self {
    let mut element_tree = Tree::new();

    // Add root node
    let root_element_id = element_tree.add_node(
      TreeNodeInput {
        data: factories::create_root_node_element(),
        children: Vec::new(),
      },
      None,
    );

    element_tree.root_node_id = Some(root_element_id);

    Self {
      title,
      element_tree,
    }
  }
}

#[derive(Debug, Clone)]
pub enum LayerTitle {
  None,
  HUD,
}
