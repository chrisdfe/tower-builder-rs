use crate::{
  game::world::tower::rooms::{definitions::ROOM_DEFINITIONS, validation::RoomValidationContext},
  map::Coordinates,
};

use super::{input, timers, tools::Tools, ui::Ui, world};

pub struct Game {
  pub timers: timers::Slice,

  // TODO - serialize
  pub input: input::Slice,
  pub world: world::Slice,
  pub ui: Ui,
  pub tools: Tools,

  pub camera_position: Coordinates,
}

impl Default for Game {
  fn default() -> Self {
    Self {
      input: input::Slice::new(),
      timers: timers::Slice::new(),

      world: world::Slice::new(),
      ui: Ui::new(),
      tools: Tools::new(),

      camera_position: Coordinates::zero(),
    }
  }
}

impl Game {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn initialize(&mut self) {
    // TODO - world.initialize
    self.world.time.register_timers(&mut self.timers);
    self.world.tower.register_timers(&mut self.timers);
  }

  pub fn load(&mut self, loaded: Game) {
    // self.time = loaded.time;
    // self.tower = loaded.tower;
    // self.wallet = loaded.wallet;

    // // TODO - probably unload timers first?
    // self.timers = TimersSlice::new();
    // self.initialize();
  }

  pub fn try_to_build_blueprint_room(&mut self) {
    if self.tools.blueprint_room.is_valid() {
      self.world.tower.tower.build_room(
        ROOM_DEFINITIONS
          .get(&self.tools.selected_room_definition_id)
          .unwrap(),
        &self.ui.selection.selection_box,
      );

      self
        .world
        .wallet
        .subtract_funds(self.tools.blueprint_room.price());

      // Re-validate after build to prevent placing 2 rooms in the same place
      self
        .tools
        .blueprint_room
        .validate(RoomValidationContext {
          tower: &self.world.tower.tower,
          wallet: &self.world.wallet,
        });
    } else {
      // TODO - add status text
      // self.ui.sstatus_text = self
      //   .tools
      //   .blueprint_room
      //   .get_first_validation_error_message()
      //   .to_string();
    }
  }

  pub fn add_camera_position(&mut self, coordinates: Coordinates) {
    self.camera_position.x += coordinates.x;
    self.camera_position.y += coordinates.y;
  }
}
