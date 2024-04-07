use lazy_static::lazy_static;

use crate::tower::rooms::definitions::RoomDefinitionId;

use super::{
  interactivity::{Action, ElementEventHandlers},
  ElementConfig, ElementInput,
};
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
      event_handlers: ElementEventHandlers {
        on_mouse_up: Some(|_| Action::PrintDebugStatement),
        ..Default::default()
      },
      ..ElementConfig::default()
    },
    ..Default::default()
  }
}

lazy_static! {
  pub static ref ROOM_DEFINITION_BUTTONS: Vec<ElementInput> = vec![
    create_room_definition_element(RoomDefinitionId::Lobby, String::from("Lobby")),
    // create_room_definition_element(RoomDefinitionId::LobbyLarge, String::from("Large Lobby")),
    // create_room_definition_element(RoomDefinitionId::Office, String::from("Office")),
    create_room_definition_element(RoomDefinitionId::Condo, String::from("Condo")),
  ];
}
