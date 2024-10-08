use macroquad::color::Color;

use crate::{
  game::slices::ui::{BackgroundColorKind, Element},
  types::measurements::Axis,
};

const PANEL_BG_COLOR: Color = Color::new(0., 0., 0., 0.6);

pub fn create_base_element() -> Element {
  Element {
    padding: 10,
    child_gap: 10,

    background_color: BackgroundColorKind::Fixed(PANEL_BG_COLOR),
    stack_axis: Axis::Horizontal,
    ..Default::default()
  }
}
