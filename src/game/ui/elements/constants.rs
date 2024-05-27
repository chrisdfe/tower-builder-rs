use lazy_static::lazy_static;
use macroquad::color::Color;

use crate::tower::rooms::definitions::RoomDefinitionId;

use super::{
  interactivity::{Action, ElementEventHandlers, InteractivityConfig},
  BackgroundColorKind, ElementConfig, ElementInput,
};

lazy_static! {
  pub static ref ROOM_DEFINITION_BUTTONS: Vec<ElementInput> = vec![
    create_room_definition_element(RoomDefinitionId::Lobby, String::from("Lobby")),
    // create_room_definition_element(RoomDefinitionId::LobbyLarge, String::from("Large Lobby")),
    // create_room_definition_element(RoomDefinitionId::Office, String::from("Office")),
    create_room_definition_element(RoomDefinitionId::Condo, String::from("Condo")),
  ];

  pub static ref BUTTON_BACKGROUND_COLOR_BASE: Color = Color::new(1., 0., 0., 1.);
  pub static ref BUTTON_BACKGROUND_COLOR_OVER: Color = Color::new(0., 1., 0., 1.);
  pub static ref BUTTON_BACKGROUND_COLOR_DOWN: Color = Color::new(0., 0., 1., 1.);
}

// fn set_room_definition(room_definition_id: RoomDefinitionId, ctx: ButtonEventHandlerContext) {
//   ctx.tools.set_selected_room_definition(
//     room_definition_id,
//     &ctx.selection_box,
//     RoomValidationContext {
//       tower: &ctx.world.tower.tower,
//       wallet: &ctx.world.wallet,
//     },
//   );
// }

// TODO - defaults for a button
fn create_room_definition_element(
  room_definition_id: RoomDefinitionId,
  text: String,
) -> ElementInput {
  let name = format!("Room definition btn: {}", &text.clone());

  ElementInput {
    text,
    name,
    config: ElementConfig {
      padding: 10,
      background_color: BackgroundColorKind::Fixed(*BUTTON_BACKGROUND_COLOR_BASE),
      interactivity: Some(InteractivityConfig {
        background_color_down: BackgroundColorKind::Fixed(*BUTTON_BACKGROUND_COLOR_DOWN),
        background_color_over: BackgroundColorKind::Fixed(*BUTTON_BACKGROUND_COLOR_OVER),
        event_handlers: ElementEventHandlers {
          // on_mouse_up: Some(|_| Action::PrintDebugStatement),
          on_mouse_up: Some(|_| Action::RemoveAllRootNodeChildren),
          ..Default::default()
        },
        ..Default::default()
      }),
      ..ElementConfig::default()
    },
    ..Default::default()
  }
}
