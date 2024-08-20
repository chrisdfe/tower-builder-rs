use uuid::Uuid;

use crate::game::slices::world::tower::Tower;

#[derive(Debug, Clone)]
pub struct RouteNode {
  pub from_floor: i32,
  pub to_floor: i32,
  // The transportation item used to travel from from_floor to to_floor
  pub transportation_room_id: Uuid,
}

// Starting out with v. basic algorithm
#[derive(Debug, Clone)]
pub struct Route {
  pub path: Vec<RouteNode>,
}

impl Route {
  pub fn new() -> Self {
    Self { path: Vec::new() }
  }
}

#[derive(Debug, Clone)]
pub struct RouteAttempt {
  route: Route,
  has_reached_destination: bool,
  current_floor: i32,
  visited_floors: Vec<i32>,
  visited_transportation_item_ids: Vec<Uuid>,
}

impl RouteAttempt {
  pub fn new() -> Self {
    Self {
      route: Route::new(),
      has_reached_destination: false,
      current_floor: 0,
      visited_floors: vec![0],
      visited_transportation_item_ids: Vec::new(),
    }
  }
}

pub fn find_route_from_lobby_to(tower: &Tower, destination_floor: i32) -> Option<Route> {
  use std::collections::VecDeque;

  let mut completed_route_attempts: Vec<RouteAttempt> = Vec::new();

  let mut route_attempts_to_process: VecDeque<RouteAttempt> = VecDeque::new();
  let first_route_attempt = RouteAttempt::new();
  route_attempts_to_process.push_back(first_route_attempt);

  while let Some(mut route_attempt) = route_attempts_to_process.pop_front() {
    let transportation_rooms_connected_to_floor: Vec<Uuid> =
      tower.find_transportation_rooms_connected_to_floor(route_attempt.current_floor);

    let mut should_add_to_completed_routes = false;
    if route_attempt.current_floor == destination_floor {
      route_attempt.has_reached_destination = true;
      should_add_to_completed_routes = true;
    } else {
      let transportation_room_and_floor_tuples = transportation_rooms_connected_to_floor
        .iter()
        .filter(|transportation_room_id| {
          !route_attempt
            .visited_transportation_item_ids
            .contains(transportation_room_id)
        })
        .flat_map(|transportation_room_id| {
          let transportation_room = tower
            .rooms
            .iter()
            .find(|room| room.id == *transportation_room_id)
            .unwrap();

          // For now, transportation room connects to every floor its on
          let floors = transportation_room
            .coordinates_box
            .y_range()
            .filter(|floor| !route_attempt.visited_floors.contains(floor));

          floors.map(|floor| (transportation_room_id.clone(), floor))
        })
        .collect::<Vec<_>>();

      if transportation_room_and_floor_tuples.len() == 0 {
        // Nowhere left to go & we're at a dead end
        should_add_to_completed_routes = true
      } else {
        // Continue searching for routes to the destination floor
        for (transportation_room_id, floor) in transportation_room_and_floor_tuples {
          let prev_floor = route_attempt.current_floor;

          let new_route_attempt = {
            let mut result = route_attempt.clone();
            result.visited_floors.push(floor);
            result
              .visited_transportation_item_ids
              .push(transportation_room_id);
            result.current_floor = floor;
            result.route.path.push(RouteNode {
              from_floor: prev_floor,
              to_floor: floor,
              transportation_room_id,
            });
            result
          };
          route_attempts_to_process.push_back(new_route_attempt);
        }
      }
    }

    if should_add_to_completed_routes {
      completed_route_attempts.push(route_attempt.clone());
    }
  }

  let completed_successful_routes = completed_route_attempts
    .iter()
    .filter(|route| route.has_reached_destination)
    .collect::<Vec<_>>();

  if completed_successful_routes.len() > 0 {
    // Just get the first one for now
    // TODO - compare lengths and pick the shortest
    Some(
      completed_successful_routes
        .first()
        .unwrap()
        .route
        .clone(),
    )
  } else {
    None
  }
}
