use macroquad::color::BLACK;

use crate::{
  game::slices::{
    tools::Tool,
    ui::elements::{
      BackgroundColorKind, ContentAlignment, Element, ElementData, ElementHandle, ElementTag,
      ElementUpdateAction, ElementUpdateCtx, TwoDimensional, UpdateHandler,
    },
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

use super::tools_panel::room_definitions_button_wrapper::DEFINITION_DATA_KEY;

pub fn create_node_input() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("debug text section"),
      handle: ElementHandle::DebugTextPanel,
      padding: 2,
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Start,
        vertical: ContentAlignment::Start,
      },
      background_color: BackgroundColorKind::Fixed(BLACK),
      ..Default::default()
    },
    get_children(),
  )
}

fn get_children() -> Vec<TreeNodeInput<Element>> {
  vec![
    (
      "funds text node", //
      update_text_with_funds as UpdateHandler,
    ),
    (
      "population text node",
      update_text_with_population as UpdateHandler,
    ),
    (
      "clock text node", //
      update_text_with_clock as UpdateHandler,
    ),
    (
      "date text node", //
      update_text_with_date as UpdateHandler,
    ),
    (
      "camera position node",
      update_text_with_camera_position as UpdateHandler,
    ),
    (
      "current tool node",
      update_text_with_current_tool as UpdateHandler,
    ),
    (
      "selected room definition text node",
      update_text_with_selected_room_definition as UpdateHandler,
    ),
    (
      "current hovered room definition button node",
      update_text_with_current_hovered_room_definition_button as UpdateHandler,
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
  ctx: &ElementUpdateCtx,
  _: &Element,
) -> ElementUpdateAction {
  let text = if let Tool::Build = &ctx.tools.tool.current {
    format!(
      "room definition: {:?}",
      ctx.tools.build_tool.selected_room_definition_id
    )
  } else {
    String::from("nothing.")
  };

  ElementUpdateAction::UpdateText(text)
}

fn update_text_with_funds(ctx: &ElementUpdateCtx, _: &Element) -> ElementUpdateAction {
  let text = format!("funds: {:?}", ctx.world.wallet.funds);

  ElementUpdateAction::UpdateText(text)
}

fn update_text_with_population(ctx: &ElementUpdateCtx, _: &Element) -> ElementUpdateAction {
  let text = format!("population: {:?}", ctx.world.tower.tower.population());

  ElementUpdateAction::UpdateText(text)
}

fn update_text_with_clock(ctx: &ElementUpdateCtx, _: &Element) -> ElementUpdateAction {
  let time = ctx.world.time.current_time();
  let padded_hours = if time.hour < 10 {
    format!("0{}", time.hour)
  } else {
    format!("{}", time.hour)
  };

  let padded_minutes = if time.minute < 10 {
    format!("0{}", time.minute)
  } else {
    format!("{}", time.minute)
  };

  ElementUpdateAction::UpdateText(format!("time: {}:{}", padded_hours, padded_minutes))
}

fn update_text_with_date(ctx: &ElementUpdateCtx, _: &Element) -> ElementUpdateAction {
  let time = ctx.world.time.current_time();

  let text = format!(
    "date: day {}, month {}, year {}",
    time.day, time.month, time.year
  );

  ElementUpdateAction::UpdateText(text)
}

fn update_text_with_camera_position(ctx: &ElementUpdateCtx, _: &Element) -> ElementUpdateAction {
  let text = format!(
    "camera position: {}, {}",
    ctx.world.camera.camera_position.x, ctx.world.camera.camera_position.y
  );

  ElementUpdateAction::UpdateText(text)
}

fn update_text_with_current_tool(ctx: &ElementUpdateCtx, element: &Element) -> ElementUpdateAction {
  if element.text == Some(String::from("")) || ctx.tools.tool.has_changed() {
    let text = format!("Tool: {:#?}", ctx.tools.tool.current);
    ElementUpdateAction::UpdateText(text)
  } else {
    ElementUpdateAction::None
  }
}

fn update_text_with_current_hovered_room_definition_button(
  ctx: &ElementUpdateCtx,
  element: &Element,
) -> ElementUpdateAction {
  if let Some(current_hovered_element) = ctx.ui.elements.get_current_hovered_element() {
    if current_hovered_element
      .tags
      .contains(&ElementTag::RoomDefinitionButton)
    {
      if let ElementData::HashMap(hash_map) = &current_hovered_element.data {
        if let Some(definition_data) = hash_map.get(DEFINITION_DATA_KEY) {
          return ElementUpdateAction::UpdateText(String::from(definition_data));
        }
      }
    }
  };

  if let Tool::Build = &ctx.tools.tool.current {
    ElementUpdateAction::UpdateText(format!(
      "building room: {:?}",
      &ctx.tools.build_tool.selected_room_definition_id
    ))
  } else if element.text != Some(String::from("")) {
    ElementUpdateAction::UpdateText(String::from(""))
  } else {
    ElementUpdateAction::None
  }
}
