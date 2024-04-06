use serde::{Deserialize, Serialize};

use crate::game::world::WalletSlice;
use crate::tower::{Room, Tower};

pub mod validators;

pub const VALID_LOBBY_FLOORS: [u32; 8] = [0, 15, 30, 45, 60, 75, 90, 100];

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomValidationError {
  NotEnoughFunds,
  RoomsOverlap,
  BelowGround,
  LobbyInvalidFloor,
  InvalidOverhang,
  TransportationNotInsideTower,
}

pub enum RoomValidatorResult {
  Error(RoomValidationError),
  Ok,
}

pub type RoomValidator = fn(room: &Room, ctx: &RoomValidationContext) -> RoomValidatorResult;

pub struct RoomValidationContext<'a> {
  pub tower: &'a Tower,
  pub wallet: &'a WalletSlice,
}

pub const BASE_ROOM_VALIDATORS: [RoomValidator; 3] = [
  validators::validate_enough_funds,
  validators::validate_rooms_do_not_overlap,
  validators::validate_room_is_above_another_room,
];

pub fn with_base_room_validators(validators: Vec<RoomValidator>) -> Vec<RoomValidator> {
  let mut result = BASE_ROOM_VALIDATORS.to_vec();
  result.extend(validators);
  result
}
