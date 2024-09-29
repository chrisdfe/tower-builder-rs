use macroquad::input::{
  get_last_key_pressed, is_mouse_button_pressed, is_mouse_button_released, KeyCode,
};
use macroquad::prelude::*;

use crate::game::slices::input::KEY_DOWN_HANDLERS;
use crate::game::Game;
use crate::types::map::CoordinatesBox;

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
    game.input.left_mouse_is_down = true;
    game.input.left_mouse_has_just_been_pressed = true;
    handle_left_button_pressed(game);
  } else {
    game.input.left_mouse_has_just_been_pressed = false;
  }

  if is_mouse_button_released(MouseButton::Left) {
    game.input.left_mouse_is_down = false;
    game.input.left_mouse_has_just_been_released = true;
    handle_left_button_released(game);
  } else {
    game.input.left_mouse_has_just_been_released = false;
  }
}

fn handle_key_pressed(game: &mut Game, key_code: KeyCode) {
  if let Some(handler) = KEY_DOWN_HANDLERS.get(&key_code) {
    handler(game);
  }
}

fn handle_left_button_pressed(game: &mut Game) {
  // if let None = game.ui.buttons.hovered_button_id {
  if !game.ui.mouse_is_over_ui() {
    game
      .tools
      .selection
      .start_selection_box_at_current_cell();
  }
}

fn handle_left_button_released(game: &mut Game) {
  if !game.ui.mouse_is_over_ui() {
    use crate::game::slices::tools::Tool;
    match game.tools.tool.current {
      Tool::Build => {
        game.try_to_build_blueprint_room();
      }
      Tool::Destroy => {
        game.try_to_destroy_room_at_current_cell();
      }
      Tool::None => (),
    }
  }
}

pub fn post_update(game: &mut Game) {
  // noop
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
