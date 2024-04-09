use crate::measurements::Axis;

use crate::game::ui::elements::Element;
use crate::types::tree::TreeNode;

pub fn get_outer_sizes_for_axis<'a>(
  elements: &'a [&'a TreeNode<Element>],
  calculation_axis: &'a Axis,
) -> impl Iterator<Item = u32> + 'a {
  elements.iter().map(|sibling| {
    sibling
      .data
      .calculated
      .outer_dimensions
      .as_ref()
      .unwrap()
      .get_length_for_axis(calculation_axis)
  })
}

pub fn get_total_elements_size_for_axis(
  elements: &[&TreeNode<Element>],
  calculation_axis: &Axis,
) -> u32 {
  elements
    .iter()
    .map(|element| {
      element
        .data
        .calculated
        .get_outer_size_for_axis(calculation_axis)
    })
    .sum()
}
