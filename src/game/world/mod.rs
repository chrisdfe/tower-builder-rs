use serde::{Deserialize, Serialize};

mod time;
mod tower;
mod wallet;

pub use time::TimeSlice;
pub use tower::TowerSlice;
pub use wallet::WalletSlice;

#[derive(Serialize, Deserialize)]
pub struct World {
  pub time: time::TimeSlice,
  pub tower: tower::TowerSlice,
  pub wallet: wallet::WalletSlice,
}

impl Default for World {
  fn default() -> Self {
    World::new()
  }
}

impl World {
  pub fn new() -> Self {
    Self {
      time: TimeSlice::new(),
      tower: TowerSlice::new(),
      wallet: WalletSlice::new(),
    }
  }
}
