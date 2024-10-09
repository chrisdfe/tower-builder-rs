use calculated::UnwrappedElementCalculatedProperties;
use macroquad::text::{draw_text_ex, measure_text, TextDimensions, TextParams};

use crate::{
  game::{
    lifecycle::render::text::{get_text_settings, TextSettings},
    Game,
  },
  types::measurements::Dimensions,
};

use super::*;
pub mod generic;

pub trait ElementContentRenderer {
  fn render(&self, element: &Element, ctx: &Game) -> ();
  fn measure(&self, element: &Element) -> Dimensions;
  fn box_clone(&self) -> Box<dyn ElementContentRenderer>;
}

impl Clone for Box<dyn ElementContentRenderer> {
  fn clone(&self) -> Self {
    self.box_clone()
  }
}
