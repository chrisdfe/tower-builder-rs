use macroquad::prelude::*;

use crate::game::{slices::timers::TimerLoopType, Game};

pub fn update(game: &mut Game) {
  // Process timers
  let elapsed = get_frame_time();
  for timer in game.timers.timers.values_mut() {
    timer.current_value += elapsed;

    if timer.current_value >= timer.length {
      game
        .timers
        .timers_to_complete
        .insert(timer.id.clone());

      if timer.loop_type == TimerLoopType::Once {
        game
          .timers
          .timers_to_remove
          .insert(timer.id.clone());
      } else {
        // TODO - timer.reset()
        timer.current_value = 0.;
      }
    } else {
      game
        .timers
        .timers_to_update
        .insert(timer.id.clone());
    }
  }

  // Add added timers
  for timer in game.timers.timers_to_add.clone().into_iter() {
    game.timers.add_timer(timer);
  }

  // Reset
  game.timers.timers_to_add = Vec::new();
}

pub fn post_update(game: &mut Game) {

  // noop
}
