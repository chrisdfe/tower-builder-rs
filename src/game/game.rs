use crate::{
  game::slices::world::tower::rooms::{
    definitions::ROOM_DEFINITIONS, validation::RoomValidationContext,
  },
  types::map::Coordinates,
};

use super::slices::{
  input, timers,
  tools::{self, Tool},
  ui, world,
};

// TODO - serialize
pub struct Game {
  pub timers: timers::Slice,

  pub input: input::Slice,
  pub world: world::Slice,
  pub ui: ui::Slice,
  pub tools: tools::Slice,
}

impl Default for Game {
  fn default() -> Self {
    Self {
      input: input::Slice::new(),
      timers: timers::Slice::new(),

      world: world::Slice::new(),
      ui: ui::Slice::new(),
      tools: tools::Slice::new(),
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
    if let Tool::Build(build_tool) = &mut self.tools.tool {
      if build_tool.blueprint_room.is_valid() {
        self.world.tower.tower.build_room(
          ROOM_DEFINITIONS
            .get(&mut build_tool.selected_room_definition_id)
            .unwrap(),
          &self.tools.selection.selection_box(),
        );

        self
          .world
          .wallet
          .subtract_funds(build_tool.blueprint_room.price());

        // Re-validate after build to prevent placing 2 rooms in the same place
        build_tool
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

      // Reset selection box
      self
        .tools
        .selection
        .start_selection_box_at_current_cell();

      // reset blueprint
      build_tool
        .blueprint_room
        .calculate_coordinates_box(&self.tools.selection.selection_box());
    }
  }
}
