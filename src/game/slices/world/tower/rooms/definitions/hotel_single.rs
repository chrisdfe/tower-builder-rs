use super::*;

pub fn get_definition() -> RoomDefinition {
  RoomDefinition {
    id: RoomDefinitionId::HotelSingle,
    room_type: RoomType::Hotel,

    dimensions: Dimensions {
      width: 5,
      height: 1,
    },
    layer: RoomLayer::Default,
    validators: with_base_room_validators(vec![
      validators::validate_room_is_above_ground,
      validators::validate_non_lobby_is_not_on_ground_floor,
    ]),
    price: 12000,
    resizability: RoomResizability::None,
    color: Color::from_hex(0xca3a47),
    occupancy_limit: 5,
    income: 1000,
  }
}
