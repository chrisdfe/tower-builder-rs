#[derive(Debug, Clone)]
pub struct PrevAndCurrent<T: Clone + PartialEq> {
  pub prev: Option<T>,
  pub current: Option<T>,
}

impl<T: Clone + PartialEq> PrevAndCurrent<T> {
  pub fn new(current: T) -> Self {
    Self {
      prev: None,
      current: Some(current),
    }
  }

  pub fn new_none() -> Self {
    Self {
      prev: None,
      current: None,
    }
  }

  pub fn set_current(&mut self, value: Option<T>) {
    self.prev = self.current.clone();
    self.current = value;
  }

  pub fn as_tuple(&self) -> (&Option<T>, &Option<T>) {
    (&self.prev, &self.current)
  }

  /// Returns true of prev != current, and returns false if prev == current
  pub fn has_changed(&self) -> bool {
    match self.as_tuple() {
      (None, None) => false,
      (Some(_), None) => true,
      (None, Some(_)) => true,
      (Some(prev), Some(current)) => prev != current,
    }
  }
}
