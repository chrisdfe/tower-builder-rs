use crate::game::slices::timers::{
  should_run_tick_at_infrequency, Timer, TimerCallbackContext, TimerId, TimerListener,
  TimerListenerId,
};
use crate::game::slices::world::time::constants::TICKS_ELAPSED_PER_MONTH;

use crate::game::slices::world::tower::rooms::{Room, RoomType};

// Monthly
const INFREQUENCY: u64 = TICKS_ELAPSED_PER_MONTH as u64;

pub struct Listener {
  listener_id: TimerListenerId,
  timer_id: TimerId,
}

impl TimerListener for Listener {
  fn id(&self) -> &TimerListenerId {
    &self.listener_id
  }

  fn timer_id(&self) -> &TimerId {
    &self.timer_id
  }

  fn should_run_complete_cb(&self, _: &Timer, ctx: TimerCallbackContext) -> bool {
    should_run_tick_at_infrequency(INFREQUENCY, ctx)
  }

  fn on_timer_complete(&mut self, _: &Timer, ctx: TimerCallbackContext) -> bool {
    let TimerCallbackContext { tower, .. } = ctx;

    let occupied_offices: Vec<&Room> = tower
      .tower
      .find_rooms_by_type(RoomType::Office)
      .into_iter()
      .filter(|office| {
        tower
          .tower
          .find_occupants_by_office_id(&office.id)
          .len()
          > 0
      })
      .collect::<_>();

    let rent_amount = occupied_offices
      .into_iter()
      .fold(0, |acc, office| acc + office.definition().income);

    ctx.wallet.add_funds(rent_amount);

    false
  }
}

impl Listener {
  pub fn new() -> Self {
    Self {
      timer_id: TimerId::Tick,
      listener_id: TimerListenerId::OfficeRent,
    }
  }
}
