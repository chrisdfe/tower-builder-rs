use crate::{types::map::coordinates::Coordinates, game::slices::world::tower::Room};

use super::super::definitions::RoomDefinitionId;
use super::{
  RoomValidationContext, RoomValidationError, RoomValidatorResult,
  RoomValidatorResult::{Error, Ok},
  VALID_LOBBY_FLOORS,
};

use RoomValidationError::*;

pub fn validate_enough_funds(room: &Room, ctx: &RoomValidationContext) -> RoomValidatorResult {
  let RoomValidationContext { wallet, .. } = ctx;

  if wallet.funds < room.price() as i64 {
    Error(NotEnoughFunds)
  } else {
    Ok
  }
}

pub fn validate_rooms_do_not_overlap(
  room: &Room,
  ctx: &RoomValidationContext,
) -> RoomValidatorResult {
  let RoomValidationContext { tower, .. } = ctx;

  let definition = room.definition();

  let room_in_same_position = tower.rooms.iter().find(|other_room| {
    if other_room.definition().layer == definition.layer {
      room
        .coordinates_box
        .overlaps_with(&other_room.coordinates_box)
    } else {
      false
    }
  });

  if room_in_same_position.is_some() {
    Error(RoomsOverlap)
  } else {
    Ok
  }
}

pub fn validate_room_is_above_ground(
  room: &Room,
  _: &RoomValidationContext,
) -> RoomValidatorResult {
  if room.coordinates_box.bottom_left_coordinates().y < 0 {
    Error(BelowGround)
  } else {
    Ok
  }
}

pub fn validate_room_is_above_another_room(
  room: &Room,
  ctx: &RoomValidationContext,
) -> RoomValidatorResult {
  let RoomValidationContext { tower, .. } = ctx;

  if room.coordinates_box.bottom_left_coordinates().y > 0 {
    let mut has_failed = false;

    let bottom_row = room.coordinates_box.bottom_row();

    for coordinates in bottom_row {
      let room_below = tower.find_room_at_coordinates(&Coordinates {
        x: coordinates.x,
        y: coordinates.y - 1,
      });

      if room_below.is_none() {
        has_failed = true;
        break;
      }
    }

    if has_failed {
      Error(InvalidOverhang)
    } else {
      Ok
    }
  } else {
    Ok
  }
}

pub fn validate_lobby_is_on_correct_floor(
  room: &Room,
  _: &RoomValidationContext,
) -> RoomValidatorResult {
  if !VALID_LOBBY_FLOORS.contains(&(room.coordinates_box.bottom_left_coordinates().y as u32)) {
    Error(LobbyInvalidFloor)
  } else {
    Ok
  }
}

pub fn validate_non_lobby_is_not_on_ground_floor(
  room: &Room,
  _: &RoomValidationContext,
) -> RoomValidatorResult {
  if room.coordinates_box.bottom_left_coordinates().y == 0
    && room.definition_id != RoomDefinitionId::Lobby
  {
    Error(LobbyInvalidFloor)
  } else {
    Ok
  }
}

pub fn validate_transportation_room_is_within_tower_bounds(
  room: &Room,
  ctx: &RoomValidationContext,
) -> RoomValidatorResult {
  let RoomValidationContext { tower, .. } = ctx;
  let coordinates_vec = room.coordinates_box.get_coordinates_vec();

  let cell_inside_tower = coordinates_vec.into_iter().find(|coordinates| {
    tower
      .find_room_at_coordinates(coordinates)
      .is_some()
  });

  if cell_inside_tower.is_none() {
    Error(TransportationNotInsideTower)
  } else {
    Ok
  }
}
