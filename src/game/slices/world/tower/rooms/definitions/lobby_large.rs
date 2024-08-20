use super::*;

pub fn get_definition() -> RoomDefinition {
  RoomDefinition {
    id: RoomDefinitionId::Lobby,
    room_type: RoomType::Lobby,

    dimensions: Dimensions {
      width: 1,
      height: 1,
    },
    layer: RoomLayer::Default,
    validators: with_base_room_validators(vec![validators::validate_lobby_is_on_correct_floor]),
    price: 1000,
    resizability: RoomResizability::Horizontal,
    color: Color::from_hex(0xaaaa55),
    occupancy_limit: 0,
    income: 0,
  }
}
