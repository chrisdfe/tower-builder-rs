use crate::game::ui::elements::Elements;
use crate::game::Game;

mod children;

mod expand_to_fill_pass;

mod fixed_and_shrink_to_fit_pass;

pub fn prerender(game: &mut Game, elements_replica: &mut Elements) {
  fixed_and_shrink_to_fit_pass::prerender(game, elements_replica);
  expand_to_fill_pass::prerender(game, elements_replica);
}
