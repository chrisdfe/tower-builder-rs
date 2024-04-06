use super::Axis;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  pub fn zero() -> Self {
    Self { x: 0., y: 0. }
  }

  pub fn get_value_for_axis(&self, axis: &Axis) -> f32 {
    match axis {
      Axis::Horizontal => self.x,
      Axis::Vertical => self.y,
    }
  }
}
