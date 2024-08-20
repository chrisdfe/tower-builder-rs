use uuid::Uuid;

use crate::game::slices::timers::{
  Timer, TimerCallbackContext, TimerId, TimerListener, TimerListenerId,
};
use crate::types::map::Coordinates;
use crate::game::slices::world::tower::rooms::{Room, RoomType};
use crate::game::slices::world::tower::Occupant;

// 6pm
const HOTEL_CHECK_IN_TIME: u32 = 18;

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

    current_time.minute == 0 && current_time.hour == HOTEL_CHECK_IN_TIME
  }

  fn on_timer_complete(&mut self, _: &Timer, ctx: TimerCallbackContext) -> bool {
    let TimerCallbackContext { tower, .. } = ctx;

    // TODO - check for transportation

    let unoccupied_connected_hotel_rooms: Vec<&Room> = tower
      .tower
      .find_rooms_by_type(RoomType::Hotel)
      .into_iter()
      // unoccupied
      .filter(|hotel_room| {
        tower
          .tower
          .find_occupants_by_hotel_room_id(&hotel_room.id)
          .is_empty()
      })
      // connected to transportation
      .filter(|hotel_room| tower.tower.room_is_connected_to_lobby(hotel_room))
      .collect::<_>();

    let new_occupants = unoccupied_connected_hotel_rooms.into_iter().fold(
      Vec::new(),
      |mut acc, unoccupied_hotel_room| {
        let mut occupants: Vec<Occupant> = (0..unoccupied_hotel_room.definition().occupancy_limit)
          .into_iter()
          .map(|_| Occupant {
            id: Uuid::new_v4(),
            current_cell: Some(Coordinates::zero()),
            office_id: None,
            home_id: None,
            hotel_room_id: Some(unoccupied_hotel_room.id),
          })
          .collect::<_>();

        acc.append(&mut occupants);
        acc
      },
    );

    tower.tower.add_occupants(new_occupants);

    false
  }
}

impl Listener {
  pub fn new() -> Self {
    Self {
      timer_id: TimerId::Tick,
      listener_id: TimerListenerId::HotelCheckIn,
    }
  }
}
