use crate::game::slices::ui::elements::prerender::accumulators;
use crate::game::slices::ui::elements::{Element, Elements};
use crate::types::measurements::{Axis, Point};
use crate::types::tree::TreeNode;

use super::calculations;

pub fn calculate_for_element(node: &mut TreeNode<Element>, elements_replica: &Elements) {
  let x = calculate_outer_position_for_axis(node, elements_replica, &Axis::Horizontal);
  let y = calculate_outer_position_for_axis(node, elements_replica, &Axis::Vertical);

  let outer_position = Point { x, y };

  node.data.calculated.outer_position = Some(outer_position);
}

// TODO - get parent_node, siblings, etc in base calculate_for_element fn to avoid querying twice
fn calculate_outer_position_for_axis(
  mut node: &mut TreeNode<Element>,
  elements_replica: &Elements,
  calculation_axis: &Axis,
) -> f32 {
  // if None, assume this is the root node
  if node.parent_id.is_none() {
    return 0.;
  }

  let parent_node = elements_replica
    .tree
    .find_node_by_id(node.parent_id.unwrap())
    .unwrap();

  // Start with parent content position
  // This assumes that the parent_node's content position has already been calculated
  let mut result = parent_node
    .data
    .calculated
    .content_position
    .as_ref()
    .unwrap()
    .get_value_for_axis(calculation_axis);

  // Take parent's content alignment into account
  result +=
    calculate_parent_content_alignment_offset_for_axis(parent_node, calculation_axis) as f32;

  // Finally, calculate current element's position along relative to siblings
  result += if &parent_node.data.stack_axis == calculation_axis {
    calculate_element_position_on_primary_axis(
      &mut node,
      &parent_node,
      calculation_axis,
      elements_replica,
    ) as f32
  } else {
    calculate_element_position_on_off_axis(
      &mut node,
      &parent_node,
      calculation_axis,
      elements_replica,
    ) as f32
  };

  result
}

fn calculate_parent_content_alignment_offset_for_axis(
  parent_node: &TreeNode<Element>,
  calculation_axis: &Axis,
) -> i32 {
  let alignment = parent_node
    .data
    .content_alignment
    .get_value_for_axis(calculation_axis);

  // Center content within parent
  let size = parent_node
    .data
    .calculated
    .children_dimensions
    .as_ref()
    .unwrap()
    .get_length_for_axis(calculation_axis);

  let wrapper_size = parent_node
    .data
    .calculated
    .content_dimensions
    .as_ref()
    .unwrap()
    .get_length_for_axis(calculation_axis);

  calculations::align_within_wrapper(size, wrapper_size, alignment)
}

// On primary axis, calculate width of siblings before current
fn calculate_element_position_on_primary_axis(
  node: &mut TreeNode<Element>,
  parent_node: &TreeNode<Element>,
  calculation_axis: &Axis,
  elements_replica: &Elements,
) -> u32 {
  let siblings_up_to_current = {
    let siblings = elements_replica.tree.get_siblings_for_node(&node);
    let current_sibling_position = siblings
      .iter()
      .position(|sibling| sibling.id == node.id)
      .unwrap();
    siblings
      .into_iter()
      .take(current_sibling_position)
      .collect::<Vec<_>>()
  };

  siblings_up_to_current
    .into_iter()
    .fold(0, |acc, sibling_node| {
      let sibling_size = get_outer_size_for_axis(&sibling_node.data, calculation_axis);
      let margin = &parent_node.data.child_gap;

      acc + sibling_size + margin
    })
}

// On off-axis, align node relative to siblings
fn calculate_element_position_on_off_axis(
  node: &mut TreeNode<Element>,
  parent_node: &TreeNode<Element>,
  calculation_axis: &Axis,
  elements: &Elements,
) -> i32 {
  let size = get_outer_size_for_axis(&node.data, calculation_axis);

  let wrapper_size = elements
    .tree
    .get_siblings_for_node(node)
    .into_iter()
    .map(|sibling| get_outer_size_for_axis(&sibling.data, calculation_axis))
    .fold(0, accumulators::max_sibling_size);

  let alignment = parent_node
    .data
    .content_alignment
    .get_value_for_axis(calculation_axis);

  calculations::align_within_wrapper(size, wrapper_size, alignment)
}

// utility fn for oft-unwrapped outer_dimensions
fn get_outer_size_for_axis(element: &Element, calculation_axis: &Axis) -> u32 {
  element
    .calculated
    .outer_dimensions
    .as_ref()
    .unwrap()
    .get_length_for_axis(calculation_axis)
}
