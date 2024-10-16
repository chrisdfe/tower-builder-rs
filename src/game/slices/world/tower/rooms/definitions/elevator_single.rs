use macroquad::prelude::ImageFormat;

use crate::game::slices::ui::elements::BackgroundColorKind;

use super::*;

pub fn get_definition() -> RoomDefinition {
  RoomDefinition {
    id: RoomDefinitionId::ElevatorSingle,
    room_type: RoomType::Transportation,

    dimensions: Dimensions {
      width: 5,
      height: 1,
      // height: 5,
    },
    layer: RoomLayer::Transportation,
    validators: vec![
      validators::validate_enough_funds,
      validators::validate_rooms_do_not_overlap,
      validators::validate_transportation_room_is_within_tower_bounds,
    ],
    price: 1000,
    resizability: RoomResizability::Vertical,
    // resizability: RoomResizability::None,
    render_type: RoomDefinitionRenderType::Texture(
      Texture2D::from_file_with_format(
        include_bytes!("../../../../../../../assets/room_elevator_single.png"),
        Some(ImageFormat::Png),
      ),
      |_| None,
    ),
    occupancy_limit: 12,
    income: 1000,
  }
}
