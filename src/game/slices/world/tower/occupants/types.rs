use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::map::Coordinates;

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Occupant {
  pub id: Uuid,

  pub current_cell: Option<Coordinates>,

  pub office_id: Option<Uuid>,
  pub home_id: Option<Uuid>,
  pub hotel_room_id: Option<Uuid>,
}

// impl Occupant {
//   fn new() -> Self {
//     Self {
//       current_cell: None,

//       office_id: None,
//       home_id: None,
//     }
//   }
// }
