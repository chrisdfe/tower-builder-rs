use uuid::Uuid;

use crate::game::slices::ui::elements::{Element, Elements};
use crate::game::Game;
use crate::types::tree::TreeNode;

mod calculations;
mod content;
mod outer;

pub fn prerender(game: &mut Game, mut elements_replica: &mut Elements) {
  // Calculate root node -> down
  let sibling_id_groups = game
    .ui
    .elements
    .tree
    .get_nodes_grouped_by_siblings_top_down()
    .into_iter()
    .map(|(_, node_ids)| {
      game
        .ui
        .elements
        .tree
        .find_nodes_by_ids(&node_ids)
        .into_iter()
        .filter(|element_id| {
          element_id
            .data
            .calculated
            .outer_position
            .is_none()
        })
        .map(|node| node.id)
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  calculate_sibling_group(
    sibling_id_groups,
    &mut game.ui.elements,
    &mut elements_replica,
  );
}

pub fn calculate_sibling_group(
  sibling_id_groups: Vec<Vec<Uuid>>,
  elements_to_calculate: &mut Elements,
  elements_replica: &mut Elements,
) {
  // TODO - calculate as a group instead of on a per-node basis
  //        calculate the fixed position ones first, then the "stretch to fill" ones, etc
  for sibling_id_group in sibling_id_groups {
    let nodes = elements_to_calculate
      .tree
      .find_nodes_by_ids_mut(&sibling_id_group);

    // TODO - calculate as a group instead of on a per-node basis
    //        calculate the fixed position ones first, then the "stretch to fill" ones, etc
    for node in nodes {
      outer::calculate_for_element(node, elements_replica);
      content::calculate_for_element(node);

      // Update layout node copy with updated element
      elements_replica.tree.replace_node(node);
    }
    // calculate_for_sibling_id_group(sibling_id_group, elements_to_calculate, elements_replica);
  }
}
