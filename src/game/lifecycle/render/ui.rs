use macroquad::{color::Color, shapes::draw_rectangle};

use crate::game::{
  slices::ui::elements::{calculated::UnwrappedElementCalculatedProperties, Element},
  Game,
};

pub fn render(game: &Game) {
  // Draw from the top down so child nodes will be rendered on top of parent nodes
  let ids = game
    .ui
    .elements
    .tree
    .get_node_ids_grouped_by_depth_top_down_flat();
  let sorted_nodes = game.ui.elements.tree.find_nodes_by_ids(&ids);

  for node in sorted_nodes {
    render_element(&node.data, &game);
  }
}

fn render_element(element: &Element, game: &Game) {
  let UnwrappedElementCalculatedProperties {
    outer_position,
    outer_dimensions,
    content_position,
    content_dimensions,
    background_color,
    ..
  } = element.calculated.unwrap();

  // Draw outer (background)
  {
    let x = outer_position.x;
    let y = outer_position.y;
    let w = outer_dimensions.width as f32;
    let h = outer_dimensions.height as f32;

    draw_rectangle(x, y, w, h, *background_color);
  };

  // Draw content (background)
  {
    let x = content_position.x;
    let y = content_position.y;
    let w = content_dimensions.width as f32;
    let h = content_dimensions.height as f32;
    // let debug_content_background_color = Color::from_rgba(255, 255, 255, 100);
    let debug_content_background_color = Color::new(0., 0., 0., 0.);

    draw_rectangle(x, y, w, h, debug_content_background_color);
  }

  // Render leaf node content
  element.content_renderer.render(&element, &game);
}
