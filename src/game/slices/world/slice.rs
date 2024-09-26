use serde::{Deserialize, Serialize};

use super::{camera, time, tower, wallet};

#[derive(Serialize, Deserialize)]
pub struct Slice {
  pub time: time::Slice,
  pub tower: tower::Slice,
  pub wallet: wallet::Slice,
  pub camera: camera::Slice,
}

impl Default for Slice {
  fn default() -> Self {
    Self::new()
  }
}

impl Slice {
  pub fn new() -> Self {
    Self {
      time: time::Slice::new(),
      tower: tower::Slice::new(),
      wallet: wallet::Slice::new(),
      camera: camera::Slice::new(),
    }
  }
}
