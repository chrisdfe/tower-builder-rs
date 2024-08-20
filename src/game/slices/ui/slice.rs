use super::{Elements, Selection};

pub struct Slice {
  pub elements: Elements,
  pub selection: Selection,
}

impl Default for Slice {
  fn default() -> Self {
    Slice::new()
  }
}

impl Slice {
  pub fn new() -> Self {
    Self {
      elements: Elements::new(),
      selection: Selection::new(),
    }
  }

  pub fn mouse_is_over_ui(&self) -> bool {
    self.elements.hovered_element_id.current.is_some()
  }
}
