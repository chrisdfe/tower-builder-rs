use crate::game::slices::ui::elements::ContentAlignment;

pub fn align_within_wrapper(size: u32, wrapper_size: u32, alignment: &ContentAlignment) -> i32 {
  match alignment {
    ContentAlignment::Start => 0,
    ContentAlignment::Center => (wrapper_size as i32 / 2) - (size as i32 / 2) as i32,
    ContentAlignment::End => (wrapper_size as i32) - (size as i32),
  }
}

#[cfg(test)]
mod tests {
  pub use super::*;

  #[cfg(test)]
  mod align_within_wrapper {
    pub use super::*;

    #[test]
    pub fn start_alignment() {
      let test_cases = vec![
        (10, 50),
        (10, 100),
        (40, 100),
        (60, 100),
        (90, 100),
        (100, 100),
        (10, 10),
      ];

      for (size, wrapper_size) in test_cases {
        assert_eq!(
          align_within_wrapper(size, wrapper_size, &ContentAlignment::Start),
          0
        );
      }
    }

    #[test]
    pub fn center_alignment() {
      let test_cases = vec![
        (10, 50, 20),
        (10, 100, 45),
        (40, 100, 30),
        (60, 100, 20),
        (90, 100, 5),
        (100, 100, 0),
        (10, 10, 0),
      ];

      for (size, wrapper_size, correct_result) in test_cases {
        assert_eq!(
          align_within_wrapper(size, wrapper_size, &ContentAlignment::Center),
          correct_result
        );
      }
    }

    #[test]
    pub fn end_alignment() {
      let test_cases = vec![
        (10, 50, 40),
        (10, 100, 90),
        (40, 100, 60),
        (60, 100, 40),
        (90, 100, 10),
        (100, 100, 0),
        (10, 10, 0),
      ];

      for (size, wrapper_size, correct_result) in test_cases {
        assert_eq!(
          align_within_wrapper(size, wrapper_size, &ContentAlignment::End),
          correct_result
        );
      }
    }
  }
}
