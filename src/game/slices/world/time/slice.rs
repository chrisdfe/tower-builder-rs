use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{
  game::slices::timers::{
    self, Timer, TimerCallbackContext, TimerId, TimerListener, TimerListenerId, TimerLoopType,
  },
  types::PrevAndCurrent,
};

use super::{constants::*, types::Time};

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum TimeSpeed {
  Paused,
  Normal,
  Fast,
  VeryFast,
}

lazy_static! {
  #[rustfmt::skip]
  pub static ref TIME_SPEED_MINUTES_PER_TICK_MAP: HashMap<TimeSpeed, u8> = HashMap::from([
    (TimeSpeed::Paused, 0),
    (TimeSpeed::Normal, 1),
    (TimeSpeed::Fast, 5),
    (TimeSpeed::VeryFast, 15),
  ]);
}

#[derive(Serialize, Deserialize)]
pub struct Slice {
  pub tick: u64,
  speed: PrevAndCurrent<TimeSpeed>,
  current_minute: PrevAndCurrent<u64>,
}

const DEFAULT_SPEED: TimeSpeed = TimeSpeed::Normal;

impl Slice {
  pub fn new() -> Self {
    Self {
      tick: 0,
      speed: PrevAndCurrent::new(DEFAULT_SPEED),
      current_minute: PrevAndCurrent::new(0),
    }
  }

  pub fn current_time(&self) -> Time {
    // Time::from_minutes(self.tick * MINUTES_ELAPSED_PER_TICK as u64)
    Time::from_minutes(self.current_minute.current)
  }

  pub fn current_speed(&self) -> &TimeSpeed {
    &self.speed.current
  }

  pub fn speed(&self) -> &PrevAndCurrent<TimeSpeed> {
    &self.speed
  }

  pub fn set_speed(&mut self, new_speed: TimeSpeed) {
    self.speed.set_current(new_speed.clone());
  }

  pub fn on_tick(&mut self) {
    self.tick += 1;

    let current_minute = self.current_minute.current
      + *TIME_SPEED_MINUTES_PER_TICK_MAP
        .get(&self.speed.current)
        .unwrap() as u64;

    self.current_minute.set_current(current_minute);
  }

  pub fn register_timers(&self, timers_slice: &mut timers::Slice) {
    // TODO - add this timer in the timers slice - only register the listener here instead
    timers_slice.add_timer(Timer {
      id: TimerId::Tick,
      length: TICK_INTERVAL_S,
      current_value: 0.,
      loop_type: TimerLoopType::Looping,
    });

    timers_slice.add_timer_listener(Box::new(TimeTickListener));
  }
}

pub struct TimeTickListener;

const TICK_LISTENER_ID: TimerListenerId = TimerListenerId::TimeTick;
const TICK_TIMER_ID: TimerId = TimerId::Tick;

impl TimerListener for TimeTickListener {
  fn id(&self) -> &TimerListenerId {
    &TICK_LISTENER_ID
  }

  fn timer_id(&self) -> &TimerId {
    &TICK_TIMER_ID
  }

  // true = run on_timer_complete on every tick
  fn should_run_complete_cb(&self, _: &Timer, _: TimerCallbackContext) -> bool {
    true
  }

  fn on_timer_complete(&mut self, _: &Timer, ctx: TimerCallbackContext) -> bool {
    let TimerCallbackContext { time, .. } = ctx;
    time.on_tick();

    false
  }
}
