use super::ElementConfig;
// use uuid::Uuid;

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
