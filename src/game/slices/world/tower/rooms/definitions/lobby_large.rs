use macroquad::prelude::ImageFormat;

use super::*;

pub fn get_definition() -> RoomDefinition {
  RoomDefinition {
    id: RoomDefinitionId::LobbyLarge,
    room_type: RoomType::Lobby,

    dimensions: Dimensions {
      width: 1,
      height: 3,
    },
    layer: RoomLayer::Default,
    validators: with_base_room_validators(vec![validators::validate_lobby_is_on_correct_floor]),
    price: 1000,
    resizability: RoomResizability::Horizontal,

    render_type: RoomDefinitionRenderType::Texture(
      Texture2D::from_file_with_format(
        include_bytes!("../../../../../../../assets/room_lobby.png"),
        Some(ImageFormat::Png),
      ),
      |_| None,
    ),

    occupancy_limit: 0,
    income: 0,
  }
}
