pub mod expand_to_fill {

  use crate::{
    game::slices::ui::elements::{Element, Resizability},
    types::tree::TreeNodeInput,
  };

  pub fn create_node_input() -> TreeNodeInput<Element> {
    TreeNodeInput(
      Element {
        name: String::from("expand to fill spacer"),

        // background_color: BackgroundColorKind::Fixed(RED),
        resizability: Resizability::ExpandToFill(1),

        ..Default::default()
      },
      vec![],
    )
  }
}
