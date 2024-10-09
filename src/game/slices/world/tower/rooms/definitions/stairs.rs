use macroquad::prelude::ImageFormat;

use super::*;

pub fn get_definition() -> RoomDefinition {
  RoomDefinition {
    id: RoomDefinitionId::Stairs,
    room_type: RoomType::Transportation,

    dimensions: Dimensions {
      width: 7,
      height: 2,
    },
    layer: RoomLayer::Transportation,
    validators: vec![
      validators::validate_enough_funds,
      validators::validate_rooms_do_not_overlap,
      validators::validate_transportation_room_is_within_tower_bounds,
    ],
    price: 12000,
    resizability: RoomResizability::None,
    // color: Color::from_hex(0x1111aa),
    render_type: RoomDefinitionRenderType::Image(Texture2D::from_file_with_format(
      include_bytes!("../../../../../../../assets/room_stairs.png"),
      Some(ImageFormat::Png),
    )),
    occupancy_limit: 12,
    income: 1000,
  }
}
