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

  pub fn set_current(&mut self, value: T) {
    self.prev = self.current.clone();
    self.current = Some(value);
  }

  pub fn set_maybe_current(&mut self, value: Option<T>) {
    self.prev = self.current.clone();
    self.current = value;
  }

  /// Returns true of prev != current, and returns false if prev == current
  pub fn has_changed(&self) -> bool {
    if self.current.is_none() && self.prev.is_none() {
      // Both values are None
      false
    } else if self.current.is_some() && self.prev.is_none()
      || self.current.is_none() && self.prev.is_some()
    {
      // Only 1 value is None
      true
    } else {
      // both prev and current have values - use PartialEq to compare
      let prev = self.prev.as_ref().unwrap();
      let current = self.current.as_ref().unwrap();
      prev != current
    }
  }
}
