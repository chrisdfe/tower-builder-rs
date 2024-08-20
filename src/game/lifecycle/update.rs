use crate::game::{slices::ui::elements, Game};

use crate::game::slices::input;
use crate::game::slices::timers;
use crate::game::slices::tools;
use crate::game::slices::ui;

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

  // TODO - prerender first?
  // TODO - render, not prerender
  elements::prerender(game);
}
