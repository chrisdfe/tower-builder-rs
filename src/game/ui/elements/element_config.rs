use macroquad::color::Color;

use crate::measurements::Axis;

use super::{
  types::{ContentAlignment, Resizability},
  TwoDimensional,
};

#[derive(Debug, Clone)]
pub enum BackgroundColorKind {
  None,
  Fixed(Color),
  Randomized,
}

#[derive(Debug, Clone)]
pub struct ElementConfig {
  pub padding: u32,
  pub child_gap: u32,
  pub resizability: Resizability,
  pub stack_axis: Axis,
  pub content_alignment: TwoDimensional<ContentAlignment>,
  pub background_color: BackgroundColorKind,
}

impl Default for ElementConfig {
  fn default() -> Self {
    Self {
      padding: 0,
      child_gap: 0,
      resizability: Resizability::ShrinkToFit,
      stack_axis: Axis::Horizontal,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },
      background_color: BackgroundColorKind::Randomized,
    }
  }
}
