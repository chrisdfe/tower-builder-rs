use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::map::{Coordinates, CoordinatesBox};

use super::{
  occupants::Occupant,
  rooms::{
    definitions::{RoomDefinition, RoomType},
    Room,
  },
  route_finding::{find_route_from_lobby_to, Route},
};

#[derive(Serialize, Deserialize)]
pub struct Tower {
  pub rooms: Vec<Room>,
  pub occupants: Vec<Occupant>,
}

impl Tower {
  pub fn new() -> Self {
    Self {
      rooms: Vec::new(),
      occupants: Vec::new(),
    }
  }

  pub fn build_room(&mut self, definition: &RoomDefinition, selection_box: &CoordinatesBox) {
    let mut room = Room::from(definition);
    room.calculate_coordinates_box(selection_box);
    self.rooms.push(room);
  }

  pub fn find_room_at_coordinates(&self, coordinates: &Coordinates) -> Option<&Room> {
    self
      .rooms
      .iter()
      .find(|room| room.coordinates_box.contains(coordinates))
  }

  pub fn find_rooms_by_type(&self, room_type: RoomType) -> Vec<&Room> {
    self
      .rooms
      .iter()
      .filter(|room| room.definition().room_type == room_type)
      .collect::<_>()
  }

  pub fn find_rooms_by_floor(&self, floor: i32) -> Vec<&Room> {
    self
      .rooms
      .iter()
      .filter(|room| room.coordinates_box.contains_y(floor))
      .collect::<_>()
  }

  pub fn find_transportation_rooms_connected_to_floor(&self, floor: i32) -> Vec<Uuid> {
    self
      .find_rooms_by_floor(floor)
      .into_iter()
      .filter(|room| room.definition().room_type == RoomType::Transportation)
      .map(|room| room.id)
      .collect::<_>()
  }

  pub fn find_route_from_lobby_to_room(&self, room: &Room) -> Option<Route> {
    find_route_from_lobby_to(self, room.coordinates_box.bottom_left_coordinates().y)
  }

  pub fn room_is_connected_to_lobby(&self, room: &Room) -> bool {
    self.find_route_from_lobby_to_room(room).is_some()
  }

  /*
    Occupants
  */
  pub fn add_occupants(&mut self, occupants: Vec<Occupant>) {
    for o in occupants {
      self.occupants.push(o);
    }
  }

  pub fn remove_occupants(&mut self, occupant_ids: Vec<Uuid>) {
    for id in occupant_ids {
      let idx = self
        .occupants
        .iter()
        .position(|other| other.id == id);

      if let Some(idx) = idx {
        self.occupants.remove(idx);
      }
    }
  }

  pub fn find_occupants_by_office_id(&self, room_id: &Uuid) -> Vec<&Occupant> {
    self
      .occupants
      .iter()
      .filter(|occupant| occupant.office_id == Some(*room_id))
      .collect::<_>()
  }

  pub fn find_occupants_by_home_id(&self, room_id: &Uuid) -> Vec<&Occupant> {
    self
      .occupants
      .iter()
      .filter(|occupant| occupant.home_id == Some(*room_id))
      .collect::<_>()
  }

  pub fn find_occupants_by_hotel_room_id(&self, room_id: &Uuid) -> Vec<&Occupant> {
    self
      .occupants
      .iter()
      .filter(|occupant| occupant.hotel_room_id == Some(*room_id))
      .collect::<_>()
  }

  pub fn get_room_occupant_count(&self, room: &Room) -> usize {
    match room.definition().room_type {
      RoomType::Office => self.find_occupants_by_office_id(&room.id).len(),
      RoomType::Home => self.find_occupants_by_home_id(&room.id).len(),
      RoomType::Hotel => self
        .find_occupants_by_hotel_room_id(&room.id)
        .len(),
      _ => 0,
    }
  }

  // TODO - cache
  pub fn population(&self) -> usize {
    self.occupants.len()
  }

  pub fn find_route_from_lobby_to(&self, floor: i32) -> Option<Route> {
    find_route_from_lobby_to(self, floor)
  }
}
