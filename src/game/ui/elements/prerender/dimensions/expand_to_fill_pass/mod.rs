use uuid::Uuid;

use crate::game::ui::elements::element_node_vec;
use crate::game::ui::elements::prerender::accumulators;
use crate::game::ui::elements::{Element, Elements};
use crate::game::Game;
use crate::measurements::Axis;
use crate::types::tree::TreeNode;

/// Second pass - traverses back down from the root & if there is a difference between
/// parent's content_size and children_size then use that to set expand_to_fill node values
pub fn prerender(game: &mut Game, mut elements_replica: &mut Elements) {
  let sibling_id_groups = game
    .ui
    .elements
    .get_nodes_grouped_by_siblings_top_down();

  for (maybe_parent_id, sibling_id_group) in sibling_id_groups {
    // The root node won't have any siblings
    if let Some(parent_id) = maybe_parent_id {
      calculate_sibling_group(
        sibling_id_group,
        parent_id,
        &mut game.ui.elements,
        &mut elements_replica,
      );
    }
  }
}

fn calculate_sibling_group(
  sibling_id_group: Vec<Uuid>,
  parent_id: Uuid,
  elements_to_calculate: &mut Elements,
  elements_replica: &mut Elements,
) {
  for axis in [Axis::Horizontal, Axis::Vertical] {
    calculate_sibling_group_for_axis(
      &sibling_id_group,
      parent_id,
      elements_to_calculate,
      elements_replica,
      &axis,
    );
  }

  // recalculate parent children dimensions
  {
    let parent_node_mut = elements_to_calculate
      .layers
      .find_node_by_id_mut(parent_id)
      .unwrap();

    super::children::calculate(parent_node_mut, elements_replica);

    elements_replica.replace_node(parent_node_mut);
  };
}

fn calculate_sibling_group_for_axis(
  sibling_id_group: &Vec<Uuid>,
  parent_id: Uuid,
  elements_to_calculate: &mut Elements,
  elements_replica: &mut Elements,
  calculation_axis: &Axis,
) {
  let parent_node = elements_replica
    .tree
    .find_node_by_id(parent_id)
    .unwrap();

  let (expand_to_fill_siblings, non_expand_to_fill_siblings) =
    group_siblings_by_expand_to_fill(&sibling_id_group, &elements_replica);

  let total_expand_to_fill_sibling_weights = expand_to_fill_siblings
    .iter()
    .map(|sibling| {
      sibling
        .data
        .config
        .resizability
        .extract_expand_to_fill_weight()
    })
    .fold(0, accumulators::sum);

  let primary_axis = parent_node.data.config.stack_axis.clone();
  let is_on_primary_axis = primary_axis == *calculation_axis;

  let (_, parent_content_size, parent_children_size) = parent_node
    .data
    .calculated
    .get_sizes_for_axis(&calculation_axis);

  // checked_sub to avoid overflow errors if this ends up being less than 0
  // (the layout will look weird but the application won't crash)
  let size_diff = parent_content_size.checked_sub(parent_children_size);

  // TODO - print a warning at some point, because this will cause some confusing layout situations
  if size_diff.is_none() {
    return;
  }

  let non_expand_to_fill_siblings_size = element_node_vec::get_total_elements_size_for_axis(
    &non_expand_to_fill_siblings,
    calculation_axis,
  );

  let calculated_sizes = expand_to_fill_siblings
    .into_iter()
    .map(|sibling| {
      let child_gap = parent_node.data.config.child_gap;
      let weight = sibling
        .data
        .config
        .resizability
        .extract_expand_to_fill_weight();

      // Do the actual calculation here
      let outer_size = if is_on_primary_axis {
        let total_available_expand_to_fill_space = (parent_content_size
          - non_expand_to_fill_siblings_size)
          - (child_gap * (sibling_id_group.len() - 1) as u32);

        (total_available_expand_to_fill_space / total_expand_to_fill_sibling_weights) * weight
      } else {
        parent_content_size
      };

      let content_size = outer_size - (sibling.data.config.padding * 2);

      (sibling.id, outer_size, content_size)
    })
    .collect::<Vec<_>>();

  // Save calculations to element
  for (sibling_id, outer_size, content_size) in calculated_sizes {
    let sibling = elements_to_calculate
      .tree
      .find_node_by_id_mut(sibling_id)
      .unwrap();

    sibling
      .data
      .calculated
      .set_outer_dimensions_for_axis(outer_size, &calculation_axis);

    sibling
      .data
      .calculated
      .set_content_dimensions_for_axis(content_size, &calculation_axis);

    elements_replica.tree.replace_node(sibling);
  }
}

// (<expand to fill siblings>, <non-expand to fill siblings>)
fn group_siblings_by_expand_to_fill<'a>(
  sibling_id_group: &Vec<Uuid>,
  elements: &'a Elements,
) -> (Vec<&'a TreeNode<Element>>, Vec<&'a TreeNode<Element>>) {
  elements
    .tree
    .find_nodes_by_ids(sibling_id_group)
    .into_iter()
    .partition(|sibling| {
      sibling
        .data
        .config
        .resizability
        .is_expand_to_fill()
    })
}
