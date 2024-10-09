use macroquad::prelude::*;
use macroquad::texture::Image;

use crate::types::measurements::Point;

use super::super::*;

#[derive(Clone)]
pub struct ImageElementContentRenderer {
  image: Image,
}

impl ImageElementContentRenderer {
  pub fn new(path: String) -> Self {
    let bytes = std::fs::read(path).unwrap();
    // TODO - texture instead I think
    let image = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).unwrap();

    //
    Self { image }
  }
}

impl ElementContentRenderer for ImageElementContentRenderer {
  fn render(&self, element: &Element, ctx: &Game) -> () {
    let texture = Texture2D::from_image(&self.image);
    // TODO - element position
    let Point { x, y, .. } = element.calculated.unwrap().content_position;
    draw_texture(&texture, *x, *y, WHITE);
  }

  fn measure(&self, _: &Element) -> Dimensions {
    Dimensions {
      width: self.image.width() as u32,
      height: self.image.height() as u32,
    }
  }

  fn box_clone(&self) -> Box<dyn ElementContentRenderer> {
    Box::new((*self).clone())
  }
}
