use crate::game::{ui::elements, Game};

mod input;
mod interactivity;
mod timers;
mod tools;
mod ui;

pub fn update(game: &mut Game) {
  // Event handlers
  timers::run_event_handlers(game);
  interactivity::run_event_handlers(game);

  // TODO - do I need to do a prerender here too

  interactivity::update(game);
  timers::update(game);
  tools::update(game);
  input::update(game);
  ui::update(game);

  let mut elements_replica = game.ui.elements.clone();
  elements::prerender::dimensions::prerender(game, &mut elements_replica);
  elements::prerender::positions::prerender(game, &mut elements_replica);
  elements::prerender::background::prerender(game);
}
