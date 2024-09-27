use macroquad::color::{BLUE, RED};

use crate::{
  game::slices::{
    tools::Tool,
    ui::elements::{
      interactivity::{Action, ActionCreatorCtx, InteractivityConfig},
      BackgroundColorKind, ContentAlignment, Element, ElementData, ElementTag, TwoDimensional,
    },
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

pub fn create_node_input() -> TreeNodeInput<Element> {
  let base_tool_button: Element = Element {
    padding: 10,
    tags: vec![ElementTag::ToolButton],
    ..Default::default()
  };

  TreeNodeInput(
    Element {
      name: String::from("build tool panel"),

      padding: 10,
      child_gap: 10,

      background_color: BackgroundColorKind::Fixed(BLUE),
      stack_axis: Axis::Horizontal,

      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Center,
        vertical: ContentAlignment::Center,
      },
      ..base_tool_button.clone()
    },
    vec![
      TreeNodeInput(
        Element {
          name: String::from("none tool button"),
          text: Some(String::from("None")),
          interactivity: Some(InteractivityConfig {
            on_mouse_up: Some(on_none_button_click),
            ..Default::default()
          }),
          ..base_tool_button.clone()
        },
        Vec::new(),
      ),
      TreeNodeInput(
        Element {
          name: String::from("build tool button"),
          text: Some(String::from("Build")),
          interactivity: Some(InteractivityConfig {
            on_mouse_up: Some(on_build_button_click),
            ..Default::default()
          }),
          ..base_tool_button.clone()
        },
        Vec::new(),
      ),
      TreeNodeInput(
        Element {
          name: String::from("destroy tool button"),
          text: Some(String::from("Destroy")),
          interactivity: Some(InteractivityConfig {
            on_mouse_up: Some(on_destroy_button_click),
            ..Default::default()
          }),
          ..base_tool_button.clone()
        },
        Vec::new(),
      ),
    ],
  )
}

fn on_none_button_click(_: ActionCreatorCtx) -> Action {
  Action::SetCurrentTool(Tool::None)
}

fn on_build_button_click(_: ActionCreatorCtx) -> Action {
  Action::SetCurrentTool(Tool::Build)
}

fn on_destroy_button_click(_: ActionCreatorCtx) -> Action {
  Action::SetCurrentTool(Tool::Destroy)
}
