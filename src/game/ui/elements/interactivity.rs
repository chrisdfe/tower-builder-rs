use std::collections::VecDeque;

use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
  None,
  PrintDebugStatement,
  RemoveAllRootNodeChildren,
}

pub type ActionCreator = fn(ctx: ActionCreatorCtx) -> Action;

pub struct ActionCreatorCtx {
  pub node_id: Uuid,
}

fn _none_action_creator(_: ActionCreatorCtx) -> Action {
  Action::None
}

#[derive(Debug, Clone, Default)]
pub struct ElementEventHandlers {
  pub on_mouse_over: Option<ActionCreator>,
  pub on_mouse_out: Option<ActionCreator>,
  pub on_mouse_down: Option<ActionCreator>,
  pub on_mouse_up: Option<ActionCreator>,
}

impl ElementEventHandlers {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn has_at_least_one_not_none_handler(&self) -> bool {
    !self.is_none()
  }

  pub fn is_none(&self) -> bool {
    self.on_mouse_over.is_none()
      && self.on_mouse_out.is_none()
      && self.on_mouse_down.is_none()
      && self.on_mouse_up.is_none()
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
