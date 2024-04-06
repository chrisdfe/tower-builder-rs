use crate::game::{ui::elements, Game};

mod input;
mod interactivity;
mod timers;
mod tools;
mod ui;

pub fn update(game: &mut Game) {
  input::update(game);
  ui::update(game);

  // TODO - this feels pretty bad; reorganize
  let mut elements_replica = game.ui.elements.clone();
  elements::prerender::dimensions::prerender(game, &mut elements_replica);
  elements::prerender::positions::prerender(game, &mut elements_replica);
  interactivity::update(game);
  elements::prerender::background::prerender(game, &mut elements_replica);

  timers::update(game);
  tools::update(game);
}
