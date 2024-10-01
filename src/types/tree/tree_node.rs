use uuid::Uuid;

#[derive(Clone)]
pub struct TreeNode<T: Clone> {
  pub id: Uuid,
  pub parent_id: Option<Uuid>,
  pub data: T,
}

impl<T: Clone> PartialEq for TreeNode<T> {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl<T: Clone> TreeNode<T> {
  pub fn new(data: T, parent_id: Option<Uuid>) -> TreeNode<T> {
    let id = Uuid::new_v4();

    TreeNode {
      id,
      parent_id,
      data,
    }
  }
}

// (node, children)
#[derive(Clone)]
pub struct TreeNodeInput<T>(pub T, pub Vec<TreeNodeInput<T>>);
