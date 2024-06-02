use std::collections::VecDeque;

use uuid::Uuid;

use super::{TreeNode, TreeNodeInput};

pub type ParentIdSiblingGroupTuple = (Option<Uuid>, Vec<Uuid>);

/*
  Tree
*/
#[derive(Debug, Clone)]
pub struct Tree<T: Clone + std::fmt::Debug> {
  pub nodes: Vec<TreeNode<T>>,
  pub root_node_id: Option<Uuid>,
}

impl<T: Clone + std::fmt::Debug> Tree<T> {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new(),
      root_node_id: None,
    }
  }

  pub fn node_is_leaf_node(&self, id: Uuid) -> bool {
    self.get_children_ids_for_node_id(id).is_empty()
  }

  pub fn find_node_by_id(&self, id: Uuid) -> Option<&TreeNode<T>> {
    self.nodes.iter().find(|node| node.id == id)
  }

  pub fn find_node_by_id_mut(&mut self, id: Uuid) -> Option<&mut TreeNode<T>> {
    self.nodes.iter_mut().find(|node| node.id == id)
  }

  pub fn find_nodes_by_ids(&self, ids: &Vec<Uuid>) -> Vec<&TreeNode<T>> {
    self
      .nodes
      .iter()
      .filter(|node| ids.contains(&node.id))
      .collect()
  }

  pub fn find_nodes_by_ids_mut(&mut self, ids: &Vec<Uuid>) -> Vec<&mut TreeNode<T>> {
    self
      .nodes
      .iter_mut()
      .filter(|node| ids.contains(&node.id))
      .collect()
  }

  pub fn get_children_ids_for_node_id(&self, id: Uuid) -> Vec<Uuid> {
    self
      .nodes
      .iter()
      .filter(|tree_node| {
        if let Some(parent_id) = tree_node.parent_id {
          parent_id == id
        } else {
          false
        }
      })
      .map(|tree_node| tree_node.id)
      .collect::<Vec<_>>()
  }

  pub fn get_children_for_node(&self, node: &TreeNode<T>) -> Vec<&TreeNode<T>> {
    let children_ids = self.get_children_ids_for_node_id(node.id);
    self.find_nodes_by_ids(&children_ids)
  }

  pub fn get_sibling_ids_for_node_id(&self, id: Uuid) -> Vec<Uuid> {
    let element = self.find_node_by_id(id).unwrap();

    match element.parent_id {
      Some(parent_id) => self
        .get_children_ids_for_node_id(parent_id)
        .clone(),
      None => Vec::new(),
    }
  }

  pub fn get_siblings_for_node(&self, node: &TreeNode<T>) -> Vec<&TreeNode<T>> {
    let sibling_ids = self.get_sibling_ids_for_node_id(node.id);
    self.find_nodes_by_ids(&sibling_ids)
  }

  pub fn get_node_ids_grouped_by_depth_top_down_flat(&self) -> Vec<Uuid> {
    self
      .get_nodes_grouped_by_siblings_top_down()
      .into_iter()
      .map(|(_, siblings)| siblings)
      .flatten()
      .collect::<Vec<_>>()
  }

  pub fn get_node_ids_grouped_by_depth_bottom_up_flat(&self) -> Vec<Uuid> {
    self
      .get_nodes_grouped_by_siblings_top_down()
      .into_iter()
      .map(|(_, siblings)| siblings)
      .rev()
      .flatten()
      .collect::<Vec<_>>()
  }

  pub fn get_leaf_node_ids_flat(&self) -> Vec<Uuid> {
    self
      .nodes
      .iter()
      .filter(|node| self.get_children_ids_for_node_id(node.id).len() == 0)
      .map(|node| node.id)
      .collect::<Vec<Uuid>>()
      .clone()
  }

  // TODO - don't include element_id
  pub fn get_all_descendant_ids_flat(&self, element_id: Uuid) -> Vec<Uuid> {
    let mut result: Vec<Uuid> = Vec::new();

    let mut queue: VecDeque<Uuid> = VecDeque::new();
    queue.push_back(element_id);

    while let Some(id) = queue.pop_front() {
      if id != element_id {
        result.push(id.clone());
      }

      let node = self.find_node_by_id(id);

      if let Some(node) = node {
        // Add children to queue and recurse
        for child_id in self.get_children_ids_for_node_id(node.id) {
          queue.push_back(child_id);
        }
      }
    }

    result
  }

  // (Option<parent_id>, Vec<siblings>)
  pub fn get_nodes_grouped_by_siblings_top_down(&self) -> Vec<ParentIdSiblingGroupTuple> {
    let mut result: Vec<(Option<Uuid>, Vec<Uuid>)> = Vec::new();
    // nodes that have already been added to result
    let mut visited_nodes: Vec<Uuid> = Vec::new();
    let mut queue: VecDeque<Uuid> = VecDeque::new();

    result.push((None, vec![self.root_node_id.unwrap()]));
    queue.push_back(self.root_node_id.unwrap());

    while let Some(node_id) = queue.pop_front() {
      if visited_nodes.contains(&node_id) {
        continue;
      }
      visited_nodes.push(node_id);

      let children_ids = self.get_children_ids_for_node_id(node_id);

      for child_id in children_ids.iter() {
        queue.push_back(*child_id);
      }

      if children_ids.len() > 0 {
        result.push((Some(node_id), children_ids));
      }
    }

    result
  }

  pub fn get_nodes_grouped_by_siblings_bottom_up(&self) -> Vec<(Option<Uuid>, Vec<Uuid>)> {
    self
      .get_nodes_grouped_by_siblings_top_down()
      .into_iter()
      .rev()
      .collect::<Vec<_>>()
  }

  /*
   * Mutations
   */

  //  TODO - return value should return children ids too probably
  pub fn add_node(&mut self, input: TreeNodeInput<T>, parent_id: Option<Uuid>) -> Uuid {
    let TreeNodeInput(data, children) = input;

    let node = TreeNode::new(data, parent_id);

    let node_id = node.id.clone();
    self.nodes.push(node);

    // recursively add children
    for child in children {
      self.add_node(child, Some(node_id));
    }

    node_id
  }

  pub fn add_nodes(&mut self, input_tuples: Vec<(TreeNodeInput<T>, Option<Uuid>)>) -> Vec<Uuid> {
    input_tuples
      .into_iter()
      .map(|(input, parent_id)| self.add_node(input, parent_id))
      .collect::<Vec<_>>()
  }

  pub fn remove_node_by_id(&mut self, element_id: Uuid) -> Option<Uuid> {
    if let Some(idx) = self
      .nodes
      .iter()
      .position(|node| node.id == element_id)
    {
      let node = self.nodes.get(idx).unwrap();
      let returned_id = node.id.clone();
      self.nodes.remove(idx);
      Some(returned_id)
    } else {
      None
    }
  }

  pub fn remove_nodes_by_ids(&mut self, element_ids: Vec<Uuid>) -> Vec<Uuid> {
    let returned_ids = element_ids
      .iter()
      // filter out potentially non-existent nodes
      .filter(|id| self.find_node_by_id(**id).is_some())
      // remove descendants as well
      .flat_map(|id| {
        let mut result = vec![*id];
        result.append(&mut self.get_all_descendant_ids_flat(*id));
        result
      })
      .map(|id| id.clone())
      .collect::<Vec<_>>();

    self
      .nodes
      .retain(|node| !element_ids.contains(&node.id));

    returned_ids
  }
  pub fn remove_node(&mut self, node_id: Uuid) {
    let element_ids_to_remove = self.get_all_descendant_ids_flat(node_id);

    // Delete node
    self
      .nodes
      .retain(|element| !&element_ids_to_remove.contains(&element.id));

    self.remove_nodes_by_ids(element_ids_to_remove);
  }

  pub fn replace_node(&mut self, node: &mut TreeNode<T>) {
    self.nodes = self
      .nodes
      .iter()
      .map(|other_node| {
        if node.id == other_node.id {
          node.clone()
        } else {
          other_node.clone()
        }
      })
      .collect::<Vec<_>>();
  }
}
