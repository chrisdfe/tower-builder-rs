use uuid::Uuid;

use crate::{
  game::{
    slices::{
      tools::{self, Tool},
      ui::{self, components::tools_panel::tools_panel::TOOLS_PANEL_HANDLE},
      world::{
        self,
        time::slice::TimeSpeed,
        tower::rooms::{definitions::RoomDefinitionId, validation::RoomValidationContext},
      },
    },
    Game,
  },
  types::{
    map::CoordinatesBox,
    tree::{TreeNode, TreeNodeInput},
  },
};

use super::Element;

// TODO - this being here couples game logic with UI framework logic, I should probably move it
pub enum ElementAction {
  None,
  PrintDebugStatement,
  SetSelectedRoomDefinition(RoomDefinitionId),
  SetCurrentTool(Tool),
  UpdateText(String),
  UpdateActiveState(bool),
  UpdateTimeSpeed(TimeSpeed),
  AppendChild(TreeNodeInput<Element>, Option<Uuid>),
  PrependChild(TreeNodeInput<Element>, Option<Uuid>),
  RemoveNodeByHandle(&'static str),
}

pub type ElementActionCreator =
  fn(ctx: ElementActionCreatorCtx, element: &Element) -> ElementAction;

// TODO - this is a bit of an encapsulation issue
// Maybe a generic instead or something?
pub struct ElementActionCreatorCtx<'a> {
  pub world: &'a world::Slice,
  pub tools: &'a tools::Slice,
  pub ui: &'a ui::Slice,
}

pub fn run_action_creator(
  game: &Game,
  element: &Element,
  action_creator: ElementActionCreator,
) -> ElementAction {
  let ctx = ElementActionCreatorCtx {
    world: &game.world,
    tools: &game.tools,
    ui: &game.ui,
  };

  action_creator(ctx, element)
}

pub fn perform_element_mutation(game: &mut Game, node_id: Uuid, action: ElementAction) {
  use ElementAction::*;
  match action {
    None => {
      // no-op
    }
    PrintDebugStatement => {
      println!("debug statement. {}", node_id);
    }
    SetSelectedRoomDefinition(room_definition_id) => {
      if let Tool::Build = &mut game.tools.tool.current {
        game
          .tools
          .build_tool
          .set_selected_room_definition(
            room_definition_id,
            // TODO - work this out

            // Is this even neccessary???
            // game.tools.selection_box,
            &CoordinatesBox::at_coords(&game.tools.selection.current_selected_cell),
            RoomValidationContext {
              tower: &game.world.tower.tower,
              wallet: &game.world.wallet,
            },
          );
      }
    }
    SetCurrentTool(tool) => {
      game.tools.tool.set_current(tool);

      let parent_id = {
        if let Some(tools_panel_element) = game
          .ui
          .elements
          .find_node_by_handle(TOOLS_PANEL_HANDLE)
        {
          Some(tools_panel_element.id)
        } else {
          Option::None
        }
      };
    }
    UpdateText(text) => {
      let element = game
        .ui
        .elements
        .tree
        .find_node_by_id_mut(node_id)
        .unwrap();

      element.data.text = Some(text);
    }
    UpdateActiveState(is_active) => {
      let element = game
        .ui
        .elements
        .tree
        .find_node_by_id_mut(node_id)
        .unwrap();

      if let Some(interactivity) = &mut element.data.interactivity {
        interactivity
          .state
          .is_active
          .set_current(is_active);
      }
    }
    AppendChild(input, parent_id) => {
      game
        .ui
        .elements
        .tree
        .append_node(input, parent_id);
    }
    PrependChild(input, parent_id) => {
      game
        .ui
        .elements
        .tree
        .prepend_node(input, parent_id);
    }

    RemoveNodeByHandle(handle) => {
      game.ui.elements.remove_node_by_handle(handle);
    }

    UpdateTimeSpeed(speed) => {
      game.world.time.set_speed(speed);
    }
  }
}
