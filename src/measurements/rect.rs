use super::{Dimensions, Point};
use crate::map::Coordinates;

pub struct Rect {
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
}

impl Default for Rect {
  fn default() -> Self {
      Self {
        x: 0 ,
        y: 0,
        width: 0,
        height: 0
      }
  }
}

impl Rect {
  pub fn from_coordinates_and_dimensions(coordinates: &Coordinates, dimensions: &Dimensions) -> Self {
      let Coordinates { x, y } = coordinates.clone();
      let Dimensions { width, height } = dimensions.clone();
      Self {
        x,
        y,
        width,
        height,
      }
    }
 pub fn from_point_and_dimensions(point: &Point, dimensions: &Dimensions) -> Self {
      let Point { x, y } = point.clone();
      let Dimensions { width, height } = dimensions.clone();
      Self {
        x: x as i32,
        y: y as i32,
        width,
        height,
      }
    }
  
  
  // Rect overlap algorithm taken from
  // https://stackoverflow.com/a/306379
  pub fn rects_overlap(a: &Rect, b: &Rect) -> bool {
    let x_overlap =
      (b.x..b.x + b.width as i32).contains(&a.x) || 
      //-
      (a.x..a.x + a.width as i32).contains(&b.x);

    let y_overlap = 
      (b.y..b.y + b.height as i32).contains(&a.y) ||
      // -
      (a.y..a.y + a.height as i32).contains(&b.y);

    x_overlap && y_overlap
  }

  pub fn overlaps_with(&self, other: &Rect) -> bool {
    Rect::rects_overlap(self, other)
  }

  // TODO - I could consolidate this with coordinates_box
  pub fn contains_point(&self, point: &Point) -> bool {
    point.x >= self.x as f32
      && point.x <= self.max_x() as f32
      && point.y >= self.y as f32
      && point.y <= self.max_y() as f32
  }

  pub fn max_x(&self) -> i32 {
    self.x + self.width as i32
  }

    pub fn max_y(&self) -> i32 {
    self.y + self.height as i32
  }
}
