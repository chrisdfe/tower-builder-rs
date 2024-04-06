use crate::measurements::{Axis, Point};
use crate::types::tree::TreeNode;

use crate::game::ui::elements::Element;

pub fn calculate_for_element(node: &mut TreeNode<Element>) {
  let x = calculate_content_position_for_axis(&node.data, &Axis::Horizontal);

  let y = calculate_content_position_for_axis(&node.data, &Axis::Vertical);

  let content_position = Point { x, y };

  node.data.calculated.content_position = Some(content_position)
}

fn calculate_content_position_for_axis(element: &Element, calculation_axis: &Axis) -> f32 {
  let outer_position = element
    .calculated
    .outer_position
    .as_ref()
    .unwrap()
    .get_value_for_axis(calculation_axis);

  outer_position + element.config.padding as f32
}
