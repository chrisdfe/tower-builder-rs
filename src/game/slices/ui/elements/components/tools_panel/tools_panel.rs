use macroquad::color::RED;

use crate::{
  game::slices::ui::elements::{
    BackgroundColorKind, ContentAlignment, Element, ElementData, ElementTag, TwoDimensional,
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("build tool panel"),

      padding: 10,
      child_gap: 10,

      background_color: BackgroundColorKind::Fixed(RED),
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },
      ..Default::default()
    },
    vec![TreeNodeInput(
      Element {
        name: String::from("build tool button"),

        text: Some(String::from("Build")),

        padding: 10,

        tags: vec![ElementTag::RoomDefinitionButton],

        // interactivity: Some(InteractivityConfig {
        //   on_mouse_up: Some(on_room_definition_button_click),
        //   ..Default::default()
        // }),
        // data: ElementData::HashMap(HashMap::from([(
        //   DEFINITION_DATA_KEY,
        //   format!("{:?}", definition.id),
        // )])),
        ..Default::default()
      },
      Vec::new(),
    )],
  )
}
