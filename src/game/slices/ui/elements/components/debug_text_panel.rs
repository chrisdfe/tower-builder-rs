use crate::{
  game::slices::{
    tools::Tool,
    ui::{
      actions::{ElementAction, ElementActionCreator, ElementActionCreatorCtx},
      elements::{ContentAlignment, Element, ElementTag, TwoDimensional},
    },
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

use super::{panel, tools_panel::room_definition_buttons::DEFINITION_DATA_KEY};

const DEBUG_TEXT_PANEL_HANDLE: &'static str = "debug text panel";

pub fn create_node_input() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("debug text section"),
      handle: DEBUG_TEXT_PANEL_HANDLE,
      //
      child_gap: 2,
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Start,
        vertical: ContentAlignment::Start,
      },
      ..panel::create_base_element()
    },
    get_children(),
  )
}

fn get_children() -> Vec<TreeNodeInput<Element>> {
  vec![
    (
      "funds text element", //
      update_text_with_funds as ElementActionCreator,
    ),
    (
      "population text element",
      update_text_with_population as ElementActionCreator,
    ),
    (
      "camera position element",
      update_text_with_camera_position as ElementActionCreator,
    ),
    (
      "current tool element",
      update_text_with_current_tool as ElementActionCreator,
    ),
    (
      "current selected cell element",
      update_text_with_current_selected_cell as ElementActionCreator,
    ),
    (
      "selected room definition text element",
      update_text_with_selected_room_definition as ElementActionCreator,
    ),
    (
      "current hovered room definition button element",
      update_text_with_current_hovered_room_definition_button as ElementActionCreator,
    ),
    (
      "status text element",
      update_status_text_element as ElementActionCreator,
    ),
  ]
  .into_iter()
  .map(|(text, on_update)| {
    TreeNodeInput(
      Element {
        name: String::from(text),
        padding: 2,
        on_update: Some(on_update),

        ..Default::default()
      },
      Vec::new(),
    )
  })
  .collect::<Vec<_>>()
}

fn update_text_with_selected_room_definition(
  ctx: ElementActionCreatorCtx,
  _: &Element,
) -> ElementAction {
  let text = if let Tool::Build = &ctx.tools.tool.current {
    format!(
      "room definition: {:?}",
      ctx
        .tools
        .build_tool
        .selected_room_definition_id
        .current
    )
  } else {
    String::from("nothing.")
  };

  ElementAction::UpdateText(text)
}

fn update_text_with_funds(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  let text = format!("funds: {:?}", ctx.world.wallet.funds);

  ElementAction::UpdateText(text)
}

fn update_text_with_population(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  let text = format!("population: {:?}", ctx.world.tower.tower.population());

  ElementAction::UpdateText(text)
}

fn update_text_with_camera_position(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  let text = format!(
    "camera position: {}, {}",
    ctx.world.camera.camera_position.x, ctx.world.camera.camera_position.y
  );

  ElementAction::UpdateText(text)
}

fn update_text_with_current_tool(ctx: ElementActionCreatorCtx, element: &Element) -> ElementAction {
  if element.text == Some(String::from("")) || ctx.tools.tool.has_changed() {
    let text = format!("Tool: {:#?}", ctx.tools.tool.current);
    ElementAction::UpdateText(text)
  } else {
    ElementAction::None
  }
}

fn update_text_with_current_selected_cell(
  ctx: ElementActionCreatorCtx,
  _: &Element,
) -> ElementAction {
  // TODO - has_changed
  if ctx.tools.selection.selected_cell_has_changed() {
    let text = format!(
      "Current selected cell: {:?}, {:?}",
      ctx.tools.selection.current_selected_cell.x, ctx.tools.selection.current_selected_cell.y,
    );
    ElementAction::UpdateText(text)
  } else {
    ElementAction::None
  }
}

fn update_text_with_current_hovered_room_definition_button(
  ctx: ElementActionCreatorCtx,
  element: &Element,
) -> ElementAction {
  if let Some(current_hovered_element) = ctx.ui.elements.get_current_hovered_element() {
    if current_hovered_element
      .tags
      .contains(&ElementTag::RoomDefinitionButton)
    {
      if let Some(definition_data) = &current_hovered_element
        .attributes
        .get(DEFINITION_DATA_KEY)
      {
        return ElementAction::UpdateText((definition_data).to_string());
      }
    }
  };

  if let Tool::Build = &ctx.tools.tool.current {
    ElementAction::UpdateText(format!(
      "building room: {:?}",
      &ctx
        .tools
        .build_tool
        .selected_room_definition_id
        .current
    ))
  } else if element.text != Some(String::from("")) {
    ElementAction::UpdateText(String::from(""))
  } else {
    ElementAction::None
  }
}

fn update_status_text_element(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  //
  if ctx.ui.state.status_text.has_changed() {
    ElementAction::UpdateText(ctx.ui.state.status_text.current.clone())
  } else {
    ElementAction::None
  }
}
