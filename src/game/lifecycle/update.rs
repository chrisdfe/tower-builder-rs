use crate::game::slices::ui::elements::ElementHandle;
use crate::game::{slices::ui::elements, Game};

use crate::game::slices::input;
use crate::game::slices::timers;
use crate::game::slices::tools;
use crate::game::slices::ui;

pub fn update(game: &mut Game) {
  // Event handlers
  timers::run_event_handlers(game);
  ui::run_event_handlers(game);

  // Tick = increment PrevAndNext
  // game.input.tick();
  // game.timers.tick();
  game.tools.tick();
  // game.ui.tick();

  // Update
  input::update(game);
  timers::update(game);
  tools::update(game);
  ui::update(game);

  // TODO - prerender first?
  // TODO - render, not prerender
  elements::prerender(game);
}
