use uuid::Uuid;

use crate::{measurements::Axis, types::tree::TreeNode};

use super::{ElementCalculatedProperties, ElementConfig, ElementInput};

#[derive(Debug, Clone)]
pub struct Element {
  pub name: String,
  pub id: Uuid,
  // text will be ignored for wrapper nodes (i.e if its tree node has children)
  pub text: String,
  pub config: ElementConfig,
  pub calculated: ElementCalculatedProperties,
}

impl Default for Element {
  fn default() -> Self {
    Self {
      name: String::from("untitled node"),
      id: Uuid::new_v4(),
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
      ..Default::default()
    }
  }
}
