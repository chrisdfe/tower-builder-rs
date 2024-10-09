use super::*;

pub fn get_definition() -> RoomDefinition {
  RoomDefinition {
    id: RoomDefinitionId::Floor,
    room_type: RoomType::None,
    dimensions: Dimensions {
      width: 1,
      height: 1,
    },
    layer: RoomLayer::Default,
    validators: with_base_room_validators(vec![
      validators::validate_room_is_above_ground,
      validators::validate_non_lobby_is_not_on_ground_floor,
    ]),
    price: 1000,
    resizability: RoomResizability::Horizontal,
    render_type: RoomDefinitionRenderType::Color(Color::from_hex(0x000000)),
    occupancy_limit: 0,
    income: 10000,
  }
}
