use uuid::Uuid;

use crate::types::tree::{Tree, TreeNode, TreeNodeInput};

use super::{factories, Element};

#[derive(Debug, Clone)]
pub struct Layers {
  // I might want to make this a Tree later, but keeping it simple for now
  layers: Vec<Layer>,
}

impl Layers {
  pub fn new() -> Self {
    Self { layers: Vec::new() }
  }

  pub fn iter(&self) -> impl Iterator<Item = &Layer> {
    self.layers.iter()
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Layer> {
    self.layers.iter_mut()
  }

  pub fn back(&self) -> Option<&Layer> {
    self.layers.first()
  }

  pub fn front(&self) -> Option<&Layer> {
    self.layers.last()
  }

  pub fn find_node_by_id(&self, id: Uuid) -> Option<(LayerTitle, &TreeNode<Element>)> {
    for layer in self.iter() {
      if let Some(node) = layer.tree.find_node_by_id(id) {
        return Some((layer.title, node));
      }
    }

    None
  }

  pub fn find_node_by_id_mut(&mut self, id: Uuid) -> Option<(LayerTitle, &TreeNode<Element>)> {
    for layer in self.iter_mut() {
      if let Some(node) = layer.tree.find_node_by_id(id) {
        return Some((layer.title, node));
      }
    }

    None
  }

  pub fn get_children_ids_for_node_id(&self, id: Uuid) -> Option<(LayerTitle, Vec<Uuid>)> {
    for layer in self.iter() {
      if let Some(_) = layer.tree.find_node_by_id(id) {
        let ids = layer.tree.get_children_ids_for_node_id(id);
        return Some((layer.title, ids));
      }
    }

    None
  }
}

#[derive(Debug, Clone)]
pub struct Layer {
  pub title: LayerTitle,
  // TODO - make private
  pub tree: Tree<Element>,
}

impl Layer {
  pub fn new(title: LayerTitle) -> Self {
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

    Self { title, tree }
  }

  // /*
  // Tree passthrough fns
  // */
  // pub fn iter(&self) -> impl Iterator<Item = &TreeNode<Element>> {
  //   self.tree.iter()
  // }

  // pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TreeNode<Element>> {
  //   self.tree.iter_mut()
  // }

  // pub fn find_node_by_id(&self, id: Uuid) -> Option<&TreeNode<Element>> {
  //   self.tree.find_node_by_id(id)
  // }

  // pub fn get_node_ids_grouped_by_depth_bottom_up_flat(&self) -> Vec<Uuid> {
  //   self
  //     .tree
  //     .get_node_ids_grouped_by_depth_bottom_up_flat()
  // }

  // pub fn get_children_ids_for_node_id(&self, id: Uuid) -> Vec<Uuid> {
  //   self.tree.get_children_ids_for_node_id(id)
  // }
}

#[derive(Debug, Clone)]
pub enum LayerTitle {
  None,
  HUD,
}
