use macroquad::color::Color;

use crate::measurements::{Axis, Dimensions, Point, Rect};

use super::{
  interactivity::ElementEventHandlers,
  types::{ContentAlignment, Resizability},
  TwoDimensional,
};

#[derive(Debug, Clone)]
pub struct Element {
  pub name: String,
  // text will be ignored for wrapper nodes (i.e if its node has children)
  pub text: String,
  pub config: ElementConfig,
  pub calculated: ElementCalculatedProperties,
}

impl Default for Element {
  fn default() -> Self {
    Self {
      name: String::from("untitled node"),
      config: Default::default(),
      calculated: Default::default(),
      text: String::from(""),
    }
  }
}

impl Element {
  pub fn new(input: ElementInput) -> Element {
    let ElementInput { name, text, config } = input;

    Element {
      name,
      text,
      config,
      calculated: Default::default(),
    }
  }
}

#[derive(Clone)]
pub struct ElementInput {
  // Mostly for debugging reasons
  pub name: String,
  // pub parent_id: Option<Uuid>,
  pub text: String,
  pub config: ElementConfig,
}

impl Default for ElementInput {
  fn default() -> Self {
    Self {
      name: String::from("unnamed node (from input)"),
      // parent_id: None,
      text: String::from(""),
      config: ElementConfig::default(),
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BackgroundColorKind {
  None,
  Fixed(Color),
  Randomized,
}

#[derive(Debug, Clone)]
pub struct ElementConfig {
  // dimensions/position
  pub padding: u32,
  pub child_gap: u32,
  pub resizability: Resizability,
  pub stack_axis: Axis,
  pub content_alignment: TwoDimensional<ContentAlignment>,

  // Colors
  pub background_color: BackgroundColorKind,

  // Interactivity
  pub event_handlers: ElementEventHandlers,
}

impl Default for ElementConfig {
  fn default() -> Self {
    Self {
      padding: 0,
      child_gap: 0,
      resizability: Resizability::ShrinkToFit,
      stack_axis: Axis::Horizontal,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },

      background_color: BackgroundColorKind::Randomized,

      event_handlers: ElementEventHandlers::new(),
    }
  }
}

impl ElementConfig {
  pub fn is_interactive(&self) -> bool {
    self
      .event_handlers
      .has_at_least_one_not_none_handler()
  }
}

#[derive(Debug, Clone, Default)]
pub struct ElementCalculatedProperties {
  pub outer_dimensions: Option<Dimensions>,
  pub content_dimensions: Option<Dimensions>,
  pub children_dimensions: Option<Dimensions>,

  pub outer_position: Option<Point>,
  pub content_position: Option<Point>,

  pub background_color: Option<Color>,
}

impl ElementCalculatedProperties {
  /// Unwraps all properties
  /// A convenience fn for when we are totally sure that a layout node has been calculated
  pub fn unwrap(&self) -> UnwrappedElementCalculatedProperties {
    UnwrappedElementCalculatedProperties {
      outer_dimensions: self.outer_dimensions.as_ref().unwrap(),
      content_dimensions: self.content_dimensions.as_ref().unwrap(),
      children_dimensions: self.content_dimensions.as_ref().unwrap(),

      outer_position: self.outer_position.as_ref().unwrap(),
      content_position: self.content_position.as_ref().unwrap(),

      background_color: self.background_color.as_ref().unwrap(),
    }
  }

  /// Sets all properties to `None`
  pub fn clear(&mut self) {
    self.outer_dimensions = None;
    self.content_dimensions = None;
    self.children_dimensions = None;

    self.content_position = None;
    self.outer_position = None;

    self.background_color = None;
  }

  pub fn outer_as_rect(&self) -> Rect {
    let outer_position = self.outer_position.as_ref().unwrap();
    let outer_dimensions = self.outer_dimensions.as_ref().unwrap();
    Rect::from_point_and_dimensions(outer_position, outer_dimensions)
  }

  /// Gets the calculated outer/content/children dimensions field for the specified axis.
  /// Panics if any of outer/content/children dimensions is None.
  pub fn get_sizes_for_axis(&self, axis: &Axis) -> (u32, u32, u32) {
    (
      self.get_outer_size_for_axis(axis),
      self.get_content_size_for_axis(axis),
      self.get_children_size_for_axis(axis),
    )
  }

  /// Gets the calculated outer dimensions field for the specified axis.
  /// Panics if outer_dimensions is None.
  pub fn get_outer_size_for_axis(&self, axis: &Axis) -> u32 {
    self
      .outer_dimensions
      .as_ref()
      .unwrap()
      .get_length_for_axis(axis)
  }

  /// Gets the calculated content_dimensions field for the specified axis.
  /// Panics if content_dimensions is None.
  pub fn get_content_size_for_axis(&self, calculation_axis: &Axis) -> u32 {
    self
      .content_dimensions
      .as_ref()
      .unwrap()
      .get_length_for_axis(calculation_axis)
  }

  /// Gets the calculated children_dimensions field for the specified axis.
  /// Panics if children_dimensions is None.
  pub fn get_children_size_for_axis(&self, calculation_axis: &Axis) -> u32 {
    self
      .children_dimensions
      .as_ref()
      .unwrap()
      .get_length_for_axis(calculation_axis)
  }

  /// Sets the specified axis of outer_dimensions to `value`
  /// panics if outer_dimensions is `None`
  pub fn set_outer_dimensions_for_axis(&mut self, value: u32, axis: &Axis) {
    self
      .outer_dimensions
      .as_mut()
      .unwrap()
      .set_for_axis(value, axis);
  }

  /// Sets the specified axis of outer_dimensions to `value`
  /// panics if content_dimensions is `None`
  pub fn set_content_dimensions_for_axis(&mut self, value: u32, axis: &Axis) {
    self
      .content_dimensions
      .as_mut()
      .unwrap()
      .set_for_axis(value, axis);
  }
}

pub struct UnwrappedElementCalculatedProperties<'a> {
  pub outer_dimensions: &'a Dimensions,
  pub content_dimensions: &'a Dimensions,
  pub children_dimensions: &'a Dimensions,

  pub content_position: &'a Point,
  pub outer_position: &'a Point,

  pub background_color: &'a Color,
}
