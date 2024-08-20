use uuid::Uuid;

use super::TimerLoopType;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TimerId {
  None,
  Tick,
}

#[derive(Clone)]
pub struct Timer {
  pub id: TimerId,
  pub length: f32,
  pub current_value: f32,
  pub loop_type: TimerLoopType,
}

impl PartialEq for Timer {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl Default for Timer {
  fn default() -> Self {
    Self {
      id: TimerId::None,
      length: 1.,
      current_value: 0.,
      loop_type: TimerLoopType::Once,
    }
  }
}
