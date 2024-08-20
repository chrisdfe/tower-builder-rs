use uuid::Uuid;

use crate::game::timers::{
  should_run_tick_at_infrequency, Timer, TimerCallbackContext, TimerId, TimerListener,
  TimerListenerId,
};
use crate::game::world::time::constants::TICKS_ELAPSED_PER_DAY;
use crate::game::world::tower::rooms::{Room, RoomType};
use crate::game::world::tower::Occupant;
use crate::map::Coordinates;

// Daily
// TODO - only on weekdays
const INFREQUENCY: u64 = TICKS_ELAPSED_PER_DAY as u64;

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

    let offices = tower.tower.find_rooms_by_type(RoomType::Office);

    let unoccupied_connected_offices: Vec<&Room> = offices
      .into_iter()
      // unoccupied
      .filter(|office| {
        tower
          .tower
          .find_occupants_by_office_id(&office.id)
          .len()
          == 0
      })
      // connected to transportation
      .filter(|hotel_room| tower.tower.room_is_connected_to_lobby(hotel_room))
      .collect::<_>();

    let new_occupants =
      unoccupied_connected_offices
        .into_iter()
        .fold(Vec::new(), |mut acc, unoccupied_office| {
          let mut occupants: Vec<Occupant> = (0..unoccupied_office.definition().occupancy_limit)
            .into_iter()
            .map(|_| Occupant {
              id: Uuid::new_v4(),
              // TODO - unoccupied office bottom left
              current_cell: Some(Coordinates::zero()),
              office_id: Some(unoccupied_office.id.clone()),
              home_id: None,
              hotel_room_id: None,
            })
            .collect::<_>();

          acc.append(&mut occupants);
          acc
        });

    tower.tower.add_occupants(new_occupants);

    false
  }
}

impl Listener {
  pub fn new() -> Self {
    Self {
      timer_id: TimerId::Tick,
      listener_id: TimerListenerId::OfficeMoveIn,
    }
  }
}
