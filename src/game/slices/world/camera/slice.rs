use serde::{Deserialize, Serialize};

use crate::types::map::Coordinates;

#[derive(Serialize, Deserialize)]
pub struct Slice {
  pub camera_position: Coordinates,
}

impl Slice {
  pub fn new() -> Self {
    Self {
      camera_position: Coordinates::zero(),
    }
  }

  pub fn add_camera_position(&mut self, coordinates: Coordinates) {
    self.camera_position.x += coordinates.x;
    self.camera_position.y += coordinates.y;
  }
}
