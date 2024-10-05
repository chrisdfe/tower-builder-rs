use crate::{
  game::slices::{
    ui::{
      actions::{ElementAction, ElementActionCreator, ElementActionCreatorCtx},
      interactivity::{ElementInteractivity, ElementInteractivityConfig},
      Element,
    },
    world::time::slice::TimeSpeed,
  },
  types::tree::TreeNodeInput,
};

pub const WRAPPER_HANDLE: &'static str = "speed buttons wrapper";

pub fn create_node_input() -> TreeNodeInput<Element> {
  TreeNodeInput(
    Element {
      name: String::from(WRAPPER_HANDLE),
      handle: WRAPPER_HANDLE,

      ..Default::default()
    },
    create_buttons(),
  )
}

fn create_buttons() -> Vec<TreeNodeInput<Element>> {
  //
  [
    (
      "pause",
      switch_to_paused as ElementActionCreator,
      on_paused_update as ElementActionCreator,
    ),
    ("normal", switch_to_normal, on_normal_update),
    ("fast", switch_to_fast, on_fast_update),
    ("very fast", switch_to_very_fast, on_very_fast_update),
  ]
  .into_iter()
  .map(|(handle, on_click, on_update)| {
    TreeNodeInput(
      Element {
        name: handle.to_string(),
        handle: handle,
        text: Some(handle.to_string()),

        on_update: Some(on_update),
        interactivity: Some(ElementInteractivity {
          config: ElementInteractivityConfig {
            on_mouse_up: Some(on_click),
            ..Default::default()
          },
          ..Default::default()
        }),

        ..Default::default()
      },
      Vec::new(),
    )
  })
  .collect::<Vec<_>>()
}

fn on_update(ctx: ElementActionCreatorCtx, _: &Element, button_speed: TimeSpeed) -> ElementAction {
  if ctx.world.time.speed().has_changed() {
    let is_active = ctx.world.time.speed().current == button_speed;
    ElementAction::UpdateActiveState(is_active)
  } else {
    ElementAction::None
  }
}

fn on_paused_update(ctx: ElementActionCreatorCtx, element: &Element) -> ElementAction {
  on_update(ctx, element, TimeSpeed::Paused)
}

fn switch_to_paused(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::UpdateTimeSpeed(TimeSpeed::Paused)
}

fn on_normal_update(ctx: ElementActionCreatorCtx, element: &Element) -> ElementAction {
  on_update(ctx, element, TimeSpeed::Normal)
}

fn switch_to_normal(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::UpdateTimeSpeed(TimeSpeed::Normal)
}

fn on_fast_update(ctx: ElementActionCreatorCtx, element: &Element) -> ElementAction {
  on_update(ctx, element, TimeSpeed::Fast)
}

fn switch_to_fast(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::UpdateTimeSpeed(TimeSpeed::Fast)
}

fn on_very_fast_update(ctx: ElementActionCreatorCtx, element: &Element) -> ElementAction {
  on_update(ctx, element, TimeSpeed::VeryFast)
}

fn switch_to_very_fast(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::UpdateTimeSpeed(TimeSpeed::VeryFast)
}
