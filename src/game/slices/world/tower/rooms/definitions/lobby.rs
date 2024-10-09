use macroquad::{
  math::{Rect, Vec2},
  prelude::ImageFormat,
};

use crate::constants::{CELL_HEIGHT, CELL_WIDTH};

use super::*;

const DIMENSIONS: Dimensions = Dimensions {
  width: 1,
  height: 1,
};

pub fn get_definition() -> RoomDefinition {
  RoomDefinition {
    id: RoomDefinitionId::Lobby,
    room_type: RoomType::Lobby,

    dimensions: DIMENSIONS,
    layer: RoomLayer::Default,
    validators: with_base_room_validators(vec![validators::validate_lobby_is_on_correct_floor]),
    price: 1000,
    resizability: RoomResizability::Horizontal,

    render_type: RoomDefinitionRenderType::Texture(
      Texture2D::from_file_with_format(
        include_bytes!("../../../../../../../assets/room_lobby.png"),
        Some(ImageFormat::Png),
      ),
      |texture| {
        // TODO - also return an offset (Point) to calculate into the screen position to render this texture at
        //        since setting the source doesn't also set the center of the new texture
        Some(DrawTextureParams {
          source: Some(Rect {
            x: 0.,
            y: (texture.height() / 3.) * 2.,
            w: texture.width(),
            h: texture.height() / 3.,
          }),
          dest_size: Some(Vec2 {
            x: texture.width() / 2.,
            y: (texture.height() / 2.) / 3.,
            // y: (texture.height() / 2.),
          }),
          ..Default::default()
        })
      },
    ),

    occupancy_limit: 0,
    income: 0,
  }
}
