use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WalletSlice {
  pub funds: i64,
}

impl WalletSlice {
  pub fn new() -> Self {
    Self {
      // funds: 1_000_000,
      funds: 100_000,
    }
  }

  pub fn add_funds(&mut self, amount: u32) {
    self.funds += amount as i64;
  }

  pub fn subtract_funds(&mut self, amount: u32) {
    self.funds -= amount as i64;
  }
}
