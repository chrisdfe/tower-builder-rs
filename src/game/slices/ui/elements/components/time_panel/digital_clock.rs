use crate::{
  game::slices::ui::{
    actions::{ElementAction, ElementActionCreatorCtx},
    Element,
  },
  types::tree::TreeNodeInput,
};

pub fn create_node_input() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from("digital clock"),
      on_update: Some(on_update),

      ..Default::default()
    },
    Vec::new(),
  )
}

fn on_update(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  let time = ctx.world.time.current_time();
  let padded_hours = if time.hour < 10 {
    format!("0{}", time.hour)
  } else {
    format!("{}", time.hour)
  };

  let padded_minutes = if time.minute < 10 {
    format!("0{}", time.minute)
  } else {
    format!("{}", time.minute)
  };

  ElementAction::UpdateText(format!("{}:{}", padded_hours, padded_minutes))
}
