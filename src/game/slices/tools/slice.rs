use crate::{
  game::slices::world::tower::rooms::{
    definitions::{RoomDefinitionId, ROOM_DEFINITIONS},
    validation::RoomValidationContext,
    Room,
  },
  types::map::coordinates_box::CoordinatesBox,
};

use super::Selection;

pub struct Slice {
  pub selected_room_definition_id: RoomDefinitionId,

  pub selection: Selection,

  pub blueprint_room: Room,
}

impl Default for Slice {
  fn default() -> Self {
    Slice::new()
  }
}

impl Slice {
  pub fn new() -> Self {
    let selected_room_definition_id = RoomDefinitionId::Lobby;

    Self {
      selected_room_definition_id,
      blueprint_room: {
        let mut room = Room::from(&ROOM_DEFINITIONS[&selected_room_definition_id]);
        room.calculate_coordinates_box(&CoordinatesBox::zero());
        room
      },
      selection: Selection::new(),
    }
  }

  pub fn set_selected_room_definition(
    &mut self,
    room_definition_id: RoomDefinitionId,
    selection_box: &CoordinatesBox,
    // TODO - rename this
    ctx: RoomValidationContext,
  ) {
    self.selected_room_definition_id = room_definition_id;
    let selected_room_definition = ROOM_DEFINITIONS.get(&room_definition_id).unwrap();
    self.blueprint_room = Room::from(selected_room_definition);

    println!(
      "self.blueprint_room definition: {:?}",
      self.blueprint_room.definition()
    );

    self
      .blueprint_room
      .calculate_coordinates_box(selection_box);

    self.blueprint_room.validate(ctx)
  }
}
