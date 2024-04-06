use uuid::Uuid;

use crate::game::ui::elements::ContentAlignment;
use crate::measurements::{Axis, Dimensions};
use crate::types::tree::TreeNodeInput;

use super::constants::ROOM_DEFINITION_BUTTONS;
use super::{
  BackgroundColorKind, Element, ElementConfig, ElementInput, Resizability, TwoDimensional,
};

pub fn create_room_definition_buttons_with_wrapper() -> TreeNodeInput<Element> {
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

pub fn create_debug_stretch_to_fill_node() -> TreeNodeInput<Element> {
  TreeNodeInput {
    data: Element {
      name: String::from("debug stretch_to_fill node"),
      config: ElementConfig {
        padding: 10,
        resizability: Resizability::ExpandToFill(1),
        ..Default::default()
      },
      ..Default::default()
    },
    children: Vec::new(),
  }
}

#[rustfmt::skip]
pub fn create_debug_stretch_to_fill_node_group(root_node_id: Uuid) -> Vec<(TreeNodeInput<Element>, Option<Uuid>)> {
  vec![
    (create_debug_stretch_to_fill_node(), Some(root_node_id)),
    (create_room_definition_buttons_with_wrapper(), Some(root_node_id)),
    (create_debug_stretch_to_fill_node(), Some(root_node_id)),
    (create_room_definition_buttons_with_wrapper(), Some(root_node_id)),
    (create_debug_stretch_to_fill_node(), Some(root_node_id)),
  ]  
}

pub fn create_empty_leaf_node() -> ElementInput {
  ElementInput {
    name: String::from("empty leaf node"),
    config: ElementConfig {
      padding: 10,
      child_gap: 10,
      stack_axis: Axis::Horizontal,
      resizability: Resizability::Fixed(Dimensions {
        width: 300,
        height: 200,
      }),
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Start,
      },
      ..Default::default()
    },
    ..Default::default()
  }
}

pub fn create_root_node_element() -> Element {
  let input = ElementInput {
    name: String::from("root node"),
    config: ElementConfig {
      padding: 30,
      child_gap: 10,
      stack_axis: Axis::Horizontal,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Start,
      },
      resizability: Resizability::Fixed(Dimensions::of_screen()),
      background_color: BackgroundColorKind::Randomized,
      ..Default::default()
    },
    ..Default::default()
  };

  Element::new(input)
}
