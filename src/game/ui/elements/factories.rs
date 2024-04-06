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

pub fn create_childless_debug_stretch_to_fill_node(expand_weight: u32) -> TreeNodeInput<Element> {
  TreeNodeInput {
    data: Element {
      name: String::from("debug stretch_to_fill node"),
      config: ElementConfig {
        padding: 10,
        resizability: Resizability::ExpandToFill(expand_weight),
        ..Default::default()
      },
      ..Default::default()
    },
    children: Vec::new(),
  }
}

pub fn create_debug_stretch_to_fill_node_with_children(
  expand_weight: u32,
) -> TreeNodeInput<Element> {
  TreeNodeInput {
    data: Element {
      name: String::from("debug stretch_to_fill node with children"),
      config: ElementConfig {
        padding: 10,
        resizability: Resizability::ExpandToFill(expand_weight),
        stack_axis: Axis::Vertical,
        ..Default::default()
      },
      ..Default::default()
    },
    children: vec![
      create_childless_debug_stretch_to_fill_node(1),
      TreeNodeInput {
        data: Element {
          name: String::from("debug stretch_to_fill node with children"),
          config: ElementConfig {
            padding: 10,
            resizability: Resizability::ExpandToFill(expand_weight),
            stack_axis: Axis::Horizontal,
            ..Default::default()
          },
          ..Default::default()
        },
        children: vec![
          // create_childless_debug_stretch_to_fill_node(1),
          create_childless_debug_stretch_to_fill_node(1),
          create_room_definition_buttons_with_wrapper(),
          create_childless_debug_stretch_to_fill_node(1),
          // create_childless_debug_stretch_to_fill_node(1),
        ],
      },
      create_childless_debug_stretch_to_fill_node(1),
    ],
  }
}

#[rustfmt::skip]
pub fn create_debug_stretch_to_fill_node_group(root_node_id: Uuid) -> Vec<(TreeNodeInput<Element>, Option<Uuid>)> {
  vec![
    (create_childless_debug_stretch_to_fill_node(1), Some(root_node_id)),
    // (create_room_definition_buttons_with_wrapper(), Some(root_node_id)),
    (create_debug_stretch_to_fill_node_with_children(2), Some(root_node_id)),
    // (create_room_definition_buttons_with_wrapper(), Some(root_node_id)),
    (create_childless_debug_stretch_to_fill_node(1), Some(root_node_id)),
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
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },
      resizability: Resizability::Fixed(Dimensions::of_screen()),
      background_color: BackgroundColorKind::Randomized,
      ..Default::default()
    },
    ..Default::default()
  };

  Element::new(input)
}
