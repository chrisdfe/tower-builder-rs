use crate::types::PrevAndCurrent;

use super::{elements::interactivity, Elements};

pub struct UIState {
  pub status_text: PrevAndCurrent<String>,
}

impl UIState {
  pub fn new() -> Self {
    Self {
      status_text: PrevAndCurrent::new(String::from("")),
    }
  }
}

pub struct Slice {
  pub elements: Elements,
  pub state: UIState,
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
      state: UIState::new(),
    }
  }

  pub fn mouse_is_over_ui(&self) -> bool {
    self.elements.hovered_element_id.current.is_some()
  }
}
