use super::Point;

pub struct Coordinates {
  pub x: u32,
  pub y: u32,
}

impl From<Point> for Coordinates {
  fn from(value: Point) -> Self {
    Self {
      x: value.x as u32,
      y: value.y as u32,
    }
  }
}
