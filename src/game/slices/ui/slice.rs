use super::Elements;

pub struct Slice {
  pub elements: Elements,
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
    }
  }

  pub fn mouse_is_over_ui(&self) -> bool {
    self.elements.hovered_element_id.current.is_some()
  }
}
