use macroquad::color::{GREEN, YELLOW};

use crate::{
  game::slices::ui::elements::{
    BackgroundColorKind, ContentAlignment, Element, ElementHandle, Resizability, TwoDimensional,
  },
  types::measurements::{Axis, Dimensions},
  types::tree::TreeNodeInput,
};

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput(
    create_root_node_element(),
    vec![
      TreeNodeInput(
        Element {
          name: String::from("generic expand to fill wrapper"),
          stack_axis: Axis::Horizontal,
          content_alignment: TwoDimensional::same(ContentAlignment::Start),
          resizability: TwoDimensional {
            horizontal: Resizability::ExpandToFill(1),
            vertical: Resizability::ShrinkToFit,
          },
          ..Default::default()
        },
        vec![
          super::debug_text_panel::create_node_input(),
          super::spacers::expand_to_fill::create_node_input(1),
          super::time_panel::create_node_input(),
        ],
      ),
      super::spacers::expand_to_fill::create_node_input(3),
      super::tools_panel::tools_panel::create_node_input(),
    ],
  )
}
fn create_root_node_element() -> Element {
  Element {
    name: String::from("root node"),
    handle: ElementHandle::RootNode,

    padding: 10,
    child_gap: 10,
    background_color: BackgroundColorKind::None,

    stack_axis: Axis::Vertical,
    resizability: TwoDimensional {
      horizontal: Resizability::Fixed(Dimensions::of_screen()),
      vertical: Resizability::Fixed(Dimensions::of_screen()),
    },
    content_alignment: TwoDimensional {
      horizontal: ContentAlignment::Start,
      vertical: ContentAlignment::Start,
    },
    ..Default::default()
  }
}
