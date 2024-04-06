use crate::measurements::{Axis, Dimensions};

#[derive(Debug, Clone)]
pub struct TwoDimensional<T> {
  pub horizontal: T,
  pub vertical: T,
}

impl<T> TwoDimensional<T> {
  pub fn get_value_for_axis(&self, axis: &Axis) -> &T {
    match axis {
      Axis::Horizontal => &self.horizontal,
      Axis::Vertical => &self.vertical,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resizability {
  Fixed(Dimensions),
  ShrinkToFit,
  ExpandToFill(u32),
}

impl Resizability {
  pub fn is_expand_to_fill(self: &Self) -> bool {
    match self {
      Resizability::ExpandToFill(_) => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone)]
pub enum ContentAlignment {
  Start,
  Center,
  End,
}
