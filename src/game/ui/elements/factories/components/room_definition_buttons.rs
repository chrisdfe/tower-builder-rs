use crate::{
  game::ui::elements::{ContentAlignment, Element, TwoDimensional},
  measurements::Axis,
  tower::rooms::definitions::ROOM_DEFINITIONS,
  types::tree::TreeNodeInput,
};

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("room buttons wrapper"),

      padding: 10,
      child_gap: 10,
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },
      ..Default::default()
    },
    ROOM_DEFINITIONS
      .iter()
      .map(|(_definition_id, definition)| {
        TreeNodeInput(
          Element {
            name: String::from(format!("room definition button: {}", definition.id)),
            // TODO - on_click
            ..Default::default()
          },
          Vec::new(),
        )
      })
      .collect::<Vec<_>>(),
  )
}
