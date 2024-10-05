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
      name: String::from("date text element"),
      on_update: Some(update_text_with_date),
      ..Default::default()
    },
    Vec::new(),
  )
}

fn update_text_with_date(ctx: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  let time = ctx.world.time.current_time();

  let text = format!(
    "date: day {}, month {}, year {}",
    time.day, time.month, time.year
  );

  ElementAction::UpdateText(text)
}
