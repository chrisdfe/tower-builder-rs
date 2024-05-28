use macroquad::input::KeyCode;
use std::collections::HashMap;

use crate::map::Coordinates;

use super::super::{ui::elements::factories, Game};
use lazy_static::lazy_static;

use KeyCode::*;

pub type KeyEventHandler = for<'a> fn(game: &'a mut Game) -> ();

lazy_static! {
  pub static ref KEY_DOWN_HANDLERS: HashMap<KeyCode, KeyEventHandler> = HashMap::from([
    (Key1, handle_key1_down as KeyEventHandler),
    (Key2, handle_key2_down as KeyEventHandler),
    (W, handle_w_down as KeyEventHandler),
    (A, handle_a_down as KeyEventHandler),
    (S, handle_s_down as KeyEventHandler),
    (D, handle_d_down as KeyEventHandler),
    (Up, handle_up_down as KeyEventHandler),
    (Left, handle_left_down as KeyEventHandler),
    (Down, handle_down_down as KeyEventHandler),
    (Right, handle_right_down as KeyEventHandler),
  ]);
}

fn handle_key1_down(game: &mut Game) {
  let root_node_id = game.ui.elements.tree.root_node_id.unwrap();

  game
    .ui
    .elements
    .tree
    .add_nodes(factories::debug::create_stretch_to_fill_node_group(
      root_node_id,
    ));

  game.ui.elements.clear_all_calculated_properties();
}

fn handle_key2_down(game: &mut Game) {
  let ids = game
    .ui
    .elements
    .tree
    .get_all_descendant_ids_flat(game.ui.elements.tree.root_node_id.unwrap());

  game.ui.elements.tree.remove_nodes_by_ids(ids);

  game.ui.elements.clear_all_calculated_properties();
}

fn handle_w_down(game: &mut Game) {
  game.add_camera_position(Coordinates { x: 0, y: 1 })
}

fn handle_a_down(game: &mut Game) {
  game.add_camera_position(Coordinates { x: -1, y: 0 })
}

fn handle_s_down(game: &mut Game) {
  game.add_camera_position(Coordinates { x: 0, y: -1 })
}

fn handle_d_down(game: &mut Game) {
  game.add_camera_position(Coordinates { x: 1, y: 0 })
}

fn handle_up_down(game: &mut Game) {
  game.add_camera_position(Coordinates { x: 0, y: 1 })
}

fn handle_left_down(game: &mut Game) {
  game.add_camera_position(Coordinates { x: -1, y: 0 })
}

fn handle_down_down(game: &mut Game) {
  game.add_camera_position(Coordinates { x: 0, y: -1 })
}

fn handle_right_down(game: &mut Game) {
  game.add_camera_position(Coordinates { x: 1, y: 0 })
}
