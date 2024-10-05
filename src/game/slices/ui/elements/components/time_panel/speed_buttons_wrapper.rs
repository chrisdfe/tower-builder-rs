use std::f32::consts::PI;

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
    ("pause", update_to_paused as ElementActionCreator),
    ("normal", update_to_normal),
    ("fast", update_to_fast),
    ("very fast", update_to_very_fast),
  ]
  .into_iter()
  .map(|(handle, on_click)| {
    TreeNodeInput(
      Element {
        name: handle.to_string(),
        handle: handle,
        text: Some(handle.to_string()),

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

fn update_to_paused(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::UpdateTimeSpeed(TimeSpeed::Paused)
}

fn update_to_normal(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::UpdateTimeSpeed(TimeSpeed::Normal)
}

fn update_to_fast(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::UpdateTimeSpeed(TimeSpeed::Fast)
}

fn update_to_very_fast(_: ElementActionCreatorCtx, _: &Element) -> ElementAction {
  ElementAction::UpdateTimeSpeed(TimeSpeed::VeryFast)
}
