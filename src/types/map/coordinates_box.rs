use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::types::measurements::{Dimensions, Rect};

use super::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoordinatesBox {
  lowest_x: i32,
  highest_x: i32,
  lowest_y: i32,
  highest_y: i32,
}

impl Default for CoordinatesBox {
  fn default() -> Self {
    CoordinatesBox::zero()
  }
}

impl CoordinatesBox {
  pub fn zero() -> Self {
    Self::at_coords(&Coordinates::zero())
  }

  pub fn at_coords(coords: &Coordinates) -> Self {
    Self::from_bottom_left_coords_and_dimensions(coords, &Dimensions::one())
  }

  pub fn from_start_and_end_coords(start: &Coordinates, end: &Coordinates) -> Self {
    use std::cmp::{max, min};

    let lowest_x = min(start.x, end.x);
    let highest_x = max(start.x, end.x);
    let lowest_y = min(start.y, end.y);
    let highest_y = max(start.y, end.y);

    Self {
      lowest_x,
      highest_x,
      lowest_y,
      highest_y,
    }
  }

  pub fn from_bottom_left_coords_and_dimensions(
    bottom_left_coords: &Coordinates,
    dimensions: &Dimensions,
  ) -> Self {
    let top_right_coordinates = Coordinates {
      x: bottom_left_coords.x + dimensions.width as i32 - 1,
      y: bottom_left_coords.y + dimensions.height as i32 - 1,
    };

    let lowest_x = bottom_left_coords.x;
    let highest_x = top_right_coordinates.x;
    let lowest_y = bottom_left_coords.y;
    let highest_y = top_right_coordinates.y;

    Self {
      lowest_x,
      highest_x,
      lowest_y,
      highest_y,
    }
  }

  pub fn dimensions(&self) -> Dimensions {
    Dimensions {
      width: (&self.highest_x - &self.lowest_x) as u32 + 1,
      height: (&self.highest_y - &self.lowest_y) as u32 + 1,
    }
  }

  pub fn bottom_left_coordinates(&self) -> Coordinates {
    Coordinates {
      x: self.lowest_x,
      y: self.lowest_y,
    }
  }

  pub fn top_right_coordinates(&self) -> Coordinates {
    Coordinates {
      x: self.highest_x,
      y: self.highest_y,
    }
  }

  // TODO - get_coordinates_iter or just 'iter' might be better
  pub fn get_coordinates_vec(&self) -> Vec<Coordinates> {
    let mut result: Vec<Coordinates> = Vec::new();

    for x in self.x_range() {
      for y in self.y_range() {
        result.push(Coordinates { x, y });
      }
    }

    result
  }

  pub fn x_range(&self) -> Range<i32> {
    self.lowest_x..self.highest_x + 1
  }

  pub fn y_range(&self) -> Range<i32> {
    self.lowest_y..self.highest_y + 1
  }

  pub fn bottom_row(&self) -> Vec<Coordinates> {
    self
      .x_range()
      .into_iter()
      .map(|x| Coordinates {
        x,
        y: self.lowest_y,
      })
      .collect()
  }

  pub fn contains(&self, coordinates: &Coordinates) -> bool {
    let bottom_left_coordinates = self.bottom_left_coordinates();
    let top_right_coordinates = self.top_right_coordinates();

    coordinates.x >= bottom_left_coordinates.x
      && coordinates.x <= top_right_coordinates.x
      && coordinates.y >= bottom_left_coordinates.y
      && coordinates.y <= bottom_left_coordinates.y
  }

  pub fn contains_y(&self, y: i32) -> bool {
    self.lowest_y <= y && self.highest_y >= y
  }

  pub fn as_rect(&self) -> Rect {
    Rect::from_coordinates_and_dimensions(&self.bottom_left_coordinates(), &self.dimensions())
  }

  pub fn overlaps_with(&self, other: &CoordinatesBox) -> bool {
    let a = self.as_rect();
    let b = other.as_rect();
    a.overlaps_with(&b)
  }

  pub fn cell_count(&self) -> u32 {
    let dimensions = self.dimensions();
    dimensions.width * dimensions.height
  }
}
