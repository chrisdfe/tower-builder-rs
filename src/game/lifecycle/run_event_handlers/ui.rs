// use crate::game::ui::elements::ButtonEventHandlerContext;
use crate::game::Game;

pub fn run_event_handlers(game: &mut Game) {
  // button click handlers
  // if let Some(clicked_button_id) = game.ui.buttons.clicked_button_id {
  //   if let Some(clicked_button) = game
  //     .ui
  //     .buttons
  //     .buttons
  //     .iter()
  //     .find(|button| button.id == clicked_button_id)
  //   {
  //     let ctx = ButtonEventHandlerContext {
  //       button: clicked_button,
  //       world: &mut game.world,
  //       tools: &mut game.tools,
  //       selection_box: &game.ui.selection_box,
  //     };

  //     (clicked_button.on_click)(ctx);
  //   }

  //   game.ui.buttons.clicked_button_id = None;
  // }
}
