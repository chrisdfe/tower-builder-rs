use macroquad::color::BLUE;

use crate::{
  game::slices::{
    tools::Tool,
    ui::{
      actions::{ElementAction, ElementActionCreatorCtx},
      components::panel,
      elements::{
        interactivity::ElementInteractivity, BackgroundColorKind, ContentAlignment, Element,
        ElementTag, TwoDimensional,
      },
      interactivity::ElementInteractivityConfig,
    },
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

use super::room_definition_buttons::{self, ROOM_BUTTONS_WRAPPER_HANDLE};

pub const TOOLS_PANEL_HANDLE: &'static str = "tools panel";
pub const TOOLS_BUTTONS_WRAPER_HANDLE: &'static str = "tools buttons wrapper";

pub fn create_node_input() -> TreeNodeInput<Element> {
  let base_tool_button: Element = Element {
    padding: 10,
    stack_axis: Axis::Vertical,
    tags: vec![ElementTag::ToolButton],
    ..Default::default()
  };

  TreeNodeInput(
    Element {
      name: String::from(TOOLS_PANEL_HANDLE),
      handle: TOOLS_PANEL_HANDLE,

      stack_axis: Axis::Vertical,

      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Start,
        vertical: ContentAlignment::Center,
      },
      ..panel::create_base_element()
    },
    vec![TreeNodeInput(
      Element {
        name: String::from(TOOLS_BUTTONS_WRAPER_HANDLE),
        handle: TOOLS_BUTTONS_WRAPER_HANDLE,

        on_update: Some(update_tool_buttons_wrapper),

        ..Default::default()
      },
      vec![
        TreeNodeInput(
          Element {
            name: String::from("none tool button"),
            text: Some(String::from("None")),

            on_update: Some(update_none_button),

            interactivity: Some(ElementInteractivity {
              config: ElementInteractivityConfig {
                on_mouse_up: Some(on_none_button_click),
                ..Default::default()
              },
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
              config: ElementInteractivityConfig {
                on_mouse_up: Some(on_build_button_click),
                ..Default::default()
              },
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
              config: ElementInteractivityConfig {
                on_mouse_up: Some(on_destroy_button_click),
                ..Default::default()
              },
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

fn on_none_button_click(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::SetCurrentTool(Tool::None)
}

fn on_build_button_click(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::SetCurrentTool(Tool::Build)
}

fn on_destroy_button_click(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::SetCurrentTool(Tool::Destroy)
}

fn update_tool_buttons_wrapper(ctx: ElementActionCreatorCtx, _element: &Element) -> ElementAction {
  // Add/remove room definitions buttons
  if ctx.tools.tool.has_changed() {
    if ctx.tools.tool.current == Tool::Build {
      let parent_id = ctx
        .ui
        .elements
        .find_node_id_by_handle(TOOLS_PANEL_HANDLE);

      ElementAction::PrependChild(room_definition_buttons::create(), parent_id)
    } else {
      ElementAction::RemoveNodeByHandle(ROOM_BUTTONS_WRAPPER_HANDLE)
    }
  } else {
    ElementAction::None
  }
}

fn update_none_button(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  if ctx.tools.tool.has_changed() {
    ElementAction::UpdateActiveState(ctx.tools.tool.current == Tool::None)
  } else {
    ElementAction::None
  }
}

fn update_build_button(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  if ctx.tools.tool.has_changed() {
    ElementAction::UpdateActiveState(ctx.tools.tool.current == Tool::Build)
  } else {
    ElementAction::None
  }
}

fn update_destroy_button(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  if ctx.tools.tool.has_changed() {
    ElementAction::UpdateActiveState(ctx.tools.tool.current == Tool::Destroy)
  } else {
    ElementAction::None
  }
}
