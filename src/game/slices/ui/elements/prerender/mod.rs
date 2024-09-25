use macroquad::color::RED;

use crate::{
  game::{slices::ui::elements::BackgroundColorKind, Game},
  utils::get_random_color,
};

/// A collection of generic functions to use in reduce/fold algorithms
mod accumulators;

pub mod background;
pub mod dimensions;
pub mod positions;

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
