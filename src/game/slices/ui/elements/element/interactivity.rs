use std::collections::VecDeque;

use macroquad::color::*;
use uuid::Uuid;

use crate::{
  game::slices::{tools::Tool, world::tower::rooms::definitions::RoomDefinitionId},
  types::PrevAndCurrent,
};

use super::{actions::ElementActionCreator, BackgroundColorKind, Element};

#[derive(Debug, Clone, PartialEq)]
pub enum ElementInteractionState {
  Default,
  Hover,
  Click,
}

#[derive(Debug, Clone)]
pub struct ElementInteractivityState {
  pub interaction_state: PrevAndCurrent<ElementInteractionState>,
  pub is_active: PrevAndCurrent<bool>,
}

impl Default for ElementInteractivityState {
  fn default() -> Self {
    Self {
      is_active: PrevAndCurrent::new(false),
      interaction_state: PrevAndCurrent::new(ElementInteractionState::Default),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ElementInteractivityConfig {
  pub background_color_over: BackgroundColorKind,
  pub background_color_down: BackgroundColorKind,
  pub background_color_up: BackgroundColorKind,
  pub background_color_active: BackgroundColorKind,

  pub on_mouse_over: Option<ElementActionCreator>,
  pub on_mouse_out: Option<ElementActionCreator>,
  pub on_mouse_down: Option<ElementActionCreator>,
  pub on_mouse_up: Option<ElementActionCreator>,
}

impl Default for ElementInteractivityConfig {
  fn default() -> Self {
    Self {
      background_color_over: BackgroundColorKind::Fixed(BLUE),
      background_color_down: BackgroundColorKind::Fixed(GREEN),
      background_color_up: BackgroundColorKind::Fixed(YELLOW),
      background_color_active: BackgroundColorKind::Fixed(PURPLE),

      on_mouse_over: None,
      on_mouse_out: None,
      on_mouse_down: None,
      on_mouse_up: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct ElementInteractivity {
  // state
  pub state: ElementInteractivityState,
  // config
  pub config: ElementInteractivityConfig,
}

impl Default for ElementInteractivity {
  fn default() -> Self {
    Self {
      state: Default::default(),
      config: Default::default(),
    }
  }
}

impl ElementInteractivity {
  pub fn has_at_least_one_not_none_handler(&self) -> bool {
    !self.is_none()
  }

  pub fn is_none(&self) -> bool {
    self.config.on_mouse_over == None
      && self.config.on_mouse_out == None
      && self.config.on_mouse_down == None
      && self.config.on_mouse_up == None
  }
}

#[derive(Debug, Clone)]
pub struct EventHandlerQueue {
  queue: VecDeque<QueuedAction>,
}

impl EventHandlerQueue {
  pub fn new() -> Self {
    Self {
      queue: VecDeque::new(),
    }
  }

  pub fn len(&self) -> usize {
    self.queue.len()
  }

  pub fn pop(&mut self) -> Option<QueuedAction> {
    self.queue.pop_front()
  }

  pub fn push(&mut self, action: QueuedAction) {
    self.queue.push_front(action)
  }
}

#[derive(Debug, Clone)]
pub struct QueuedAction {
  pub action_creator: ElementActionCreator,
  pub node_id: Uuid,
}
