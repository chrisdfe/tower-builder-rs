use serde::{Deserialize, Serialize};

use crate::game::timers::Timers;
use crate::tower::Tower;

pub mod timer_listeners;

#[derive(Serialize, Deserialize)]
pub struct TowerSlice {
  pub tower: Tower,
}

impl TowerSlice {
  pub fn new() -> Self {
    Self {
      tower: Tower::new(),
    }
  }

  pub fn register_timers(&self, timers: &mut Timers) {
    timers.add_timer_listener(Box::new(timer_listeners::OfficeMoveIn::new()));
    timers.add_timer_listener(Box::new(timer_listeners::CondoMoveIn::new()));
    timers.add_timer_listener(Box::new(timer_listeners::OfficeRent::new()));
    timers.add_timer_listener(Box::new(timer_listeners::HotelCheckIn::new()));
    timers.add_timer_listener(Box::new(timer_listeners::HotelCheckOut::new()));
  }
}
