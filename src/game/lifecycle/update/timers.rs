use macroquad::prelude::*;
use std::collections::HashSet;

use crate::game::timers::TimerCallbackContext;
use crate::game::{timers::TimerLoopType, Game};

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

pub fn run_event_handlers(game: &mut Game) {
  update_updated_timers(game);
  complete_completed_timers(game);
  remove_removed_timers(game);
}

fn update_updated_timers(game: &mut Game) {
  // Update updated timers
  for timer_id in game.timers.timers_to_update.iter() {
    let timer = game.timers.timers.get(&timer_id).unwrap();
    game
      .timers
      .timer_listeners
      .iter_mut()
      .filter(|listener| *listener.timer_id() == *timer_id)
      .for_each(|listener| {
        let _ = listener.on_timer_update(
          timer,
          TimerCallbackContext {
            time: &mut game.world.time,
            tower: &mut game.world.tower,
            wallet: &mut game.world.wallet,
          },
        );
      });
  }

  // Reset
  game.timers.timers_to_update = HashSet::new();
}

fn complete_completed_timers(game: &mut Game) {
  for timer_id in game.timers.timers_to_complete.clone() {
    let timer = game.timers.timers.get(&timer_id).unwrap();

    game
      .timers
      .timer_listeners
      .iter_mut()
      .filter(|listener| *listener.timer_id() == timer.id)
      .for_each(|listener| {
        if listener.should_run_complete_cb(
          timer,
          TimerCallbackContext {
            time: &mut game.world.time,
            tower: &mut game.world.tower,
            wallet: &mut game.world.wallet,
          },
        ) {
          let _ = listener.on_timer_complete(
            timer,
            TimerCallbackContext {
              time: &mut game.world.time,
              tower: &mut game.world.tower,
              wallet: &mut game.world.wallet,
            },
          );
        }
      });
  }

  // Reset
  game.timers.timers_to_complete = HashSet::new();
}

fn remove_removed_timers(game: &mut Game) {
  for timer_id in game.timers.timers_to_remove.clone() {
    // Remove all listeners for timer first
    let listeners_to_remove =
      game
        .timers
        .timer_listeners
        .iter()
        .fold(Vec::new(), |mut acc, listener| {
          if *listener.timer_id() == timer_id {
            acc.push(listener.id().clone());
          }
          acc
        });

    listeners_to_remove
      .into_iter()
      .for_each(|listener_id| {
        let idx = game
          .timers
          .timer_listeners
          .iter()
          .position(|listener| *listener.id() == listener_id)
          .unwrap();
        game.timers.timer_listeners.remove(idx);
      });

    // remove timer itself
    game.timers.timers.remove(&timer_id);
  }

  // Reset
  game.timers.timers_to_remove = HashSet::new();
}
