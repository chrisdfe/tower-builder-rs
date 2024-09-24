use macroquad::input::mouse_position;

use crate::{
  game::Game,
  types::{
    map::{Coordinates, CoordinatesBox},
    measurements::Point,
  },
  utils::screen_point_to_cell,
};

pub struct Selection {
  selection_box: CoordinatesBox,
  selection_box_start: Coordinates,
  selection_box_end: Coordinates,

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

  pub fn selection_box(&self) -> &CoordinatesBox {
    &self.selection_box
  }

  fn recalculate_selection_box(&mut self) {
    // TODO - "reset_at_coords" or something instead of reallocating
    self.selection_box =
      CoordinatesBox::from_start_and_end_coords(&self.selection_box_start, &self.selection_box_end);
  }

  fn _set_start(&mut self, coords: &Coordinates) {
    self.selection_box_start = coords.clone();
    self.recalculate_selection_box();
  }

  fn set_end(&mut self, coords: &Coordinates) {
    self.selection_box_end = coords.clone();
    self.recalculate_selection_box();
  }

  pub fn start_selection_box_at_current_cell(&mut self) {
    self.selection_box_start = self.current_selected_cell.clone();
    self.selection_box_end = self.current_selected_cell.clone();
    self.recalculate_selection_box();
  }

  pub fn set_selection_box_end_to_current_cell(&mut self) {
    self.set_end(&self.current_selected_cell.clone());
  }

  pub fn selected_cell_has_changed(&self) -> bool {
    self.previous_selected_cell != self.current_selected_cell
  }
}

pub fn update(game: &mut Game) {
  // game.ui.debug_text = format!(
  //   "{},{} | {} {} | {}",
  //   game.ui.current_selected_cell.x,
  //   game.ui.current_selected_cell.y,
  //   game.camera_position.x,
  //   game.camera_position.y,
  //   get_fps()
  // );

  // TODO - only if !ui.mouse_is_hover_ui()
  // Mouse position
  game.tools.selection.previous_selected_cell = game.tools.selection.current_selected_cell.clone();
  game.tools.selection.current_selected_cell = {
    let (mouse_x, mouse_y) = mouse_position();
    screen_point_to_cell(
      &Point {
        x: mouse_x,
        y: mouse_y,
      },
      game,
    )
  };

  if game.tools.selection.selected_cell_has_changed() {
    if game.input.left_mouse_is_down {
      game
        .tools
        .selection
        .set_selection_box_end_to_current_cell();
    } else {
      game
        .tools
        .selection
        .start_selection_box_at_current_cell();
    };
  }
}
