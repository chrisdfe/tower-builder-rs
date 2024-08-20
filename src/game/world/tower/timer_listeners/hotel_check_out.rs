use uuid::Uuid;

use crate::game::timers::{Timer, TimerCallbackContext, TimerId, TimerListener, TimerListenerId};

use crate::game::world::tower::rooms::{Room, RoomType};

// 8am
const HOTEL_CHECK_OUT_TIME: u32 = 8;

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
    let TimerCallbackContext { time, .. } = ctx;
    let current_time = time.current_time();

    current_time.minute == 0 && current_time.hour == HOTEL_CHECK_OUT_TIME
  }

  fn on_timer_complete(&mut self, _: &Timer, ctx: TimerCallbackContext) -> bool {
    let TimerCallbackContext {
      time,
      tower,
      wallet,
      ..
    } = ctx;

    let occupied_hotel_rooms: Vec<&Room> = tower
      .tower
      .find_rooms_by_type(RoomType::Hotel)
      .into_iter()
      .filter(|hotel_room| {
        tower
          .tower
          .find_occupants_by_hotel_room_id(&hotel_room.id)
          .len()
          > 0
      })
      .collect::<_>();

    let income = occupied_hotel_rooms
      .iter()
      .fold(0, |acc, room| acc + room.definition().income);

    let occupants_to_remove =
      occupied_hotel_rooms
        .into_iter()
        .fold(Vec::new(), |mut acc, occupied_hotel_room| {
          let mut occupant_ids: Vec<Uuid> = tower
            .tower
            .find_occupants_by_hotel_room_id(&occupied_hotel_room.id)
            .iter()
            .map(|occupant| occupant.id)
            .collect::<_>();

          acc.append(&mut occupant_ids);
          acc
        });

    tower.tower.remove_occupants(occupants_to_remove);

    wallet.add_funds(income);

    false
  }
}

impl Listener {
  pub fn new() -> Self {
    Self {
      timer_id: TimerId::Tick,
      listener_id: TimerListenerId::HotelCheckOut,
    }
  }
}
