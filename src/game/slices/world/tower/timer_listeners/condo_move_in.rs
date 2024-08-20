use uuid::Uuid;

use crate::game::slices::timers::{
  should_run_tick_at_infrequency, Timer, TimerCallbackContext, TimerId, TimerListener,
  TimerListenerId,
};
use crate::types::map::Coordinates;
use crate::game::slices::world::tower::rooms::{Room, RoomType};
use crate::game::slices::world::tower::Occupant;

const INFREQUENCY: u64 = 5;

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
    let TimerCallbackContext { tower, wallet, .. } = ctx;

    let condos = tower.tower.find_rooms_by_type(RoomType::Home);

    let unoccupied_connected_condos: Vec<&Room> = condos
      .into_iter()
      // unoccupied
      .filter(|office| {
        tower
          .tower
          .find_occupants_by_home_id(&office.id)
          .len()
          == 0
      })
      // connected to transportation
      .filter(|hotel_room| tower.tower.room_is_connected_to_lobby(hotel_room))
      .collect::<_>();

    // TODO - check for transportation

    let new_occupants =
      unoccupied_connected_condos
        .iter()
        .fold(Vec::new(), |mut acc, unoccupied_condo| {
          let mut occupants: Vec<Occupant> = (0..unoccupied_condo.definition().occupancy_limit)
            .into_iter()
            .map(|_| Occupant {
              id: Uuid::new_v4(),
              current_cell: Some(Coordinates::zero()),
              office_id: None,
              home_id: Some(unoccupied_condo.id.clone()),
              hotel_room_id: None,
            })
            .collect::<_>();

          acc.append(&mut occupants);
          acc
        });

    let income = unoccupied_connected_condos
      .iter()
      .fold(0, |acc, condo| acc + condo.definition().income);
    wallet.add_funds(income);

    tower.tower.add_occupants(new_occupants);

    false
  }
}

impl Listener {
  pub fn new() -> Self {
    Self {
      timer_id: TimerId::Tick,
      listener_id: TimerListenerId::CondoMoveIn,
    }
  }
}
