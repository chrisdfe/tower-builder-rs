use macroquad::text::measure_text;

use crate::types::measurements::{Axis, Dimensions};
use crate::types::tree::TreeNode;

use crate::game::slices::ui::elements::Resizability;

use crate::game::lifecycle::render::{get_text_settings, TextSettings, DEFAULT_FONT_SIZE};
use crate::game::slices::ui::elements::{Element, Elements};

pub fn calculate(node: &mut TreeNode<Element>, elements_replica: &mut Elements) {
  let width = calculate_content_dimensions_for_axis(node, &Axis::Horizontal, elements_replica);
  let height = calculate_content_dimensions_for_axis(node, &Axis::Vertical, elements_replica);

  let calculated_content_dimensions = Dimensions { width, height };
  node.data.calculated.content_dimensions = Some(calculated_content_dimensions);
}

fn calculate_content_dimensions_for_axis(
  node: &mut TreeNode<Element>,
  calculation_axis: &Axis,
  elements_replica: &mut Elements,
) -> u32 {
  if elements_replica.tree.node_is_leaf_node(node.id) {
    calculate_leaf_node_content_dimensions_for_axis(node, calculation_axis)
  } else {
    calculate_wrapper_node_content_dimensions_for_axis(node, calculation_axis)
  }
}

fn calculate_leaf_node_content_dimensions_for_axis(
  node: &mut TreeNode<Element>,
  calculation_axis: &Axis,
) -> u32 {
  // If a leaf node has neither text nor a fixed size, it collapses to 0x0
  if let Some(text) = &node.data.text {
    let TextSettings {
      font,
      font_size,
      font_scale,
      ..
    } = get_text_settings();
    let text_dimensions = measure_text(text, font, font_size, font_scale);

    match calculation_axis {
      Axis::Horizontal => text_dimensions.width as u32,
      Axis::Vertical => std::cmp::max(DEFAULT_FONT_SIZE as u32, text_dimensions.height as u32),
    }
  } else {
    use Resizability::*;
    match &node.data.resizability {
      Fixed(dimensions) => {
        dimensions.get_length_for_axis(calculation_axis) - (node.data.padding * 2)
      }
      ShrinkToFit => 0,
      ExpandToFill(_) => 0,
    }
  }
}

fn calculate_wrapper_node_content_dimensions_for_axis(
  node: &mut TreeNode<Element>,
  calculation_axis: &Axis,
) -> u32 {
  match &node.data.resizability {
    Resizability::Fixed(dimensions) => {
      dimensions.get_length_for_axis(calculation_axis) - (node.data.padding * 2)
    }
    Resizability::ShrinkToFit => {
      // Assumes children has been calculated
      node
        .data
        .calculated
        .children_dimensions
        .as_ref()
        .unwrap()
        .get_length_for_axis(calculation_axis)
    }
    Resizability::ExpandToFill(_) => 0,
  }
}
