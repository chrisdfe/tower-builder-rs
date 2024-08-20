use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::slices::world::tower::rooms::definitions::RoomResizability;
use crate::map::coordinates_box::CoordinatesBox;
use crate::{
  map::Coordinates,
  measurements::Dimensions,
  utils::{comma_seperate_number_vec, round_to_nearest},
};

use super::definitions::{RoomDefinition, RoomDefinitionId, ROOM_DEFINITIONS};

use super::validation::{
  RoomValidationContext, RoomValidationError, RoomValidatorResult, VALID_LOBBY_FLOORS,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
  pub id: Uuid,
  pub definition_id: RoomDefinitionId,
  pub coordinates_box: CoordinatesBox,

  pub validation_errors: Vec<RoomValidationError>,
}

impl Default for Room {
  fn default() -> Self {
    let definition_id = RoomDefinitionId::Lobby;

    Self {
      id: Uuid::new_v4(),
      definition_id,
      validation_errors: Vec::new(),
      coordinates_box: CoordinatesBox::zero(),
    }
  }
}

impl From<&RoomDefinition> for Room {
  fn from(definition: &RoomDefinition) -> Self {
    Self {
      definition_id: definition.id,
      ..Default::default()
    }
  }
}

impl Room {
  pub fn definition(&self) -> &'static RoomDefinition {
    ROOM_DEFINITIONS.get(&self.definition_id).unwrap()
  }

  pub fn price(&self) -> u32 {
    let definition = self.definition();

    if definition.resizability == RoomResizability::None {
      definition.price
    } else {
      definition.price * self.coordinates_box.cell_count()
    }
  }

  pub fn calculate_coordinates_box(&mut self, selection_box: &CoordinatesBox) {
    let definition = self.definition();

    let rounded_dimensions = {
      use std::cmp::max;

      let width = {
        if definition.resizability == RoomResizability::Horizontal
          || definition.resizability == RoomResizability::Both
        {
          // TODO - round up to nearest size
          let width = round_to_nearest(
            selection_box.dimensions().width,
            definition.dimensions.width,
          );
          max(width, 1)
        } else {
          definition.dimensions.width
        }
      };
      let height = {
        if definition.resizability == RoomResizability::Vertical
          || definition.resizability == RoomResizability::Both
        {
          let height = round_to_nearest(
            selection_box.dimensions().height,
            definition.dimensions.height,
          );
          max(height, 1)
        } else {
          definition.dimensions.height
        }
      };

      Dimensions { width, height }
    };

    let offset = {
      let x = (definition.dimensions.width / 2) as i32 * -1;
      let y = (definition.dimensions.height / 2) as i32 * -1;
      Coordinates { x, y }
    };

    let coordinates = Coordinates {
      x: selection_box.bottom_left_coordinates().x + offset.x,
      y: selection_box.bottom_left_coordinates().y + offset.y,
    };

    self.coordinates_box =
      CoordinatesBox::from_bottom_left_coords_and_dimensions(&coordinates, &rounded_dimensions);
  }

  pub fn validate(&mut self, ctx: RoomValidationContext) {
    self.validation_errors = self
      .definition()
      .validators
      .iter()
      .map(|validator| {
        if let RoomValidatorResult::Error(validation_error) = validator(self, &ctx) {
          Some(validation_error)
        } else {
          None
        }
      })
      .filter(|r| r.is_some())
      .map(|r| r.unwrap())
      .collect::<Vec<_>>()
  }

  pub fn is_valid(&self) -> bool {
    self.validation_errors.len() == 0
  }

  // TODO - don't crash if validation_errors is empty
  pub fn get_first_validation_error_message(&self) -> String {
    use RoomValidationError::*;
    // TODO - put this in impl RoomValidationError
    match self.validation_errors[0] {
      NotEnoughFunds => String::from("Insufficient funds."),
      RoomsOverlap => String::from("Cannot overlap with another room."),
      LobbyInvalidFloor => format!(
        "Lobby must be on floors {}",
        comma_seperate_number_vec(VALID_LOBBY_FLOORS.to_vec(), String::from("or"))
      ),
      BelowGround => String::from("Room must be built above ground"),
      InvalidOverhang => String::from("Room must be built above another room"),
      TransportationNotInsideTower => {
        // String::from("Transportation must be built completely inside of tower")
        String::from("Transportation ")
      }
    }
  }
}
