use calculated::UnwrappedElementCalculatedProperties;
use macroquad::text::{draw_text_ex, measure_text, TextDimensions, TextParams};

use crate::{
  game::{
    lifecycle::render::{get_text_settings, TextSettings, DEFAULT_FONT_SIZE},
    Game,
  },
  types::measurements::Dimensions,
};

use super::*;

#[derive(Debug, Clone)]
pub struct ElementContentRenderer {
  pub render: fn(element: &Element, ctx: &Game) -> (),
  pub measure: fn(element: &Element) -> Dimensions,
}

pub const NOOP_ELEMENT_CONTENT_RENDERER: ElementContentRenderer = ElementContentRenderer {
  render: |_element: &Element, _: &Game| {},
  measure: |_element: &Element| Dimensions::zero(),
};

pub const TEXT_ELEMENT_CONTENT_RENDERER: ElementContentRenderer = ElementContentRenderer {
  render: |element, _: &Game| {
    //
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
  },

  measure: |element| {
    let TextSettings {
      font,
      font_size,
      font_scale,
      ..
    } = get_text_settings();
    if let Some(text) = &element.text {
      let text_dimensions = measure_text(text, font, font_size, font_scale);

      // TODO - a more robust solution than this
      // let height = std::cmp::max(DEFAULT_FONT_SIZE as u32, text_dimensions.height as u32);
      let height = DEFAULT_FONT_SIZE as u32;
      // let height = text_dimensions.height as u32;

      Dimensions {
        width: text_dimensions.width as u32,
        height,
      }
    } else {
      // TODO - warning here
      Dimensions::zero()
    }
  },
};
