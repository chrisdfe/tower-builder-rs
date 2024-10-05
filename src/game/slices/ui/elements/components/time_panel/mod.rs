use macroquad::color::BLACK;

use crate::{
  game::slices::ui::{
    elements::{Element, Resizability},
    BackgroundColorKind, ContentAlignment, ElementUpdateAction, ElementUpdateCtx, TwoDimensional,
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

mod analog_clock;
mod date_text;
mod digital_clock;

pub fn create_node_input() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("Time panel"),

      padding: 10,
      child_gap: 10,

      background_color: BackgroundColorKind::Fixed(BLACK),
      stack_axis: Axis::Horizontal,
      resizability: TwoDimensional::same(Resizability::ShrinkToFit),
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Start,
        vertical: ContentAlignment::Center,
      },

      ..Default::default()
    },
    vec![
      TreeNodeInput(
        Element {
          name: String::from("clocks wrapper"),
          stack_axis: Axis::Vertical,
          child_gap: 10,
          content_alignment: TwoDimensional::same(ContentAlignment::Center),
          ..Default::default()
        },
        vec![
          analog_clock::create_node_input(),
          digital_clock::create_node_input(),
        ],
      ),
      TreeNodeInput(
        Element {
          name: String::from("other stuff wrapper"),
          stack_axis: Axis::Vertical,
          child_gap: 5,
          content_alignment: TwoDimensional::same(ContentAlignment::Start),
          ..Default::default()
        },
        vec![date_text::create_node_input()],
      ),
    ],
  )
}
