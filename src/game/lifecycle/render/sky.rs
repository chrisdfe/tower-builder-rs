use macroquad::{
  color::{Color, BLACK, BLUE, RED, WHITE},
  math::FloatExt,
  shapes::draw_rectangle,
  window::{screen_height, screen_width},
};

use crate::game::{slices::world::time::Time, Game};

#[rustfmt::skip]
const SKY_COLORS: [(u8, Color); 5] = [
  // midnight
  (0, BLUE),
  // morning
  (6, WHITE),
  // evening
  (18, RED),
  // begining of night
  (20, BLACK),
  // midnight again
  (24, BLUE)
];

pub fn render(game: &Game) {
  let x = 0.;
  let y = 0.;
  let w = screen_width();
  let h = screen_height();

  let color = {
    let Time {
      hour: current_hour,
      minute: current_minute,
      ..
    } = game.world.time.current_time();

    let mut prev_sky_time_idx = SKY_COLORS.len() - 1;
    let mut next_sky_time_idx = 0;
    for (idx, (sky_hour, _)) in SKY_COLORS.iter().enumerate() {
      //
      if *sky_hour as u32 > current_hour {
        next_sky_time_idx = idx;
        prev_sky_time_idx = if next_sky_time_idx > 0 {
          idx - 1
        } else {
          SKY_COLORS.len() - 1
        };
        break;
      }
    }

    let (prev_hour, prev_color) = SKY_COLORS[prev_sky_time_idx];
    let (next_hour, next_color) = SKY_COLORS[next_sky_time_idx];

    // TODO here - convert prev/next color to hsl for interpolation (or just store them that way in SKY_COLORS)
    // TODO here - normalize current hour/current minute between prev_hour and next_hour
    // let prev_color_hsl = rgb_to_hsl(prev_color);
    // let next_color_hsl = rgb_to_hsl(next_color);

    let total_in_minutes = (next_hour as u32 * 60) - (prev_hour as u32 * 60);
    let progress_in_minutes = ((current_hour * 60) + current_minute) - (prev_hour as u32 * 60);
    let normalized_progress = (progress_in_minutes as f32) / (total_in_minutes as f32);

    // TODO - pull out into fn
    // let current_color_hsl = (
    //   prev_color_hsl
    //     .0
    //     .lerp(next_color_hsl.0, normalized_progress),
    //   prev_color_hsl
    //     .1
    //     .lerp(next_color_hsl.1, normalized_progress),
    //   prev_color_hsl
    //     .2
    //     .lerp(next_color_hsl.2, normalized_progress),
    // );
    let current_color = Color::new(
      prev_color
        .r
        .lerp(next_color.r, normalized_progress),
      prev_color
        .g
        .lerp(next_color.g, normalized_progress),
      prev_color
        .b
        .lerp(next_color.b, normalized_progress),
      1.,
    );

    // hsl_to_rgb(
    //   current_color_hsl.0,
    //   current_color_hsl.1,
    //   current_color_hsl.2,
    // )

    current_color
  };

  draw_rectangle(x, y, w, h, color);
}
