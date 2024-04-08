pub mod elements;
use elements::Elements;

pub mod selection;
use selection::Selection;

pub struct Ui {
  pub elements: Elements,
  pub selection: Selection,
}

impl Default for Ui {
  fn default() -> Self {
    Ui::new()
  }
}

impl Ui {
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
