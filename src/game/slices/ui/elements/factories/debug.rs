use uuid::Uuid;

use crate::game::slices::ui::elements::{ContentAlignment, TwoDimensional};
use crate::measurements::Axis;
use crate::measurements::Dimensions;
use crate::types::tree::TreeNodeInput;

use super::super::{Element, Resizability};

pub fn create_childless_stretch_to_fill_node(expand_weight: u32) -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("debug stretch_to_fill node"),
      padding: 10,
      resizability: Resizability::ExpandToFill(expand_weight),
      ..Default::default()
    },
    Vec::new(),
  )
}

pub fn create_stretch_to_fill_node_with_children(expand_weight: u32) -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("debug stretch_to_fill node with children"),
      padding: 10,
      resizability: Resizability::ExpandToFill(expand_weight),
      stack_axis: Axis::Vertical,
      ..Default::default()
    },
    vec![
      create_childless_stretch_to_fill_node(1),
      TreeNodeInput(
        Element {
          name: String::from("debug stretch_to_fill node with children"),
          padding: 10,
          resizability: Resizability::ExpandToFill(expand_weight),
          stack_axis: Axis::Horizontal,
          ..Default::default()
        },
        vec![
          // create_childless_stretch_to_fill_node(1),
          create_childless_stretch_to_fill_node(1),
          TreeNodeInput(
            Element {
              name: String::from("expanding node"),
              text: String::from("expanding node."),
              padding: 10,
              resizability: Resizability::ExpandToFill(expand_weight),
              stack_axis: Axis::Horizontal,
              ..Default::default()
            },
            Vec::new(),
          ),
          create_childless_stretch_to_fill_node(1),
          // create_childless_stretch_to_fill_node(1),
        ],
      ),
      create_childless_stretch_to_fill_node(1),
    ],
  )
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

pub fn create_empty_leaf_node() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("empty leaf node"),
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
    Vec::new(),
  )
}
