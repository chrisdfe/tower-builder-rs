pub struct Slice {
  pub left_mouse_is_down: bool,
  pub left_mouse_has_just_been_pressed: bool,
  pub left_mouse_has_just_been_released: bool,
}

impl Slice {
  pub fn new() -> Self {
    Self {
      left_mouse_is_down: false,
      left_mouse_has_just_been_pressed: false,
      left_mouse_has_just_been_released: false,
    }
  }
}
