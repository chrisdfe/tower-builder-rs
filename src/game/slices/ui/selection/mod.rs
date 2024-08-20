use crate::types::map::{Coordinates, CoordinatesBox};

pub struct Selection {
  pub selection_box: CoordinatesBox,
  pub selection_box_start: Coordinates,
  pub selection_box_end: Coordinates,
  pub previous_selected_cell: Coordinates,
  pub current_selected_cell: Coordinates,
}

impl Selection {
  pub fn new() -> Self {
    Self {
      selection_box: CoordinatesBox::at_coords(&Coordinates::zero()),
      selection_box_start: Coordinates::zero(),
      selection_box_end: Coordinates::zero(),
      previous_selected_cell: Coordinates::zero(),
      current_selected_cell: Coordinates::zero(),
    }
  }

  pub fn selected_cell_has_changed(&self) -> bool {
    self.previous_selected_cell != self.current_selected_cell
  }
}
