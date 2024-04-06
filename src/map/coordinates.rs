use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coordinates {
  pub x: i32,
  pub y: i32,
}

impl Coordinates {
  pub fn zero() -> Self {
    Self { x: 0, y: 0 }
  }

  // pub fn one() -> Self {
  //   Self { x: 1, y: 1 }
  // }
}

impl Default for Coordinates {
  fn default() -> Self {
    Coordinates::zero()
  }
}

impl Add for Coordinates {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Sub for Coordinates {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}
