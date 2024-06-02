use crate::measurements::{Axis, Dimensions};

use crate::game::ui::elements::Element;
use crate::types::tree::TreeNode;

pub fn calculate(node: &mut TreeNode<Element>) {
  let width = calculate_outer_size_for_axis(node, &Axis::Horizontal);
  let height = calculate_outer_size_for_axis(node, &Axis::Vertical);

  let calculated_outer_dimensions = Dimensions { width, height };
  node.data.calculated.outer_dimensions = Some(calculated_outer_dimensions);
}

fn calculate_outer_size_for_axis(node: &TreeNode<Element>, calculation_axis: &Axis) -> u32 {
  let content_length = node
    .data
    .calculated
    .content_dimensions
    .as_ref()
    .unwrap()
    .get_length_for_axis(calculation_axis);

  content_length + (node.data.padding * 2)
}
