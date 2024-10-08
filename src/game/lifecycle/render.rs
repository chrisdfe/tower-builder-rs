use macroquad::prelude::*;
use macroquad::{
  color::{Color, WHITE},
  shapes::draw_rectangle,
  text::{draw_text_ex, Font, TextParams},
  window::clear_background,
};
use once_cell::sync::OnceCell;

use crate::game::slices::tools::Tool;
use crate::game::slices::ui::elements::calculated::UnwrappedElementCalculatedProperties;
use crate::game::slices::world::time::Time;
use crate::game::slices::world::tower::Room;
use crate::types::map::Coordinates;
use crate::utils::cell_to_screen_dimensions;
use crate::{
  constants::{CELL_HEIGHT, CELL_WIDTH},
  game::{slices::ui::elements::Element, Game},
  types::measurements::Point,
  utils::coordinates_to_screen_point,
};

// Helpful JS to convert 0-255 rgb to what we need here
// "118, 16, 70, 255".split(',').map(n => parseInt(n, 10)).map(n => n/255).map(n => Math.round(n * 1000) / 1000).join(', ')

// Helpful JS to convert hex to what we need here
// "2d0d03".match(/.{1,2}/g).map(n => parseInt(n, 16)).map(n => n/255).map(n => Math.round(n * 1000) / 1000).join(', ')
pub const BG_COLOR: Color = Color::new(0.075, 0.094, 0.067, 1.);
pub const GROUND_COLOR: Color = Color::new(0.357, 0.055, 0.082, 1.);

pub const DEFAULT_TEXT_COLOR: Color = WHITE;
pub const DEFAULT_FONT_SIZE: u16 = 14;
pub const DEFAULT_FONT_SCALE: f32 = 1.;
pub const DEFAULT_LINE_HEIGHT: u32 = 20;

const DEFAULT_FONT_PATH: &'static str = "./fonts/space-mono/SpaceMono-Regular.ttf";

static UI_FONT: OnceCell<Font> = OnceCell::new();

pub fn get_font<'a>() -> Option<&'a Font> {
  Some(UI_FONT.get().unwrap())
}

pub struct TextSettings<'a> {
  // Realistically this will only ever be Some(font), but macroquad always accepts fonts wrapped in Options in fn arguments
  pub font: Option<&'a Font>,
  pub text_color: Color,
  pub font_size: u16,
  pub font_scale: f32,
}

pub fn get_text_settings<'a>() -> TextSettings<'a> {
  let font = get_font();

  TextSettings {
    font,
    text_color: DEFAULT_TEXT_COLOR,
    font_size: DEFAULT_FONT_SIZE,
    font_scale: DEFAULT_FONT_SCALE,
  }
}

pub async fn initialize() {
  let font = load_ttf_font(DEFAULT_FONT_PATH).await.unwrap();
  UI_FONT.set(font).unwrap();
}

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
  clear_background(BG_COLOR);

  // World
  draw_sky(game);
  draw_ground(game);
  draw_rooms(game);
  draw_current_tool(game);
  draw_room_blueprint(game);

  // UI
  draw_ui(game);
}

fn draw_sky(game: &Game) {
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

fn draw_ground(game: &Game) {
  let x = 0.;
  let y = coordinates_to_screen_point(&Coordinates::zero(), game).y;
  let w = screen_width();

  let h = screen_height() - y;
  let color = GROUND_COLOR;
  draw_rectangle(x, y, w, h, color);
}

fn draw_rooms(game: &Game) {
  for room in game.world.tower.tower.rooms.iter() {
    draw_room(&room, None, game);
  }
}

fn draw_room(room: &Room, color_override: Option<Color>, game: &Game) {
  // TODO - 'cell_to_screen_dimensions'
  let w = room.coordinates_box.dimensions().width as f32 * (CELL_WIDTH as f32);
  let h = room.coordinates_box.dimensions().height as f32 * (CELL_HEIGHT as f32) * -1.;

  let Point { x, y } =
    coordinates_to_screen_point(&room.coordinates_box.bottom_left_coordinates(), game);

  let room_is_connected = game
    .world
    .tower
    .tower
    .room_is_connected_to_lobby(room);

  let color = if let Some(color) = color_override {
    color
  } else {
    let mut color = room.definition().color.clone();
    color.a = if room_is_connected { 1. } else { 0.5 };
    color
  };

  let occupant_count = game
    .world
    .tower
    .tower
    .get_room_occupant_count(room);

  draw_rectangle(x, y, w, h, color);
  render_text_custom(&format!("{}", occupant_count), &Point { x, y })
}

fn draw_current_tool(game: &Game) {
  if game.ui.mouse_is_over_ui() {
    return;
  }

  match &game.tools.tool.current {
    Tool::Build => draw_build_tool(game),
    Tool::Destroy => draw_destroy_tool(game),
    Tool::None => {
      // noop (for now)
    }
  }
}

fn draw_build_tool(game: &Game) {
  draw_room_blueprint(game);
}

fn draw_room_blueprint(game: &Game) {
  if game.ui.mouse_is_over_ui() {
    return;
  }

  if let Tool::Build = &game.tools.tool.current {
    let mut color = if game.tools.build_tool.blueprint_room.is_valid() {
      game
        .tools
        .build_tool
        .blueprint_room
        .definition()
        .color
    } else {
      RED.clone()
    };

    color.a = 0.4;

    draw_room(&game.tools.build_tool.blueprint_room, Some(color), game);
  }
}

fn draw_destroy_tool(game: &Game) {
  let current_cell = &game.tools.selection.current_selected_cell;

  let Rect { w, h, x, y } = cell_to_screen_dimensions(&current_cell, game);

  // TODO - if current_cell is over a room, highlight the whole room
  let color = {
    let mut color = RED.clone();
    color.a = 0.5;
    color
  };

  draw_rectangle(x, y, w, h, color);
}

fn draw_ui(game: &Game) {
  // Draw from the top down so child nodes will be rendered on top of parent nodes
  let ids = game
    .ui
    .elements
    .tree
    .get_node_ids_grouped_by_depth_top_down_flat();
  let sorted_nodes = game.ui.elements.tree.find_nodes_by_ids(&ids);

  for node in sorted_nodes {
    draw_element(&node.data, &game);
  }
}

fn draw_element(element: &Element, game: &Game) {
  let UnwrappedElementCalculatedProperties {
    outer_position,
    outer_dimensions,
    content_position,
    content_dimensions,
    background_color,
    ..
  } = element.calculated.unwrap();

  // Draw outer (background)
  {
    let x = outer_position.x;
    let y = outer_position.y;
    let w = outer_dimensions.width as f32;
    let h = outer_dimensions.height as f32;

    draw_rectangle(x, y, w, h, *background_color);
  };

  // Draw content (background)
  {
    let x = content_position.x;
    let y = content_position.y;
    let w = content_dimensions.width as f32;
    let h = content_dimensions.height as f32;
    // let debug_content_background_color = Color::from_rgba(255, 255, 255, 100);
    let debug_content_background_color = Color::new(0., 0., 0., 0.);

    draw_rectangle(x, y, w, h, debug_content_background_color);
  }

  // Render leaf node content
  (element.content_renderer.render)(&element, &game);
}

fn render_text_custom(text: &String, point: &Point) {
  let font = Some(UI_FONT.get().unwrap());

  draw_text_ex(
    text,
    point.x,
    point.y,
    TextParams {
      font,
      font_size: DEFAULT_FONT_SIZE,
      color: DEFAULT_TEXT_COLOR,
      font_scale: DEFAULT_FONT_SCALE,
      ..Default::default()
    },
  );
}
