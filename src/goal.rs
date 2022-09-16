use chrono::{self, Datelike};

pub fn get_daily_goal(current_day: impl Datelike, yearly_target: u32) -> u32 {
    current_day.ordinal() * yearly_target / 365
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, IsoWeek, Weekday};
    use mockall::mock;
    mock! {
        DateTime{}
        impl Datelike for DateTime {
            fn year(&self) -> i32;
            fn month(&self) -> u32;
            fn month0(&self) -> u32;
            fn day(&self) -> u32;
            fn day0(&self) -> u32;
            fn ordinal(&self) -> u32;
            fn ordinal0(&self) -> u32;
            fn weekday(&self) -> Weekday;
            fn iso_week(&self) -> IsoWeek;
            fn with_year(&self, year: i32) -> Option<Self>;
            fn with_month(&self, month: u32) -> Option<Self>;
            fn with_month0(&self, month0: u32) -> Option<Self>;
            fn with_day(&self, day: u32) -> Option<Self>;
            fn with_day0(&self, day0: u32) -> Option<Self>;
            fn with_ordinal(&self, ordinal: u32) -> Option<Self>;
            fn with_ordinal0(&self, ordinal0: u32) -> Option<Self>;
            fn year_ce(&self) -> (bool, u32) { ... }
            fn num_days_from_ce(&self) -> i32 { ... }
        }
    }

    #[test]
    fn test_start_of_year() {
        let mut datetimemock = MockDateTime::new();
        datetimemock.expect_ordinal().return_const(0u32);
        assert_eq!(get_daily_goal(datetimemock, 1000), 0);
    }

    #[test]
    fn test_end_of_year() {
        let mut datetimemock = MockDateTime::new();
        datetimemock.expect_ordinal().return_const(365u32);
        assert_eq!(get_daily_goal(datetimemock, 1000), 1000);
    }

    #[test]
    fn test_mid_year() {
        let mut datetimemock = MockDateTime::new();
        datetimemock.expect_ordinal().return_const(182u32);
        assert!((500 - get_daily_goal(datetimemock, 1000)) < 3);
    }
}
