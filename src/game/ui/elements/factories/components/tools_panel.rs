use crate::{
  game::ui::elements::{
    ContentAlignment, Element, ElementConfig, TwoDimensional, ROOM_DEFINITION_BUTTONS,
  },
  measurements::Axis,
  types::tree::TreeNodeInput,
};

pub fn create() -> TreeNodeInput<Element> {}

fn create_room_definition_buttons() -> TreeNodeInput<Element> {
  TreeNodeInput {
    data: Element {
      name: String::from("room buttons wrapper"),
      config: ElementConfig {
        padding: 10,
        child_gap: 10,
        stack_axis: Axis::Vertical,
        content_alignment: TwoDimensional {
          horizontal: ContentAlignment::Center,
          vertical: ContentAlignment::Center,
        },
        ..Default::default()
      },
      ..Default::default()
    },
    children: ROOM_DEFINITION_BUTTONS
      .clone()
      .into_iter()
      .map(|element_input| {
        let element = Element::new(element_input);
        TreeNodeInput {
          data: element,
          children: Vec::new(),
        }
      })
      .collect::<Vec<_>>(),
  }
}
