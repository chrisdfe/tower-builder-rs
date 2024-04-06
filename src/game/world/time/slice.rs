use serde::{Deserialize, Serialize};

use crate::game::timers::{
  Timer, TimerCallbackContext, TimerId, TimerListener, TimerListenerId, TimerLoopType, Timers,
};

use super::{constants::*, types::Time};

#[derive(Serialize, Deserialize)]
pub struct TimeSlice {
  pub tick: u64,
}

impl TimeSlice {
  pub fn new() -> Self {
    Self { tick: 0 }
  }

  pub fn current_time(&self) -> Time {
    Time::from_minutes(self.tick * MINUTES_ELAPSED_PER_TICK as u64)
  }

  pub fn register_timers(&self, timers_slice: &mut Timers) {
    timers_slice.add_timer(Timer {
      id: TimerId::Tick,
      length: TICK_INTERVAL_S,
      current_value: 0.,
      loop_type: TimerLoopType::Looping,
    });

    timers_slice.add_timer_listener(Box::new(GlobalTickListener::new()));
  }
}

pub struct GlobalTickListener {
  listener_id: TimerListenerId,
  timer_id: TimerId,
}

impl TimerListener for GlobalTickListener {
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

impl GlobalTickListener {
  pub fn new() -> Self {
    GlobalTickListener {
      timer_id: TimerId::Tick,
      listener_id: TimerListenerId::GlobalTick,
    }
  }
}
