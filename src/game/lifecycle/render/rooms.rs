use macroquad::{color::Color, shapes::draw_rectangle};

use crate::{
  constants::{CELL_HEIGHT, CELL_WIDTH},
  game::{
    slices::world::tower::{rooms::definitions::RoomDefinitionRenderType, Room},
    Game,
  },
  types::measurements::Point,
  utils::coordinates_to_screen_point,
};

pub(super) fn render(game: &Game) {
  for room in game.world.tower.tower.rooms.iter() {
    render_room(&room, None, game);
  }
}

pub(super) fn render_room(room: &Room, color_override: Option<Color>, game: &Game) {
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

  // BG color/image
  match &room.definition().render_type {
    RoomDefinitionRenderType::Color(color) => {
      let color = if let Some(color) = color_override {
        color
      } else {
        let mut color = color.clone();
        color.a = if room_is_connected { 1. } else { 0.5 };
        color
      };

      draw_rectangle(x, y, w, h, color);
    }
    RoomDefinitionRenderType::Image(image) => {
      // TODO
    }
  }

  // occupant count text
  let occupant_count = game
    .world
    .tower
    .tower
    .get_room_occupant_count(room);

  super::text::render_text_custom(&format!("{}", occupant_count), &Point { x, y })
}
