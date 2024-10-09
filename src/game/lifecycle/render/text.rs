use macroquad::{
  color::{Color, WHITE},
  text::{draw_text_ex, load_ttf_font, Font, TextParams},
};
use once_cell::sync::OnceCell;

use crate::types::measurements::Point;

pub const DEFAULT_TEXT_COLOR: Color = WHITE;
pub const DEFAULT_FONT_SIZE: u16 = 15;
pub const DEFAULT_FONT_SCALE: f32 = 1.;
pub const DEFAULT_LINE_HEIGHT: u32 = 30;

const DEFAULT_FONT_PATH: &'static str = "./fonts/space-mono/SpaceMono-Regular.ttf";

static UI_FONT: OnceCell<Font> = OnceCell::new();

pub async fn initialize() {
  let font = load_ttf_font(DEFAULT_FONT_PATH).await.unwrap();
  UI_FONT.set(font).unwrap();
}

pub fn get_font<'a>() -> Option<&'a Font> {
  Some(UI_FONT.get().unwrap())
}

pub struct TextSettings<'a> {
  // Realistically this will only ever be Some(font), but macroquad always accepts fonts wrapped in Options in fn arguments
  pub font: Option<&'a Font>,
  pub text_color: Color,
  pub font_size: u16,
  pub font_scale: f32,
}

pub fn get_text_settings<'a>() -> TextSettings<'a> {
  let font = get_font();

  TextSettings {
    font,
    text_color: DEFAULT_TEXT_COLOR,
    font_size: DEFAULT_FONT_SIZE,
    font_scale: DEFAULT_FONT_SCALE,
  }
}

pub fn render_text_custom(text: &String, point: &Point) {
  let font = Some(UI_FONT.get().unwrap());

  draw_text_ex(
    text,
    point.x,
    point.y,
    TextParams {
      font,
      font_size: DEFAULT_FONT_SIZE,
      color: DEFAULT_TEXT_COLOR,
      font_scale: DEFAULT_FONT_SCALE,
      ..Default::default()
    },
  );
}
