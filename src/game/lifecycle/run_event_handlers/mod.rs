use crate::game::Game;

mod timers;
mod ui;

pub fn run_event_handlers(game: &mut Game) {
  ui::run_event_handlers(game);
  timers::run_event_handlers(game);
}
