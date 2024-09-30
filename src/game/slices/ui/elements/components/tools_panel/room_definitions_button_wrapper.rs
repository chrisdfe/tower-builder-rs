use std::{collections::HashMap, str::FromStr};

use macroquad::color::RED;

use crate::{
  game::slices::{
    tools::Tool,
    ui::{
      elements::{ContentAlignment, Element, ElementHandle, ElementTag, TwoDimensional},
      ElementUpdateAction, ElementUpdateCtx,
    },
    world::tower::rooms::definitions::RoomDefinitionId,
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

use crate::{
  game::slices::ui::elements::interactivity::{Action, ActionCreatorCtx, ElementInteractivity},
  game::slices::world::tower::rooms::definitions::ROOM_DEFINITIONS,
};

// TODO - put elsewhere
pub const DEFINITION_DATA_KEY: &str = "definition";

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("room buttons wrapper"),
      handle: ElementHandle::RoomDefinitionButtonsWrapper,
      child_gap: 10,
      stack_axis: Axis::Horizontal,
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

          interactivity: Some(ElementInteractivity {
            on_mouse_up: Some(on_room_definition_button_click),
            ..Default::default()
          }),

          on_update: Some(on_room_definition_button_update),

          attributes: HashMap::from([(DEFINITION_DATA_KEY, format!("{:?}", definition.id))]),

          ..Default::default()
        },
        Vec::new(),
      )
    })
    .collect::<Vec<_>>()
}

fn on_room_definition_button_update(
  ctx: &ElementUpdateCtx,
  element: &Element,
) -> ElementUpdateAction {
  if let Tool::Build = ctx.tools.tool.current {
    if ctx
      .tools
      .build_tool
      .selected_room_definition_id
      .has_changed()
    {
      let definition_id_as_string = format!(
        "{:?}",
        ctx
          .tools
          .build_tool
          .selected_room_definition_id
          .current
      );
      let is_active = definition_id_as_string
        == *element
          .attributes
          .get(DEFINITION_DATA_KEY)
          .unwrap();

      ElementUpdateAction::UpdateActiveState(is_active)
    } else {
      ElementUpdateAction::None
    }
  } else {
    ElementUpdateAction::None
  }
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
  if let Some(definition_as_str) = &element.attributes.get(DEFINITION_DATA_KEY) {
    if let Ok(definition) = RoomDefinitionId::from_str(definition_as_str) {
      Some(definition)
    } else {
      None
    }
  } else {
    None
  }
}
