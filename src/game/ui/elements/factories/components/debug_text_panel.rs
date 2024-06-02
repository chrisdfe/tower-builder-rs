use crate::{
  game::ui::elements::{
    ContentAlignment, Element, ElementConfig, TwoDimensional, UpdateCtx, UpdateHandler,
  },
  measurements::Axis,
  types::tree::TreeNodeInput,
};

pub fn create() -> TreeNodeInput<Element> {
  TreeNodeInput {
    data: Element {
      name: String::from("debug text section"),
      padding: 2,
      stack_axis: Axis::Vertical,
      content_alignment: TwoDimensional {
        horizontal: ContentAlignment::Start,
        vertical: ContentAlignment::Start,
      },
      ..Default::default()
    },
    children: get_children(),
  }
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
  ]
  .into_iter()
  .map(|(text, on_update)| TreeNodeInput {
    data: Element {
      name: String::from(text),
      padding: 2,
      on_update: Some(on_update),
      ..Default::default()
    },
    children: vec![],
  })
  .collect::<Vec<_>>()
}

fn update_text_with_selected_room_definition(ctx: &UpdateCtx, element: &mut Element) {
  element.text = format!(
    "room definition: {:?}",
    ctx.tools.selected_room_definition_id
  );
}

fn update_text_with_funds(ctx: &UpdateCtx, element: &mut Element) {
  element.text = format!("funds: {:?}", ctx.world.wallet.funds);
}

fn update_text_with_population(ctx: &UpdateCtx, element: &mut Element) {
  element.text = format!("population: {:?}", ctx.world.tower.tower.population());
}

fn update_text_with_clock(ctx: &UpdateCtx, element: &mut Element) {
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

  element.text = format!("time: {}:{}", padded_hours, padded_minutes);
}

fn update_text_with_date(ctx: &UpdateCtx, element: &mut Element) {
  let time = ctx.world.time.current_time();

  element.text = format!(
    "date: day {}, month {}, year {}",
    time.day, time.month, time.year
  );
}

fn update_text_with_camera_position(ctx: &UpdateCtx, element: &mut Element) {
  element.text = format!(
    "camera position: {}, {}",
    ctx.camera_position.x, ctx.camera_position.y
  );
}
