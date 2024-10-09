use std::collections::HashMap;
use std::str::FromStr;

use lazy_static::lazy_static;
use macroquad::texture::Texture2D;
use serde::{Deserialize, Serialize};

// Re-export for ease of use in RoomDefinition modules
pub(self) use crate::game::slices::world::tower::rooms::validation::{
  validators, with_base_room_validators, RoomValidator,
};
pub(self) use crate::types::measurements::Dimensions;
pub(self) use macroquad::color::Color;

mod condo;
mod elevator_single;
mod floor;
mod hotel_single;
mod lobby;
mod lobby_large;
mod office;
mod stairs;

lazy_static! {
  pub static ref ROOM_DEFINITIONS: HashMap<RoomDefinitionId, RoomDefinition> = [
    floor::get_definition(),
    lobby::get_definition(),
    lobby_large::get_definition(),
    office::get_definition(),
    condo::get_definition(),
    hotel_single::get_definition(),
    stairs::get_definition(),
    elevator_single::get_definition(),
  ]
  .into_iter()
  .map(|def| (def.id, def))
  .collect();
}

// Just a stop-gap until all rooms have images
#[derive(Debug)]
pub enum RoomDefinitionRenderType {
  Color(Color),
  Image(Texture2D),
}

#[derive(Debug)]
pub struct RoomDefinition {
  pub id: RoomDefinitionId,
  pub room_type: RoomType,
  pub dimensions: Dimensions,
  pub validators: Vec<RoomValidator>,
  pub price: u32,
  pub layer: RoomLayer,
  pub resizability: RoomResizability,
  pub occupancy_limit: u32,
  // rent for offices, nightly price for hotel rooms, sale price for condos
  pub income: u32,
  pub render_type: RoomDefinitionRenderType,
}

impl Default for RoomDefinition {
  fn default() -> Self {
    lobby::get_definition()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomDefinitionId {
  Floor,
  Lobby,
  LobbyLarge,
  Office,
  Condo,
  HotelSingle,
  Stairs,
  ElevatorSingle,
}

impl Default for RoomDefinitionId {
  fn default() -> Self {
    Self::Lobby
  }
}

impl FromStr for RoomDefinitionId {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "Floor" => Ok(Self::Floor),
      "Lobby" => Ok(Self::Lobby),
      "LobbyLarge" => Ok(Self::LobbyLarge),
      "Office" => Ok(Self::Office),
      "Condo" => Ok(Self::Condo),
      "HotelSingle" => Ok(Self::HotelSingle),
      "Stairs" => Ok(Self::Stairs),
      "ElevatorSingle" => Ok(Self::ElevatorSingle),
      _ => Err(()),
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RoomResizability {
  None,
  Horizontal,
  Vertical,
  Both,
}

#[derive(Debug, PartialEq)]
pub enum RoomLayer {
  Default,
  Transportation,
}

#[derive(Debug, PartialEq)]
pub enum RoomType {
  Lobby,
  Transportation,
  // Non-occupiable
  None,
  Office,
  Home,
  // Restaurant,
  Hotel,
}
