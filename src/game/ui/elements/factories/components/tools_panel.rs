use macroquad::color::RED;

use crate::{
  game::ui::elements::{BackgroundColorKind, ContentAlignment, Element, TwoDimensional},
  measurements::Axis,
  types::tree::TreeNodeInput,
};

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("tools panel"),
      padding: 10,
      child_gap: 10,
      background_color: BackgroundColorKind::Fixed(RED),
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },
      ..Default::default()
    },
    vec![super::room_definition_buttons::create()],
  )
}
