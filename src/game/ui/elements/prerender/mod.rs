use macroquad::color::RED;

use crate::{
  game::{ui::elements::BackgroundColorKind, Game},
  utils::get_random_color,
};

mod accumulators;
mod background;
mod dimensions;
mod positions;

/*
 * Prerender
 *
 * Performs/caches potentially expensive calculations
 * For now just for layout nodes
 */
pub fn prerender(game: &mut Game) {
  // TODO - only create these if neccessary (build list of needing-prerender ids
  // below and check before iterating)

  let mut elements_replica = game.ui.elements.clone();

  dimensions::prerender(game, &mut elements_replica);
  positions::prerender(game, &mut elements_replica);
  background::prerender(game, &mut elements_replica);
}
