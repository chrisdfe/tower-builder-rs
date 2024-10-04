use macroquad::color::BLACK;

use crate::{
  game::slices::ui::{
    elements::{Element, Resizability},
    BackgroundColorKind, ContentAlignment, ElementUpdateAction, ElementUpdateCtx, TwoDimensional,
    UpdateHandler,
  },
  types::{measurements::Axis, tree::TreeNodeInput},
};

pub fn create_node_input() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("Time panel"),

      background_color: BackgroundColorKind::Fixed(BLACK),
      stack_axis: Axis::Vertical,
      resizability: TwoDimensional::same(Resizability::ShrinkToFit),
      content_alignment: TwoDimensional::same(ContentAlignment::Start),

      ..Default::default()
    },
    get_children(),
  )
}

fn get_children() -> Vec<TreeNodeInput<Element>> {
  vec![
    (
      "clock text element", //
      update_text_with_clock as UpdateHandler,
    ),
    (
      "date text element", //
      update_text_with_date as UpdateHandler,
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
