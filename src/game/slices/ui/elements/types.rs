use crate::types::measurements::{Axis, Dimensions};

#[derive(Debug, Clone)]
pub struct TwoDimensional<T> {
  pub horizontal: T,
  pub vertical: T,
}

impl<T> TwoDimensional<T> {
  pub fn same(value: T) -> Self
  where
    T: Clone,
  {
    Self {
      horizontal: value.clone(),
      vertical: value.clone(),
    }
  }

  pub fn get_value_for_axis(&self, axis: &Axis) -> &T {
    match axis {
      Axis::Horizontal => &self.horizontal,
      Axis::Vertical => &self.vertical,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Orthogonal<T> {
  pub top: T,
  pub right: T,
  pub bottom: T,
  pub left: T,
}

impl<T> Orthogonal<T> {
  pub fn same(value: T) -> Self
  where
    T: Clone,
  {
    Self {
      top: value.clone(),
      right: value.clone(),
      bottom: value.clone(),
      left: value.clone(),
    }
  }

  pub fn top_and_bottom(top_and_bottom: T, left_and_right: T) -> Self
  where
    T: Clone,
  {
    Self {
      top: top_and_bottom.clone(),
      right: left_and_right.clone(),
      bottom: top_and_bottom.clone(),
      left: left_and_right.clone(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resizability {
  Fixed(u32),
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

  pub fn extract_expand_to_fill_weight(self: &Self) -> u32 {
    match self {
      Self::ExpandToFill(weight) => *weight,
      _ => 0,
    }
  }
}

#[derive(Debug, Clone)]
pub enum ContentAlignment {
  Start,
  Center,
  End,
}

#[derive(Default, Debug, Clone)]
pub enum InsertMode {
  #[default]
  Append,
  Prepend,
}
