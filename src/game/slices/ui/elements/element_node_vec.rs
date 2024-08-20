use crate::types::measurements::Axis;

use crate::game::slices::ui::elements::Element;
use crate::types::tree::TreeNode;

pub fn get_outer_sizes_for_axis<'a>(
  elements: &'a Vec<&'a TreeNode<Element>>,
  calculation_axis: &'a Axis,
) -> impl Iterator<Item = u32> + 'a {
  elements.into_iter().map(|sibling| {
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
  elements: &Vec<&TreeNode<Element>>,
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
    .fold(0, |acc, width| acc + width)
}
