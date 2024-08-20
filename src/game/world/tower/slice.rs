use serde::{Deserialize, Serialize};

use super::timer_listeners;
use crate::game::timers;
use crate::tower::Tower;

#[derive(Serialize, Deserialize)]
pub struct Slice {
  pub tower: Tower,
}

impl Slice {
  pub fn new() -> Self {
    Self {
      tower: Tower::new(),
    }
  }

  pub fn register_timers(&self, timers: &mut timers::Slice) {
    // TODO - use const array instead
    timers.add_timer_listener(Box::new(timer_listeners::OfficeMoveIn::new()));
    timers.add_timer_listener(Box::new(timer_listeners::CondoMoveIn::new()));
    timers.add_timer_listener(Box::new(timer_listeners::OfficeRent::new()));
    timers.add_timer_listener(Box::new(timer_listeners::HotelCheckIn::new()));
    timers.add_timer_listener(Box::new(timer_listeners::HotelCheckOut::new()));
  }
}
