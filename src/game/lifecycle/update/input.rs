use macroquad::input::{
  get_last_key_pressed, is_mouse_button_pressed, is_mouse_button_released, KeyCode,
};
use macroquad::prelude::*;

use crate::game::ui::elements::factories;
use crate::{game::Game, map::Coordinates};

pub fn update(game: &mut Game) {
  handle_input(game);
}

fn handle_input(game: &mut Game) {
  // Keyboard
  if let Some(key_char) = get_last_key_pressed() {
    handle_key_pressed(game, key_char);
  }

  // Mouse buttons
  if is_mouse_button_pressed(MouseButton::Left) {
    handle_left_button_pressed(game);
  }

  if is_mouse_button_released(MouseButton::Left) {
    handle_left_button_released(game);
  }
}

fn handle_key_pressed(game: &mut Game, key_code: KeyCode) {
  use KeyCode::*;

  match key_code {
    Key1 => {
      let layer = game.ui.elements.layers.front().unwrap();
      let root_node_id = layer.tree.root_node_id.unwrap();

      layer
        .tree
        .add_nodes(factories::create_debug_stretch_to_fill_node_group(
          root_node_id,
        ));

      game.ui.elements.clear_all_calculated_properties();
    }

    Key2 => {
      let layer = game.ui.elements.layers.front().unwrap();
      let ids = layer
        .tree
        .get_all_descendant_ids_flat(layer.tree.root_node_id.unwrap());

      println!("ids: {:?}", ids);
      layer.tree.remove_nodes_by_ids(ids);

      game.ui.elements.clear_all_calculated_properties();
    }

    W => game.add_camera_position(Coordinates { x: 0, y: 1 }),
    A => game.add_camera_position(Coordinates { x: -1, y: 0 }),
    S => game.add_camera_position(Coordinates { x: 0, y: -1 }),
    D => game.add_camera_position(Coordinates { x: 1, y: 0 }),

    Up => game.add_camera_position(Coordinates { x: 0, y: 1 }),
    Left => game.add_camera_position(Coordinates { x: -1, y: 0 }),
    Down => game.add_camera_position(Coordinates { x: 0, y: -1 }),
    Right => game.add_camera_position(Coordinates { x: 1, y: 0 }),

    _ => (),
  }
}

fn handle_left_button_pressed(game: &mut Game) {
  game.left_mouse_is_down = true;

  // if let None = game.ui.buttons.hovered_button_id {
  //   game.ui.selection_box_start = game.ui.current_selected_cell.clone();
  //   game.ui.selection_box = CoordinatesBox::at_coords(&game.ui.selection_box_start)
  // }
}

fn handle_left_button_released(game: &mut Game) {
  game.left_mouse_is_down = false;

  // if let Some(hovered_button_id) = game.ui.buttons.hovered_button_id {
  //   game.ui.buttons.clicked_button_id = Some(hovered_button_id);
  // } else {
  //   game.try_to_build_blueprint_room();

  //   game.ui.selection_box = CoordinatesBox::at_coords(&game.ui.current_selected_cell);
  //   game
  //     .tools
  //     .blueprint_room
  //     .calculate_coordinates_box(&game.ui.selection_box);
  // }
}

/*
fn debug_find_route(game: &Game) {
  let _ = game
    .world
    .tower
    .tower
    .find_route_from_lobby_to(game.ui.current_selected_cell.y);
}

fn debug_room_print(game: &Game) {
  // let serialized = serde_json::to_string(&game).unwrap();
}

fn save_game_state_to_file(game: &Game) -> std::io::Result<()> {
  // use std::fs::{create_dir_all, OpenOptions};
  // use std::io::prelude::*;

  // const ROOT: &'static str = "test_saves";

  // let serialized = serde_json::to_string(&game).unwrap();

  // create_dir_all(ROOT)?;
  // let mut file = OpenOptions::new()
  //   .write(true)
  //   .truncate(true)
  //   .create(true)
  //   .open(format!("{}/test.tdt", ROOT))?;
  // file.write_all(serialized.as_bytes())?;
  // file.flush()?;

  Ok(())
}

fn load_game_state_from_file(game: &mut Game) -> std::io::Result<()> {
  // use std::fs::File;
  // use std::io::prelude::*;

  // const ROOT: &'static str = "test_saves";

  // let mut file = File::open(format!("{}/test.tdt", ROOT))?;
  // let mut contents = String::new();
  // file.read_to_string(&mut contents)?;

  // let deserialized: Game = serde_json::from_str(&contents).unwrap();
  // game.load(deserialized);

  Ok(())
}
 */
