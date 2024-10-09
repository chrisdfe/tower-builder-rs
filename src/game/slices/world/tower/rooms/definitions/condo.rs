use macroquad::prelude::ImageFormat;

use super::*;

pub fn get_definition() -> RoomDefinition {
  RoomDefinition {
    id: RoomDefinitionId::Condo,
    room_type: RoomType::Home,
    dimensions: Dimensions {
      width: 15,
      height: 1,
    },
    layer: RoomLayer::Default,
    validators: with_base_room_validators(vec![
      validators::validate_room_is_above_ground,
      validators::validate_non_lobby_is_not_on_ground_floor,
    ]),
    price: 4000,
    resizability: RoomResizability::None,

    render_type: RoomDefinitionRenderType::Texture(
      Texture2D::from_file_with_format(
        include_bytes!("../../../../../../../assets/room_condo.png"),
        Some(ImageFormat::Png),
      ),
      |_| None,
    ),

    occupancy_limit: 5,
    income: 10000,
  }
}
