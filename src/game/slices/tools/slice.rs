use crate::{
  game::slices::world::tower::rooms::{
    definitions::{RoomDefinitionId, ROOM_DEFINITIONS},
    validation::RoomValidationContext,
    Room,
  },
  types::{map::coordinates_box::CoordinatesBox, PrevAndCurrent},
};

use super::Selection;

pub struct BuildTool {
  pub selected_room_definition_id: PrevAndCurrent<RoomDefinitionId>,
  pub blueprint_room: Room,
}

impl Default for BuildTool {
  fn default() -> Self {
    Self {
      selected_room_definition_id: PrevAndCurrent::new(RoomDefinitionId::Lobby),
      blueprint_room: Room::new(),
    }
  }
}

impl BuildTool {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn set_selected_room_definition(
    &mut self,
    room_definition_id: RoomDefinitionId,
    selection_box: &CoordinatesBox,
    // TODO - rename this
    ctx: RoomValidationContext,
  ) {
    self
      .selected_room_definition_id
      .set_current(room_definition_id);
    let selected_room_definition = ROOM_DEFINITIONS.get(&room_definition_id).unwrap();
    self.blueprint_room = Room::from(selected_room_definition);

    self
      .blueprint_room
      .calculate_coordinates_box(selection_box);

    self.blueprint_room.validate(ctx)
  }

  pub fn tick(&mut self) {
    self.selected_room_definition_id.tick();
  }
}

pub struct DestroyTool;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tool {
  None,
  Build,
  Destroy,
}

pub struct Slice {
  pub selection: Selection,

  pub tool: PrevAndCurrent<Tool>,

  pub build_tool: BuildTool,
  pub destroy_tool: DestroyTool,
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
      tool: PrevAndCurrent::new(Tool::None),
      build_tool: BuildTool::new(),
      destroy_tool: DestroyTool,
      // selected_room_definition_id,
      // blueprint_room: {
      //   let mut room = Room::from(&ROOM_DEFINITIONS[&selected_room_definition_id]);
      //   room.calculate_coordinates_box(&CoordinatesBox::zero());
      //   room
      // },
      selection: Selection::new(),
    }
  }

  pub fn tick(&mut self) {
    self.tool.tick();
    self.build_tool.tick();
  }
}
