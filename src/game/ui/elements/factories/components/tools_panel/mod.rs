use macroquad::color::RED;

use crate::{
  game::ui::elements::{
    BackgroundColorKind, ContentAlignment, Element, ElementHandle, TwoDimensional,
  },
  measurements::Axis,
  types::tree::TreeNodeInput,
};

mod room_definition_buttons;

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("tools panel"),
      handle: ElementHandle::ToolsPanel,

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
    vec![room_definition_buttons::create()],
  )
}
