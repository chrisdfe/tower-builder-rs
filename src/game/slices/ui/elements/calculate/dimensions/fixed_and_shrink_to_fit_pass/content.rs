

use crate::types::measurements::{Axis, Dimensions};
use crate::types::tree::TreeNode;

use crate::game::slices::ui::elements::Resizability;


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
  let dimensions = &(node.data.content_renderer.measure)(&node.data);
  dimensions.get_length_for_axis(calculation_axis)
}

fn calculate_wrapper_node_content_dimensions_for_axis(
  node: &mut TreeNode<Element>,
  calculation_axis: &Axis,
) -> u32 {
  match &node
    .data
    .resizability
    .get_value_for_axis(calculation_axis)
  {
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
