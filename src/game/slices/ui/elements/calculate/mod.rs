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
 * Calculate
 *
 * Calculates UI Element dimensions, position, and background color
 *
 */
pub fn calculate(game: &mut Game) {
  // TODO - cache! This gets run every tick and could get expensive
  // TODO - only create these if neccessary (build list of needs_precalculation ids
  // below and check before iterating)
  let mut elements_replica = game.ui.elements.clone();

  dimensions::calculate(game, &mut elements_replica);
  positions::calculate(game, &mut elements_replica);
  background::calculate(game, &mut elements_replica);
}
