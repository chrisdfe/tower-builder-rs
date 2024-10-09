use macroquad::{
  color::{Color, BLACK, BLUE, RED, WHITE},
  math::{FloatExt, Rect},
  shapes::draw_rectangle,
  window::{screen_height, screen_width},
};

use crate::game::{slices::world::time::Time, Game};

const NIGHT_COLOR: Color = Color::new(32. / 255., 19. / 255., 59. / 255., 1.);

// time of day (hour), Color, foreground alpha
#[rustfmt::skip]
const SKY_COLORS: [(u8, Color, f32); 6] = [
  // midnight
  (0, NIGHT_COLOR, 0.6),
  // morning
  (6, WHITE, 0.05),
  // afternoon
  (14, WHITE, 0.05),
  // evening
  (18, RED, 0.4),
  // begining of night
  (20, NIGHT_COLOR, 0.4),
  // midnight again
  (24, NIGHT_COLOR, 0.7)
];

pub fn render_background(game: &Game) {
  let Rect { x, y, w, h } = get_rect();
  let color = get_color(game, false);

  draw_rectangle(x, y, w, h, color);
}

pub fn render_foreground(game: &Game) {
  //

  let Rect { x, y, w, h } = get_rect();
  let color = get_color(game, true);
  draw_rectangle(x, y, w, h, color);
}

fn get_rect() -> Rect {
  let x = 0.;
  let y = 0.;
  let w = screen_width();
  let h = screen_height();

  Rect { x, y, w, h }
}

fn get_color(game: &Game, use_alpha: bool) -> Color {
  let Time {
    hour: current_hour,
    minute: current_minute,
    ..
  } = game.world.time.current_time();

  let mut prev_sky_time_idx = SKY_COLORS.len() - 1;
  let mut next_sky_time_idx = 0;
  for (idx, (sky_hour, _, _)) in SKY_COLORS.iter().enumerate() {
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

  let (prev_hour, prev_color, prev_alpha) = SKY_COLORS[prev_sky_time_idx];
  let (next_hour, next_color, next_alpha) = SKY_COLORS[next_sky_time_idx];

  let total_in_minutes = (next_hour as u32 * 60) - (prev_hour as u32 * 60);
  let progress_in_minutes = ((current_hour * 60) + current_minute) - (prev_hour as u32 * 60);
  let normalized_progress = (progress_in_minutes as f32) / (total_in_minutes as f32);

  // TODO - pull out into fn
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
    if use_alpha {
      prev_alpha.lerp(next_alpha, normalized_progress)
    } else {
      1.
    },
  );

  current_color
}
