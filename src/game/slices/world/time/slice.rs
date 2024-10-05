use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::game::slices::timers::{
  self, Timer, TimerCallbackContext, TimerId, TimerListener, TimerListenerId, TimerLoopType,
};

use super::{constants::*, types::Time};

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum TimeSpeed {
  Paused = 0,
  Normal = 1,
  Fast = 2,
  VeryFast = 3,
}

lazy_static! {
  #[rustfmt::skip]
  pub static ref TIME_SPEED_INTERVAL_MAP: HashMap<TimeSpeed, f32> = HashMap::from([
    (TimeSpeed::Paused, 0.),
    (TimeSpeed::Normal, 1.),
    (TimeSpeed::Fast, 3.),
    (TimeSpeed::VeryFast, 5.),
  ]);
}

#[derive(Serialize, Deserialize)]
pub struct Slice {
  pub tick: u64,
  speed: TimeSpeed,
  current_interval: f32,
}

const DEFAULT_SPEED: TimeSpeed = TimeSpeed::Normal;

impl Slice {
  pub fn new() -> Self {
    Self {
      tick: 0,
      speed: DEFAULT_SPEED,
      current_interval: *TIME_SPEED_INTERVAL_MAP
        .get(&TimeSpeed::Normal)
        .unwrap(),
    }
  }

  pub fn current_time(&self) -> Time {
    Time::from_minutes(self.tick * MINUTES_ELAPSED_PER_TICK as u64)
  }

  pub fn speed(&self) -> &TimeSpeed {
    &self.speed
  }

  pub fn set_speed(&mut self, new_speed: TimeSpeed) {
    self.speed = new_speed.clone();
    self.current_interval = *TIME_SPEED_INTERVAL_MAP.get(&new_speed).unwrap();
  }

  pub fn register_timers(&self, timers_slice: &mut timers::Slice) {
    timers_slice.add_timer(Timer {
      id: TimerId::Tick,
      length: TICK_INTERVAL_S,
      current_value: 0.,
      loop_type: TimerLoopType::Looping,
    });

    timers_slice.add_timer_listener(Box::new(TimeTickListener::new()));
  }
}

// TODO - I probably don't need this complexity here. just a 'tick' slice variable that gets incremented
pub struct TimeTickListener {
  listener_id: TimerListenerId,
  timer_id: TimerId,
}

impl TimerListener for TimeTickListener {
  fn id(&self) -> &TimerListenerId {
    &self.listener_id
  }

  fn timer_id(&self) -> &TimerId {
    &self.timer_id
  }

  fn should_run_complete_cb(&self, _: &Timer, _: TimerCallbackContext) -> bool {
    true
  }

  fn on_timer_complete(&mut self, _: &Timer, ctx: TimerCallbackContext) -> bool {
    let TimerCallbackContext { time, .. } = ctx;
    time.tick += 1;

    false
  }
}

impl TimeTickListener {
  pub fn new() -> Self {
    Self {
      timer_id: TimerId::Tick,
      listener_id: TimerListenerId::GlobalTick,
    }
  }
}
