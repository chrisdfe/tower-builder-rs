use macroquad::color::{BLUE, RED};

use crate::{
  game::slices::{
    tools::Tool,
    ui::{
      elements::{
        interactivity::{Action, ActionCreatorCtx, ElementInteractivity},
        BackgroundColorKind, ContentAlignment, Element, ElementTag, ElementUpdateAction,
        ElementUpdateCtx, TwoDimensional,
      },
      ElementHandle,
    },
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

pub fn create_node_input() -> TreeNodeInput<Element> {
  let base_tool_button: Element = Element {
    padding: 10,
    stack_axis: Axis::Vertical,
    tags: vec![ElementTag::ToolButton],
    ..Default::default()
  };

  TreeNodeInput(
    Element {
      name: String::from("build tool panel"),
      handle: ElementHandle::ToolsPanel,

      child_gap: 10,

      background_color: BackgroundColorKind::Fixed(BLUE),
      stack_axis: Axis::Vertical,

      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Start,
        vertical: ContentAlignment::Center,
      },
      ..base_tool_button.clone()
    },
    vec![TreeNodeInput(
      Element {
        name: String::from("Tool buttons wrapper"),
        handle: ElementHandle::ToolsButtonsWrapper,
        text: Some(String::from("None")),
        ..Default::default()
      },
      vec![
        TreeNodeInput(
          Element {
            name: String::from("none tool button"),
            text: Some(String::from("None")),

            on_update: Some(update_none_button),
            interactivity: Some(ElementInteractivity {
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

            on_update: Some(update_build_button),
            interactivity: Some(ElementInteractivity {
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

            on_update: Some(update_destroy_button),
            interactivity: Some(ElementInteractivity {
              on_mouse_up: Some(on_destroy_button_click),
              ..Default::default()
            }),
            ..base_tool_button.clone()
          },
          Vec::new(),
        ),
      ],
    )],
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

fn update_none_button(ctx: &ElementUpdateCtx, _: &Element) -> ElementUpdateAction {
  if ctx.tools.tool.has_changed() {
    ElementUpdateAction::UpdateActiveState(ctx.tools.tool.current == Tool::None)
  } else {
    ElementUpdateAction::None
  }
}

fn update_build_button(ctx: &ElementUpdateCtx, _: &Element) -> ElementUpdateAction {
  if ctx.tools.tool.has_changed() {
    ElementUpdateAction::UpdateActiveState(ctx.tools.tool.current == Tool::Build)
  } else {
    ElementUpdateAction::None
  }
}

fn update_destroy_button(ctx: &ElementUpdateCtx, _: &Element) -> ElementUpdateAction {
  if ctx.tools.tool.has_changed() {
    ElementUpdateAction::UpdateActiveState(ctx.tools.tool.current == Tool::Destroy)
  } else {
    ElementUpdateAction::None
  }
}
