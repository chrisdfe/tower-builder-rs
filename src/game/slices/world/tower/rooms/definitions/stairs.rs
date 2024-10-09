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
    render_type: RoomDefinitionRenderType::Color(Color::new(0.067, 0.067, 0.667, 0.5)),
    occupancy_limit: 12,
    income: 1000,
  }
}
