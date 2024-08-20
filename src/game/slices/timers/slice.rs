use std::collections::{HashMap, HashSet};

use super::{Timer, TimerId, TimerListener};

pub struct Slice {
  pub timers: HashMap<TimerId, Timer>,
  pub timer_listeners: Vec<Box<dyn TimerListener>>,

  pub timers_to_add: Vec<Timer>,
  // pub cancelled_timer_ids: HashSet<TimerId>,
  pub timers_to_update: HashSet<TimerId>,
  pub timers_to_complete: HashSet<TimerId>,
  pub timers_to_remove: HashSet<TimerId>,
}

impl Default for Slice {
  fn default() -> Self {
    Self {
      timers: HashMap::new(),
      timer_listeners: Vec::new(),

      timers_to_add: Vec::new(),
      // cancelled_timer_ids: HashSet::new(),
      timers_to_update: HashSet::new(),
      timers_to_complete: HashSet::new(),
      timers_to_remove: HashSet::new(),
    }
  }
}

impl Slice {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn add_timer(&mut self, new_timer: Timer) {
    // Cancel existing timer first
    if let Some(existing_timer) = self.timers.get(&new_timer.id) {
      self.cancel_timer(existing_timer.id.clone());
    }

    self
      .timers
      .insert(new_timer.id.clone(), new_timer);
  }

  pub fn cancel_timer(&mut self, timer_id: TimerId) {
    self.timers.remove(&timer_id);
    self.cancel_timer_listeners(timer_id);
  }

  pub fn add_timer_listener(&mut self, listener: Box<dyn TimerListener>) {
    self.timer_listeners.push(listener);
  }

  pub fn cancel_timer_listeners(&mut self, timer_id: TimerId) {
    let listeners_to_remove = self
      .timer_listeners
      .iter()
      .fold(Vec::new(), |mut acc, listener| {
        if *listener.timer_id() == timer_id {
          acc.push(listener.id().clone());
        }
        acc
      });

    for listener_id in listeners_to_remove {
      let idx = self
        .timer_listeners
        .iter()
        .position(|listener| *listener.id() == listener_id)
        .unwrap();

      self.timer_listeners.remove(idx);
    }
  }
}
