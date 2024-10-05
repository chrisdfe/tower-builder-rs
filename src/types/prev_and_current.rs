use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrevAndCurrent<T: Clone + std::fmt::Debug + PartialEq> {
  pub prev: T,
  pub current: T,
}

impl<T: Clone + std::fmt::Debug + PartialEq> PrevAndCurrent<T> {
  pub fn new(current: T) -> Self {
    Self {
      prev: current.clone(),
      current: current.clone(),
    }
  }

  pub fn tick(&mut self) {
    self.prev = self.current.clone();
  }

  pub fn set_current(&mut self, value: T) {
    self.prev = self.current.clone();
    self.current = value;
  }

  pub fn as_tuple(&self) -> (&T, &T) {
    (&self.prev, &self.current)
  }

  /// Returns true of prev != current, and returns false if prev == current
  pub fn has_changed(&self) -> bool {
    self.prev != self.current
  }
}
