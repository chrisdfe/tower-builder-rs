use crate::measurements::{Axis, Dimensions};

use crate::game::slices::ui::elements::element_node_vec::get_outer_sizes_for_axis;
use crate::game::slices::ui::elements::Element;
use crate::game::slices::ui::elements::Elements;
use crate::types::tree::TreeNode;

use super::super::accumulators;

/// Calculates the total dimensions of the node's children.
/// Assumes those children's outer dimensions have been calculated already
pub fn calculate(node: &mut TreeNode<Element>, elements_replica: &mut Elements) {
  let width = calculate_for_axis(node, &Axis::Horizontal, &elements_replica);

  let height = calculate_for_axis(node, &Axis::Vertical, &elements_replica);

  node.data.calculated.children_dimensions = Some(Dimensions { width, height });
}

fn calculate_for_axis(
  node: &mut TreeNode<Element>,
  calculation_axis: &Axis,
  elements_replica: &Elements,
) -> u32 {
  let children = elements_replica.tree.get_children_for_node(node);
  let is_for_primary_axis = &node.data.stack_axis == calculation_axis;

  get_outer_sizes_for_axis(&children, calculation_axis)
    .enumerate()
    .fold(0, |acc, (idx, sibling_size)| {
      if is_for_primary_axis {
        accumulators::sum_siblings_size(acc, sibling_size, node.data.child_gap, idx)
      } else {
        accumulators::max_sibling_size(acc, sibling_size)
      }
    })
}
