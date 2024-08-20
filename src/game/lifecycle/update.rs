use crate::game::{ui::elements, Game};

use crate::game::input;
use crate::game::timers;
use crate::game::tools;
use crate::game::ui;

pub fn update(game: &mut Game) {
  // Event handlers
  timers::run_event_handlers(game);
  ui::run_event_handlers(game);

  // TODO - do I need to do a prerender here too

  // Update
  input::update(game);
  timers::update(game);
  tools::update(game);
  ui::update(game);

  // TODO - move elewhere
  let mut elements_replica = game.ui.elements.clone();
  elements::prerender::dimensions::prerender(game, &mut elements_replica);
  elements::prerender::positions::prerender(game, &mut elements_replica);
  elements::prerender::background::prerender(game, &mut elements_replica);
}
