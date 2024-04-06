// pub enum DayPeriod {
//   Dawn,
//   Morning,
//   Afternoon,
//   Evening,
//   Dusk,
//   Night,
// }

pub enum WeekDayType {
  Weekday,
  Weekend,
}

pub struct WeekDay {
  pub name: &'static str,
  pub week_day_type: WeekDayType,
}

pub enum MonthType {
  Spring,
  Summer,
  Autumn,
  Winter,
}
