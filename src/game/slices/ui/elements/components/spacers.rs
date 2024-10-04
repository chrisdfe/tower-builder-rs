pub mod expand_to_fill {

  use crate::{
    game::slices::ui::{
      elements::{Element, Resizability},
      TwoDimensional,
    },
    types::tree::TreeNodeInput,
  };

  pub fn create_node_input(weight: u32) -> TreeNodeInput<Element> {
    TreeNodeInput(
      Element {
        name: String::from("expand to fill spacer"),

        // background_color: BackgroundColorKind::Fixed(RED),
        resizability: TwoDimensional::same(Resizability::ExpandToFill(weight)),

        ..Default::default()
      },
      vec![],
    )
  }
}
