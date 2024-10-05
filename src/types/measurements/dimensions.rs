use macroquad::window::{screen_height, screen_width};

use super::Axis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimensions {
  pub width: u32,
  pub height: u32,
}

impl Default for Dimensions {
  fn default() -> Self {
    Self::zero()
  }
}

impl std::ops::Add for Dimensions {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      width: self.width + rhs.width,
      height: self.height + rhs.height,
    }
  }
}

impl std::ops::AddAssign for Dimensions {
  fn add_assign(&mut self, rhs: Self) {
    self.width += rhs.width;
    self.height += rhs.height;
  }
}

impl std::fmt::Display for Dimensions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}x{}", self.width, self.height)
  }
}

impl Dimensions {
  pub fn zero() -> Self {
    Self {
      width: 0,
      height: 0,
    }
  }

  pub fn one() -> Self {
    Self {
      width: 1,
      height: 1,
    }
  }

  pub fn square(value: u32) -> Self {
    Self {
      width: value,
      height: value,
    }
  }

  pub fn of_screen() -> Self {
    Self {
      width: screen_width() as u32,
      height: screen_height() as u32,
    }
  }

  pub fn get_length_for_axis(&self, axis: &Axis) -> u32 {
    match axis {
      Axis::Horizontal => self.width,
      Axis::Vertical => self.height,
    }
  }

  pub fn set_for_axis(&mut self, value: u32, axis: &Axis) {
    match axis {
      Axis::Horizontal => self.width = value,
      Axis::Vertical => self.height = value,
    }
  }
}
