use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Slice {
  pub funds: i64,
}

impl Slice {
  pub fn new() -> Self {
    Self { funds: 100_000 }
  }

  pub fn add_funds(&mut self, amount: u32) {
    self.funds += amount as i64;
  }

  pub fn subtract_funds(&mut self, amount: u32) {
    self.funds -= amount as i64;
  }
}
