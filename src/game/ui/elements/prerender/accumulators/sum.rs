use std::ops::Add;

pub fn sum<T: Add<Output = T>>(acc: T, num: T) -> T {
  acc + num
}
