use super::super::*;

#[derive(Clone)]
pub struct TextElementContentRenderer;

impl ElementContentRenderer for TextElementContentRenderer {
  fn render(&self, element: &Element, _: &Game) {
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
  }

  fn measure(&self, element: &Element) -> Dimensions {
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
      // let height = DEFAULT_FONT_SIZE as u32;
      // let height = DEFAULT_LINE_HEIGHT as u32;
      let height = text_dimensions.height as u32;

      Dimensions {
        width: text_dimensions.width as u32,
        height,
      }
    } else {
      // TODO - warning here
      Dimensions::zero()
    }
  }

  fn box_clone(&self) -> Box<dyn ElementContentRenderer> {
    Box::new((*self).clone())
  }
}
