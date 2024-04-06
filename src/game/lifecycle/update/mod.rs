use crate::game::Game;

mod input;
mod timers;
mod tools;
mod ui;

pub fn update(game: &mut Game) {
  input::update(game);
  ui::update(game);
  timers::update(game);
  tools::update(game);
}
