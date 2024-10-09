use super::super::*;

#[derive(Clone)]
pub struct NoopElementContentRenderer;

impl ElementContentRenderer for NoopElementContentRenderer {
  fn render(&self, element: &Element, ctx: &Game) {}

  fn measure(&self, element: &Element) -> Dimensions {
    Dimensions::zero()
  }

  fn box_clone(&self) -> Box<dyn ElementContentRenderer> {
    Box::new((*self).clone())
  }
}
