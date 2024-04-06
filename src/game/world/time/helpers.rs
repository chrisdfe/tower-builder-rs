// use super::constants::*;
// use super::types::*;

// pub fn get_day_of_week_index(day: u32) -> u32 {
//   day % DAY_SEQUENCE.len() as u32
// }

// pub fn get_month_of_year_index(month: u32) -> u32 {
//   month % MONTHS_PER_YEAR
// }

// // TODO - PartialEq for TimeInput/Time
// pub fn time_input_matches_time(time_input: &TimeInput, time: &Time) -> bool {
//   if let Some(minute) = time_input.minute {
//     if minute != time.minute {
//       return false;
//     }
//   }

//   if let Some(hour) = time_input.hour {
//     if hour != time.hour {
//       return false;
//     }
//   }

//   if let Some(day) = time_input.day {
//     if day != time.day {
//       return false;
//     }
//   }

//   if let Some(month) = time_input.month {
//     if month != time.month {
//       return false;
//     }
//   }

//   if let Some(year) = time_input.year {
//     if year != time.year {
//       return false;
//     }
//   }

//   true
// }
