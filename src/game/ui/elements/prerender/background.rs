use macroquad::color::RED;

use crate::{
  game::{
    ui::elements::{BackgroundColorKind, Elements},
    Game,
  },
  utils::get_random_color,
};

pub fn prerender(game: &mut Game, mut elements_replica: &mut Elements) {
  // Background color
  // (doesn't really matter which direction we traverse)
  for node_id in game
    .ui
    .elements
    .tree
    .get_node_ids_grouped_by_depth_bottom_up_flat()
  {
    let node = game
      .ui
      .elements
      .tree
      .find_node_by_id_mut(node_id)
      .unwrap();

    if node.data.calculated.background_color.is_none() {
      node.data.calculated.background_color = match node.data.config.background_color {
        // TODO - transparent
        BackgroundColorKind::None => Some(RED),
        BackgroundColorKind::Fixed(color) => Some(color),
        BackgroundColorKind::Randomized => Some(get_random_color()),
      };
    }
  }
}
