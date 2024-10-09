use crate::{
  game::slices::{
    ui::{
      actions::{ElementAction, ElementActionCreator, ElementActionCreatorCtx},
      components::line_height_wrapper,
      interactivity::{ElementInteractivity, ElementInteractivityConfig},
      renderer::generic::ImageElementContentRenderer,
      ContentAlignment, Element, TwoDimensional,
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
      "./assets/icon_pause.png",
    ),
    (
      "normal",
      switch_to_normal,
      on_normal_update,
      "./assets/icon_play.png",
    ),
    (
      "fast",
      switch_to_fast,
      on_fast_update,
      "./assets/icon_fast.png",
    ),
    (
      "very fast",
      switch_to_very_fast,
      on_very_fast_update,
      "./assets/icon_very_fast.png",
    ),
  ]
  .into_iter()
  .map(|(handle, on_click, on_update, image_path)| {
    TreeNodeInput(
      Element {
        name: handle.to_string(),
        handle: handle,

        padding: 4,
        on_update: Some(on_update),

        content_alignment: TwoDimensional::same(ContentAlignment::Center),

        content_renderer: Box::new(ImageElementContentRenderer::new(image_path.to_string())),

        interactivity: Some(ElementInteractivity {
          config: ElementInteractivityConfig {
            on_mouse_up: Some(on_click),
            ..Default::default()
          },
          ..Default::default()
        }),

        ..Default::default()
      },
      vec![],
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

// TODO - none of this feels very elegant, but I'm still not sure of the best way to achieve this
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
