use uuid::Uuid;

use crate::game::slices::ui::elements::Elements;
use crate::game::Game;

mod content;
mod outer;

/// First pass - calculate fixed + shrink_to_fit node dimension from leaf nodes up,
/// setting expand_to_fill nodes to 0
pub fn prerender(game: &mut Game, elements_replica: &mut Elements) {
  let sibling_id_groups = get_sibling_id_groups_bottom_up(&game.ui.elements);

  for sibling_id_group in sibling_id_groups {
    let (_, sibling_ids) = sibling_id_group;

    // Calculate dimensions from leaf nodes -> up
    // TODO - calculate as a group instead of on a per-node basis
    //        calculate the fixed position ones first, then the "stretch to fill" ones, etc
    for node_id in sibling_ids {
      let node = game
        .ui
        .elements
        .tree
        .find_node_by_id_mut(node_id)
        .unwrap();

      super::children::calculate(node, elements_replica);
      content::calculate(node, elements_replica);
      outer::calculate(node);

      // Update layout node copy with updated element
      elements_replica.tree.replace_node(node);
    }
  }
}

fn get_sibling_id_groups_bottom_up(
  elements_to_calculate: &Elements,
) -> Vec<(Option<Uuid>, Vec<Uuid>)> {
  elements_to_calculate
    .tree
    .get_nodes_grouped_by_siblings_bottom_up()
    .into_iter()
    .collect::<Vec<_>>()
}
