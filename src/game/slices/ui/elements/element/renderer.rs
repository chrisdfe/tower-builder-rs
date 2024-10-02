use macroquad::text::{draw_text_ex, measure_text, TextDimensions, TextParams};

use crate::{
  game::{
    lifecycle::render::{get_text_settings, TextSettings, DEFAULT_FONT_SIZE},
    slices::ui::Resizability,
  },
  types::measurements::Dimensions,
};

use super::*;

pub type ContentRenderer = fn(element: &Element) -> ();
pub fn noop_content_renderer(_: &Element) {}
pub fn text_content_renderer(element: &Element) {
  let UnwrappedElementCalculatedProperties {
    content_position, ..
  } = element.calculated.unwrap();

  if let Some(text) = &element.text {
    let TextSettings {
      font,
      text_color,
      font_size,
      font_scale,
    } = get_text_settings();

    // TODO - prerender offset_y
    let TextDimensions { offset_y, .. } = measure_text(text, font, font_size, font_scale);

    // Draw content

    draw_text_ex(
      text,
      content_position.x,
      content_position.y + offset_y,
      TextParams {
        font,
        font_size,
        color: text_color,
        font_scale,
        ..Default::default()
      },
    );
  }
}

pub type ContentMeasurer = fn(element: &Element) -> Dimensions;
pub fn default_content_measurer(element: &Element) -> Dimensions {
  match &element.resizability {
    Resizability::Fixed(dimensions) => dimensions.clone(),
    _ => Dimensions::zero(),
  }
}

pub fn text_content_measurer(element: &Element) -> Dimensions {
  let TextSettings {
    font,
    font_size,
    font_scale,
    ..
  } = get_text_settings();
  if let Some(text) = &element.text {
    let text_dimensions = measure_text(text, font, font_size, font_scale);

    Dimensions {
      width: text_dimensions.width as u32,
      // TODO - something less hacky than this
      height: std::cmp::max(DEFAULT_FONT_SIZE as u32, text_dimensions.height as u32),
    }
  } else {
    // TODO - warning here
    Dimensions::zero()
  }
}
