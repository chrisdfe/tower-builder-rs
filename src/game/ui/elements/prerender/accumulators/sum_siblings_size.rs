pub fn sum_siblings_size(acc: u32, sibling_size: u32, gap: u32, idx: usize) -> u32 {
  let value = if idx > 0 {
    sibling_size + gap
  } else {
    sibling_size
  };

  acc + value
}

#[cfg(test)]
mod tests {
  // use super::*;

  // #[test]
  // fn it_calculates_the_correct_value() {
  //   let values = vec![1, 2, 5, 4, 2];
  //   let result = values.into_iter().fold(0, max_sibling_size);
  //   assert_eq!(result, 5);
  // }
}
