use macroquad::color::BLACK;

use crate::{
  game::slices::ui::elements::{
    BackgroundColorKind, ContentAlignment, Element, ElementData, ElementHandle, ElementTag,
    ElementUpdateAction, ElementUpdateCtx, Resizability, TwoDimensional, UpdateHandler,
  },
  types::{
    measurements::{Axis, Dimensions},
    tree::TreeNodeInput,
  },
};

use super::tools_panel::DEFINITION_DATA_KEY;

pub fn create() -> TreeNodeInput<Element> {
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
      "selected room definition text node",
      update_text_with_selected_room_definition as UpdateHandler,
    ),
    ("funds text node", update_text_with_funds as UpdateHandler),
    (
      "population text node",
      update_text_with_population as UpdateHandler,
    ),
    ("clock text node", update_text_with_clock as UpdateHandler),
    ("date text node", update_text_with_date as UpdateHandler),
    (
      "camera position node",
      update_text_with_camera_position as UpdateHandler,
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

        // resizability: Resizability::Fixed(Dimensions {
        //   width: 100,
        //   height: 40,
        // }),
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
  let text = format!(
    "room definition: {:?}",
    ctx.tools.selected_room_definition_id
  );

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

fn update_text_with_current_hovered_room_definition_button(
  ctx: &ElementUpdateCtx,
  element: &Element,
) -> ElementUpdateAction {
  if let Some(current_hovered_element) = ctx.ui.elements.get_current_hovered_element() {
    //
    if current_hovered_element
      .tags
      .contains(&ElementTag::RoomDefinitionButton)
    {
      match &current_hovered_element.data {
        ElementData::HashMap(hash_map) => {
          if let Some(definition_data) = hash_map.get(DEFINITION_DATA_KEY) {
            return ElementUpdateAction::UpdateText(String::from(definition_data));
          }
        }
        _ => (),
      }
    }
  };

  // Not hovering over element - clear text if it is not empty
  if element.text.len() > 0 {
    return ElementUpdateAction::UpdateText(String::from(""));
  }

  ElementUpdateAction::None
}
