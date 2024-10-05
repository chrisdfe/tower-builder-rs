use std::f32::consts::PI;

use macroquad::{
  color::{BLACK, BLUE, GREEN, RED, WHITE},
  shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle},
};

use crate::{
  game::{
    slices::{
      ui::{
        elements::{Element, Resizability},
        renderer::ElementContentRenderer,
        BackgroundColorKind, ContentAlignment, ElementCalculatedProperties, ElementUpdateAction,
        ElementUpdateCtx, TwoDimensional, UnwrappedElementCalculatedProperties, UpdateHandler,
      },
      world::time::Time,
    },
    Game,
  },
  types::{
    measurements::{Axis, Dimensions, Point},
    tree::TreeNodeInput,
  },
};

pub fn create_node_input() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("Time panel"),

      padding: 10,
      child_gap: 5,

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
  let mut result = vec![TreeNodeInput(
    Element {
      name: String::from("clock element"),
      padding: 2,
      content_renderer: CLOCK_ELEMENT_RENDERER,

      ..Default::default()
    },
    Vec::new(),
  )];

  result.extend(
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
    .collect::<Vec<_>>(),
  );

  result
}

const CLOCK_ELEMENT_SIZE: u32 = 32;
const CLOCK_ELEMENT_RENDERER: ElementContentRenderer = ElementContentRenderer {
  render: |element: &Element, game: &Game| {
    let Time { hour, minute, .. } = game.world.time.current_time();

    //
    let UnwrappedElementCalculatedProperties { outer_position, .. } = element.calculated.unwrap();
    let center_point = Point {
      x: outer_position.x + (CLOCK_ELEMENT_SIZE as f32 / 2.),
      y: outer_position.y + (CLOCK_ELEMENT_SIZE as f32 / 2.),
    };

    let radius = CLOCK_ELEMENT_SIZE as f32 / 2.;

    // const HOUR_DOT_COUNT: usize = 12;
    // const HOUR_DOT_RADIUS: f32 = 1.;
    // // distance from center in multples of radius (1 = 1x radius, 2 = 2x radius etc)
    // const HOUR_DOT_DISTANCE: f32 = 1.; // 0.8;

    // let angle_increment = 360. / HOUR_DOT_COUNT as f32;

    // for idx in 0..HOUR_DOT_COUNT {
    //   //
    //   let angle = idx as f32 * angle_increment;
    //   let x = center_point.x + radius * (-angle * PI / 180.).cos() * HOUR_DOT_DISTANCE;
    //   let y = center_point.y + radius * (-angle * PI / 180.).sin() * HOUR_DOT_DISTANCE;

    //   draw_circle(x, y, HOUR_DOT_RADIUS, GREEN);
    // }

    // hour hand
    {
      const HOUR_HAND_LENGTH: f32 = 0.5;

      const TOTAL_ANGLE_INCREMENTS: f32 =
        // degrees in a circle
        360. /
        // hours
        12. /
        // minutes
        60.;
      let current_angle_increment = (hour as f32 * 60.) + minute as f32;
      let angle = TOTAL_ANGLE_INCREMENTS * current_angle_increment
      // the default line position is 0 (i.e at 3 o'clock, not 12) so -90 to have the default be
      - 90.;

      let x = center_point.x + radius * (angle * PI / 180.).cos() * HOUR_HAND_LENGTH;
      let y = center_point.y + radius * (angle * PI / 180.).sin() * HOUR_HAND_LENGTH;
      draw_line(center_point.x, center_point.y, x, y, 1., WHITE);
    }

    // minute hand
    {
      const MINUTE_HAND_LENGTH: f32 = 0.8;

      const TOTAL_ANGLE_INCREMENTS: f32 =
        // degrees in a circle
        360. /
        // minutes
        60.;
      let current_angle_increment = minute as f32;
      let angle = TOTAL_ANGLE_INCREMENTS * current_angle_increment
      // the default line position is 0 (i.e at 3 o'clock, not 12) so -90 to have the default be
      - 90.;

      let x = center_point.x + radius * (angle * PI / 180.).cos() * MINUTE_HAND_LENGTH;
      let y = center_point.y + radius * (angle * PI / 180.).sin() * MINUTE_HAND_LENGTH;
      draw_line(center_point.x, center_point.y, x, y, 1., WHITE);
    }

    draw_circle_lines(center_point.x, center_point.y, radius, 0.5, WHITE);
  },
  measure: |_: &Element| Dimensions::square(CLOCK_ELEMENT_SIZE),
};

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
