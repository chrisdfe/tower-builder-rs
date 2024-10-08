use crate::{
  game::slices::ui::{Element, Resizability, TwoDimensional},
  types::tree::TreeNodeInput,
};

// TODO - put this in theme, not here
pub const LINE_HEIGHT: u32 = 20;

pub fn create_node_input(text: String) -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      resizability: TwoDimensional {
        horizontal: Resizability::ShrinkToFit,
        vertical: Resizability::Fixed(20),
      },
      ..Default::default()
    },
    vec![TreeNodeInput(
      Element {
        text: Some(text),
        ..Default::default()
      },
      vec![],
    )],
  )
}
