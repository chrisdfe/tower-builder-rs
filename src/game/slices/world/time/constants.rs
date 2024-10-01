use super::types::*;

pub const TICK_INTERVAL_S: f32 = 0.02;
// pub const TICK_INTERVAL_S: f32 = 0.05;

// pub const MINUTES_ELAPSED_PER_TICK: u32 = 15;
pub const MINUTES_ELAPSED_PER_TICK: u32 = 1;

pub const MINUTES_PER_HOUR: u32 = 60;

pub const HOURS_PER_DAY: u32 = 24;

pub const MINUTES_PER_DAY: u32 = HOURS_PER_DAY * MINUTES_PER_HOUR;

pub const TICKS_ELAPSED_PER_DAY: u32 = MINUTES_PER_DAY / MINUTES_ELAPSED_PER_TICK;

pub const DAY_SEQUENCE: [WeekDay; 5] = [
  WeekDay {
    name: "Monday",
    week_day_type: WeekDayType::Weekday,
  },
  WeekDay {
    name: "Wednesday",
    week_day_type: WeekDayType::Weekday,
  },
  WeekDay {
    name: "Friday",
    week_day_type: WeekDayType::Weekday,
  },
  WeekDay {
    name: "Saturday",
    week_day_type: WeekDayType::Weekday,
  },
  WeekDay {
    name: "Sunday",
    week_day_type: WeekDayType::Weekend,
  },
];

pub const DAYS_PER_MONTH: u32 = DAY_SEQUENCE.len() as u32;

pub const MONTH_SEQUENCE: [MonthType; 4] = [
  MonthType::Spring,
  MonthType::Summer,
  MonthType::Autumn,
  MonthType::Winter,
];

pub const MINUTES_PER_MONTH: u32 = DAYS_PER_MONTH * MINUTES_PER_DAY;

pub const TICKS_ELAPSED_PER_MONTH: u32 = MINUTES_PER_MONTH / MINUTES_ELAPSED_PER_TICK;

pub const MONTHS_PER_YEAR: u32 = MONTH_SEQUENCE.len() as u32;

pub const MINUTES_PER_YEAR: u32 = MONTHS_PER_YEAR * MINUTES_PER_MONTH;
