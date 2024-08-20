use lazy_static::lazy_static;
use macroquad::color::Color;

lazy_static! {
  pub static ref BUTTON_BACKGROUND_COLOR_BASE: Color = Color::new(1., 0., 0., 1.);
  pub static ref BUTTON_BACKGROUND_COLOR_OVER: Color = Color::new(0., 1., 0., 1.);
  pub static ref BUTTON_BACKGROUND_COLOR_DOWN: Color = Color::new(0., 0., 1., 1.);
}
