use std::collections::VecDeque;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
  None,
  PrintDebugStatement,
  AddAnotherNode,
}

#[derive(Debug, Clone)]
pub struct ElementEventHandlers {
  pub on_mouse_over: Action,
  pub on_mouse_out: Action,
  pub on_mouse_down: Action,
  pub on_mouse_up: Action,
}

impl Default for ElementEventHandlers {
  fn default() -> Self {
    Self {
      on_mouse_over: Action::None,
      on_mouse_out: Action::None,
      on_mouse_down: Action::None,
      on_mouse_up: Action::None,
    }
  }
}

impl ElementEventHandlers {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn has_at_least_one_not_none_handler(&self) -> bool {
    !self.is_none()
  }

  pub fn is_none(&self) -> bool {
    self.on_mouse_over == Action::None
      && self.on_mouse_out == Action::None
      && self.on_mouse_down == Action::None
      && self.on_mouse_up == Action::None
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
  pub action: Action,
  pub node_id: Uuid,
}
