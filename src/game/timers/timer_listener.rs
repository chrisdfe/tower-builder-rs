use crate::game::world::{TimeSlice, TowerSlice, WalletSlice};

use super::{Timer, TimerId};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TimerListenerId {
  GlobalTick,

  CondoMoveIn,
  HotelCheckIn,
  HotelCheckOut,
  OfficeMoveIn,
  OfficeRent,
}

pub trait TimerListener {
  fn id(&self) -> &TimerListenerId;
  fn timer_id(&self) -> &TimerId;

  fn should_run_complete_cb(&self, _: &Timer, _: TimerCallbackContext) -> bool;

  fn on_timer_update(&mut self, _: &Timer, _: TimerCallbackContext) -> bool {
    false
  }

  fn on_timer_complete(&mut self, _: &Timer, _: TimerCallbackContext) -> bool {
    false
  }
}

// Reusable functionality for should_run_complete_cb
pub fn should_run_tick_at_infrequency(infrequency: u64, ctx: TimerCallbackContext) -> bool {
  ctx.time.tick % infrequency == 0
}

// TODO - maybe return value should be 'should_cancel'
// pub type TimerCallback = fn(&Timer, TimerCallbackContext) -> ();
pub struct TimerCallbackContext<'a> {
  pub time: &'a mut TimeSlice,
  pub tower: &'a mut TowerSlice,
  pub wallet: &'a mut WalletSlice,
}
