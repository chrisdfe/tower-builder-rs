use uuid::Uuid;

use crate::measurements::Axis;
use crate::types::tree::TreeNodeInput;

use super::super::{Element, ElementConfig, Resizability};

pub fn create_childless_stretch_to_fill_node(expand_weight: u32) -> TreeNodeInput<Element> {
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

pub fn create_stretch_to_fill_node_with_children(expand_weight: u32) -> TreeNodeInput<Element> {
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
      create_childless_stretch_to_fill_node(1),
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
          // create_childless_stretch_to_fill_node(1),
          create_childless_stretch_to_fill_node(1),
          super::create_room_definition_buttons_with_wrapper(),
          create_childless_stretch_to_fill_node(1),
          // create_childless_stretch_to_fill_node(1),
        ],
      },
      create_childless_stretch_to_fill_node(1),
    ],
  }
}

#[rustfmt::skip]
pub fn create_stretch_to_fill_node_group(root_node_id: Uuid) -> Vec<(TreeNodeInput<Element>, Option<Uuid>)> {
  vec![
    (create_childless_stretch_to_fill_node(1), Some(root_node_id)),
    // (create_room_definition_buttons_with_wrapper(), Some(root_node_id)),
    (create_stretch_to_fill_node_with_children(2), Some(root_node_id)),
    // (create_room_definition_buttons_with_wrapper(), Some(root_node_id)),
    (create_childless_stretch_to_fill_node(1), Some(root_node_id)),
  ]  
}
