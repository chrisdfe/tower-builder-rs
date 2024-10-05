pub fn max_sibling_size(acc: u32, sibling_length: u32) -> u32 {
  if sibling_length > acc {
    sibling_length
  } else {
    acc
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_calculates_the_correct_value() {
    let values = vec![1, 2, 5, 4, 2];
    let result = values.into_iter().fold(0, max_sibling_size);
    assert_eq!(result, 5);
  }
}
