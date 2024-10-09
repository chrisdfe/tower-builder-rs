use std::collections::HashMap;

use macroquad::color::Color;

use crate::types::measurements::Axis;

use super::{
  super::{
    types::{ContentAlignment, Resizability},
    TwoDimensional,
  },
  actions::ElementActionCreator,
  calculated::*,
  interactivity::*,
  renderer::{self, *},
};

#[derive(Clone)]
pub struct Element {
  pub name: String,

  pub handle: &'static str,
  pub tags: Vec<ElementTag>,

  // text will be ignored for wrapper nodes (i.e if its node has children)
  pub text: Option<String>,
  pub content_renderer: Box<dyn ElementContentRenderer>,

  // dimensions/position
  pub padding: u32,
  pub child_gap: u32,
  pub resizability: TwoDimensional<Resizability>,
  pub stack_axis: Axis,
  pub content_alignment: TwoDimensional<ContentAlignment>,

  // Colors
  pub background_color: BackgroundColorKind,

  // Interactivity
  pub interactivity: Option<ElementInteractivity>,

  // TODO - should be renamed to 'needs update' or 'get update action' or something
  pub on_update: Option<ElementActionCreator>,

  pub calculated: ElementCalculatedProperties,

  // TODO - this seems pretty horrible
  pub attributes: HashMap<&'static str, String>,
}

impl Default for Element {
  fn default() -> Self {
    Self {
      name: String::from("untitled node"),
      handle: "",
      tags: Vec::new(),
      text: None,

      padding: 0,
      child_gap: 0,
      resizability: TwoDimensional::same(Resizability::ShrinkToFit),
      stack_axis: Axis::Horizontal,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },

      background_color: BackgroundColorKind::None,
      content_renderer: Box::new(renderer::generic::TextElementContentRenderer),
      on_update: None,

      interactivity: None,
      calculated: Default::default(),
      attributes: HashMap::new(),
    }
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ElementTag {
  #[default]
  None,
  RoomDefinitionButton,
  ToolButton,
}

#[derive(Debug, Clone, Copy)]
pub enum BackgroundColorKind {
  None,
  Fixed(Color),
  Randomized,
}

impl Default for BackgroundColorKind {
  fn default() -> Self {
    Self::None
  }
}
