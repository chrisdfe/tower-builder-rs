use crate::{
  game::ui::elements::{
    BackgroundColorKind, ContentAlignment, Element, ElementConfig, ElementInput, Resizability,
    TwoDimensional,
  },
  measurements::{Axis, Dimensions},
  types::tree::TreeNodeInput,
};

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput {
    data: create_root_node_element(),
    children: vec![super::debug_text_panel::create()],
  }
}

fn create_root_node_element() -> Element {
  let input = ElementInput {
    name: String::from("root node"),
    config: ElementConfig {
      padding: 10,
      child_gap: 10,
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Start,
        vertical: ContentAlignment::Start,
      },
      resizability: Resizability::Fixed(Dimensions::of_screen()),
      background_color: BackgroundColorKind::None,
      ..Default::default()
    },
    ..Default::default()
  };

  Element::new(input)
}
