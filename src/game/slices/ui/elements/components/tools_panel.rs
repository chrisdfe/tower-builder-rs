use std::{collections::HashMap, str::FromStr};

use macroquad::color::RED;

use crate::{
  game::slices::{
    ui::elements::{
      BackgroundColorKind, ContentAlignment, Element, ElementData, ElementHandle, ElementTag,
      TwoDimensional,
    },
    world::tower::rooms::definitions::RoomDefinitionId,
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

use crate::{
  game::slices::ui::elements::interactivity::{Action, ActionCreatorCtx, InteractivityConfig},
  game::slices::world::tower::rooms::definitions::ROOM_DEFINITIONS,
};

pub const DEFINITION_DATA_KEY: &str = "definition";

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("tools panel"),
      handle: ElementHandle::ToolsPanel,

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
    vec![create_wrapper()],
  )
}

pub fn create_wrapper() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("room buttons wrapper"),
      padding: 10,
      child_gap: 10,
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },
      ..Default::default()
    },
    create_buttons(),
  )
}

fn create_buttons() -> Vec<TreeNodeInput<Element>> {
  ROOM_DEFINITIONS
    .iter()
    .map(|(_definition_id, definition)| {
      TreeNodeInput(
        Element {
          name: String::from(format!("room definition button: {:?}", definition.id)),

          text: Some(String::from(format!("{:?}", definition.id))),

          padding: 10,

          tags: vec![ElementTag::RoomDefinitionButton],

          // resizability: Resizability::Fixed(Dimensions {
          //   width: 20,
          //   height: 20,
          // }),
          interactivity: Some(InteractivityConfig {
            on_mouse_up: Some(on_room_definition_button_click),
            ..Default::default()
          }),

          data: ElementData::HashMap(HashMap::from([(
            DEFINITION_DATA_KEY,
            format!("{:?}", definition.id),
          )])),

          ..Default::default()
        },
        Vec::new(),
      )
    })
    .collect::<Vec<_>>()
}

fn on_room_definition_button_click(ctx: ActionCreatorCtx) -> Action {
  let ActionCreatorCtx { element } = ctx;

  if let Some(definition) = get_definition_from_button(&element) {
    Action::SetSelectedRoomDefinition(definition)
  } else {
    println!(
      "Room definition button {:?} has invalid definition",
      element.name,
    );
    Action::None
  }
}

fn get_definition_from_button(element: &Element) -> Option<RoomDefinitionId> {
  if let ElementData::HashMap(h) = &element.data {
    if let Some(definition_as_str) = h.get(DEFINITION_DATA_KEY) {
      if let Ok(definition) = RoomDefinitionId::from_str(definition_as_str) {
        return Some(definition);
      }
    }
  }

  None
}
