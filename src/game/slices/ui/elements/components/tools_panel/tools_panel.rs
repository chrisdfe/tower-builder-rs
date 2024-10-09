use macroquad::color::{BLUE, RED};

use crate::{
  game::slices::{
    tools::Tool,
    ui::{
      actions::{ElementAction, ElementActionCreator, ElementActionCreatorCtx},
      components::{line_height_wrapper, panel},
      elements::{
        interactivity::ElementInteractivity, BackgroundColorKind, ContentAlignment, Element,
        ElementTag, TwoDimensional,
      },
      interactivity::ElementInteractivityConfig,
      renderer::generic::ImageElementContentRenderer,
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

        child_gap: 4,
        on_update: Some(update_tool_buttons_wrapper),

        ..Default::default()
      },
      vec![
        (
          "none tool button",
          "None",
          update_none_button as ElementActionCreator,
          on_none_button_click as ElementActionCreator,
        ),
        (
          "build tool button",
          "Build",
          update_build_button as ElementActionCreator,
          on_build_button_click as ElementActionCreator,
        ),
        (
          "destroy tool button",
          "Destroy",
          update_destroy_button as ElementActionCreator,
          on_destroy_button_click as ElementActionCreator,
        ),
      ]
      .into_iter()
      .map(|(name, text, on_update, on_click)| {
        TreeNodeInput(
          Element {
            name: name.to_string(),

            on_update: Some(on_update),
            child_gap: 5,
            padding: 5,

            stack_axis: Axis::Horizontal,
            content_alignment: TwoDimensional::same(ContentAlignment::Center),

            interactivity: Some(ElementInteractivity {
              config: ElementInteractivityConfig {
                on_mouse_up: Some(on_click),
                ..Default::default()
              },
              ..Default::default()
            }),
            ..base_tool_button.clone()
          },
          vec![
            TreeNodeInput(
              Element {
                // TODO - replace with actual icon
                content_renderer: Box::new(ImageElementContentRenderer::new(
                  "assets/icon_pause.png".to_string(),
                )),
                ..Default::default()
              },
              vec![],
            ),
            // TreeNodeInput(
            //   Element {
            //     text: Some(text.to_string()),
            //     // background_color: BackgroundColorKind::Fixed(RED),
            //     ..Default::default()
            //   },
            //   vec![],
            // ),
            line_height_wrapper::create_node_input(text.to_string()),
          ],
          //
          // TreeNodeInput(
          //   Element {
          //     text: Some(text.to_string()),
          //     ..Default::default()
          //   },
          //   vec![],
          // ),
        )
      })
      .collect(),
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
