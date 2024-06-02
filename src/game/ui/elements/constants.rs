use lazy_static::lazy_static;
use macroquad::color::Color;

use crate::tower::rooms::definitions::RoomDefinitionId;

use super::{
  interactivity::{Action, ElementEventHandlers, InteractivityConfig},
  BackgroundColorKind, ElementConfig,
};

lazy_static! {
  pub static ref BUTTON_BACKGROUND_COLOR_BASE: Color = Color::new(1., 0., 0., 1.);
  pub static ref BUTTON_BACKGROUND_COLOR_OVER: Color = Color::new(0., 1., 0., 1.);
  pub static ref BUTTON_BACKGROUND_COLOR_DOWN: Color = Color::new(0., 0., 1., 1.);
}
