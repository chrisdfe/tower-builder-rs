use std::collections::VecDeque;

use macroquad::color::*;
use uuid::Uuid;

use crate::tower::rooms::definitions::RoomDefinitionId;

use super::{BackgroundColorKind, Element};

// TODO - this being here couples game logic with UI framework logic, I should probably move it
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
  None,
  PrintDebugStatement,
  SetSelectedRoomDefinition(RoomDefinitionId),
}

pub type ActionCreator = fn(ctx: ActionCreatorCtx) -> Action;

pub struct ActionCreatorCtx<'a> {
  pub element: &'a Element,
}

#[derive(Debug, Clone)]
pub struct InteractivityConfig {
  pub background_color_over: BackgroundColorKind,
  pub background_color_down: BackgroundColorKind,
  pub background_color_up: BackgroundColorKind,

  pub on_mouse_over: Option<ActionCreator>,
  pub on_mouse_out: Option<ActionCreator>,
  pub on_mouse_down: Option<ActionCreator>,
  pub on_mouse_up: Option<ActionCreator>,
}

impl Default for InteractivityConfig {
  fn default() -> Self {
    Self {
      background_color_over: BackgroundColorKind::Fixed(BLUE),
      background_color_down: BackgroundColorKind::Fixed(GREEN),
      background_color_up: BackgroundColorKind::Fixed(YELLOW),

      on_mouse_over: None,
      on_mouse_out: None,
      on_mouse_down: None,
      on_mouse_up: None,
    }
  }
}

impl InteractivityConfig {
  pub fn has_at_least_one_not_none_handler(&self) -> bool {
    !self.is_none()
  }

  pub fn is_none(&self) -> bool {
    self.on_mouse_over == None
      && self.on_mouse_out == None
      && self.on_mouse_down == None
      && self.on_mouse_up == None
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

  pub fn pop(&mut self) -> Option<QueuedAction> {
    self.queue.pop_front()
  }

  pub fn push(&mut self, action: QueuedAction) {
    self.queue.push_front(action)
  }
}

#[derive(Debug, Clone)]
pub struct QueuedAction {
  pub action_creator: fn(ActionCreatorCtx) -> Action,
  pub node_id: Uuid,
}
