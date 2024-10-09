use macroquad::{
  color::{BLUE, RED},
  math::Rect,
  shapes::draw_rectangle,
};

use crate::{
  game::{slices::tools::Tool, Game},
  utils::cell_to_screen_dimensions,
};

pub fn render(game: &Game) {
  if game.ui.mouse_is_over_ui() {
    return;
  }

  match &game.tools.tool.current {
    Tool::Build => draw_build_tool(game),
    Tool::Destroy => draw_destroy_tool(game),
    Tool::None => {
      // noop (for now)
    }
  }
}

fn draw_build_tool(game: &Game) {
  draw_room_blueprint(game);
}

fn draw_room_blueprint(game: &Game) {
  if game.ui.mouse_is_over_ui() {
    return;
  }

  if let Tool::Build = &game.tools.tool.current {
    let mut color = if game.tools.build_tool.blueprint_room.is_valid() {
      BLUE.clone()
    } else {
      RED.clone()
    };

    color.a = 0.4;

    super::rooms::render_room(&game.tools.build_tool.blueprint_room, Some(color), game);
  }
}

fn draw_destroy_tool(game: &Game) {
  let current_cell = &game.tools.selection.current_selected_cell;

  let Rect { w, h, x, y } = cell_to_screen_dimensions(&current_cell, game);

  // TODO - if current_cell is over a room, highlight the whole room
  let color = {
    let mut color = RED.clone();
    color.a = 0.5;
    color
  };

  draw_rectangle(x, y, w, h, color);
}
