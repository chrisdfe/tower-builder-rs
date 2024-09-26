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
      super::debug_text_panel::create(),
      super::tools_panel::tools_panel::create(),
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
    resizability: Resizability::Fixed(Dimensions::of_screen()),
    content_alignment: TwoDimensional {
      horizontal: ContentAlignment::Start,
      vertical: ContentAlignment::Start,
    },
    ..Default::default()
  }
}
