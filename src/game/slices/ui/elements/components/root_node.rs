use crate::{
  game::slices::ui::elements::{
    BackgroundColorKind, ContentAlignment, Element, Resizability, TwoDimensional,
  },
  types::measurements::{Axis, Dimensions},
  types::tree::TreeNodeInput,
};

pub const ROOT_ELEMENT_HANDLE: &'static str = "root element";

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
      super::spacers::expand_to_fill::create_node_input(1),
      super::tools_panel::tools_panel::create_node_input(),
    ],
  )
}
fn create_root_node_element() -> Element {
  let screen_dimensions = Dimensions::of_screen();

  Element {
    name: String::from("root node"),
    handle: ROOT_ELEMENT_HANDLE,

    padding: 10,
    child_gap: 10,
    background_color: BackgroundColorKind::None,

    stack_axis: Axis::Vertical,
    resizability: TwoDimensional {
      horizontal: Resizability::Fixed(screen_dimensions.width),
      vertical: Resizability::Fixed(screen_dimensions.height),
    },
    content_alignment: TwoDimensional {
      horizontal: ContentAlignment::Start,
      vertical: ContentAlignment::Start,
    },
    ..Default::default()
  }
}
