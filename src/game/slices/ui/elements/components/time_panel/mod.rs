use macroquad::color::BLACK;

use crate::{
  game::slices::ui::{
    elements::{Element, Resizability},
    BackgroundColorKind, ContentAlignment, TwoDimensional,
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

mod analog_clock;
mod date_text;
mod digital_clock;
mod speed_buttons_wrapper;

pub const TIME_PANEL_HANDLE: &'static str = "time panel";
pub const CLOCKS_WRAPPER_HANDLE: &'static str = "clocks wrapper";

pub fn create_node_input() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from(TIME_PANEL_HANDLE),
      handle: TIME_PANEL_HANDLE,

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
          name: String::from(CLOCKS_WRAPPER_HANDLE),
          handle: CLOCKS_WRAPPER_HANDLE,

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
        vec![
          date_text::create_node_input(),
          speed_buttons_wrapper::create_node_input(),
        ],
      ),
    ],
  )
}
