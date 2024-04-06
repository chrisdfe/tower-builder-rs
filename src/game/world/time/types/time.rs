use std::ops::{Add, Sub};

use super::super::constants::*;

#[derive(PartialEq)]
pub struct Time {
  // 0-59
  pub minute: u32,
  // 0-23
  pub hour: u32,
  // day of month
  pub day: u32,
  // month of year
  pub month: u32,
  // 1 +
  pub year: u32,
}

impl Default for Time {
  fn default() -> Self {
    Time {
      minute: 0,
      hour: 0,
      day: 0,
      month: 0,
      year: 0,
    }
  }
}

impl From<TimeInput> for Time {
  fn from(input: TimeInput) -> Self {
    let minute = if let Some(minute) = input.minute {
      minute
    } else {
      0
    };
    let hour = if let Some(hour) = input.hour { hour } else { 0 };
    let day = if let Some(day) = input.day { day } else { 0 };
    let month = if let Some(month) = input.month {
      month
    } else {
      0
    };
    let year = if let Some(year) = input.year { year } else { 0 };

    Time {
      minute,
      hour,
      day,
      month,
      year,
    }
  }
}

impl Add<TimeInput> for Time {
  type Output = Self;

  fn add(self, input: TimeInput) -> Self::Output {
    let time_as_minutes = self.to_minutes();
    let input_as_minutes = Time::from(input).to_minutes();

    let new_minutes = time_as_minutes + input_as_minutes;
    Time::from_minutes(new_minutes as u64)
  }
}

impl Sub<TimeInput> for Time {
  type Output = Self;

  // TODO - make sure time doesn't go below 0
  fn sub(self, input: TimeInput) -> Self::Output {
    let time_as_minutes = self.to_minutes();
    let input_as_minutes = Time::from(input).to_minutes();

    let new_minutes = time_as_minutes - input_as_minutes;
    Time::from_minutes(new_minutes as u64)
  }
}

impl Time {
  // pub fn zero() -> Self {
  //   Self {
  //     minute: 0,
  //     hour: 0,
  //     day: 0,
  //     month: 0,
  //     year: 0,
  //   }
  // }

  pub fn from_minutes(minutes: u64) -> Self {
    let mut leftover = minutes;
    let year = (leftover as f32 / MINUTES_PER_YEAR as f32).floor() as u32;
    leftover = leftover % MINUTES_PER_YEAR as u64;

    let month = (leftover as f32 / MINUTES_PER_MONTH as f32).floor() as u32;
    leftover = leftover % MINUTES_PER_MONTH as u64;

    let day = (leftover as f32 / MINUTES_PER_DAY as f32).floor() as u32;
    leftover = leftover % MINUTES_PER_DAY as u64;

    let hour = (leftover as f32 / MINUTES_PER_HOUR as f32).floor() as u32;
    leftover = leftover % MINUTES_PER_HOUR as u64;

    let minute = leftover as u32;

    Time {
      minute,
      hour,
      day,
      month,
      year,
    }
  }

  pub fn to_minutes(&self) -> u32 {
    let minutes = self.minute;
    let hour_minutes = self.hour * MINUTES_PER_HOUR;
    let day_minutes = self.day * MINUTES_PER_DAY;
    let month_minutes = self.month * MINUTES_PER_MONTH;
    let year_minutes = self.year * MINUTES_PER_YEAR;

    minutes + hour_minutes + day_minutes + month_minutes + year_minutes
  }

  // pub fn get_day_period(&self) -> DayPeriod {
  //   match self.hour {
  //     hour if hour < 6 => DayPeriod::Night,
  //     hour if hour < 7 => DayPeriod::Dawn,
  //     hour if hour < 12 => DayPeriod::Morning,
  //     hour if hour < 17 => DayPeriod::Afternoon,
  //     hour if hour < 20 => DayPeriod::Evening,
  //     hour if hour < 21 => DayPeriod::Dusk,
  //     _ => DayPeriod::Night,
  //   }
  // }
}

pub struct TimeInput {
  pub minute: Option<u32>,
  pub hour: Option<u32>,
  pub day: Option<u32>,
  pub week: Option<u32>,
  pub month: Option<u32>,
  pub year: Option<u32>,
}
