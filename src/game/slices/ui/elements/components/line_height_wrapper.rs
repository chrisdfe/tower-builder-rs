use crate::{
  game::{
    lifecycle::render::DEFAULT_LINE_HEIGHT,
    slices::ui::{ContentAlignment, Element, Resizability, TwoDimensional},
  },
  types::tree::TreeNodeInput,
};

pub fn create_node_input(text: String) -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Start,
        vertical: ContentAlignment::Center,
      },

      resizability: TwoDimensional {
        horizontal: Resizability::ShrinkToFit,
        vertical: Resizability::Fixed(DEFAULT_LINE_HEIGHT),
      },

      ..Default::default()
    },
    vec![TreeNodeInput(
      Element {
        text: Some(text),
        ..Default::default()
      },
      vec![],
    )],
  )
}
