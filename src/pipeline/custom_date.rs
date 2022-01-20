//! A notion of the calendar date.

use time;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::cmp::Ordering;

/// A representation of a day in the Gregorian calendar.
#[derive(Clone, Copy, Default, PartialEq, Eq, Ord, Debug, Deserialize, Serialize)]
pub struct Date {
    /// The year.
    pub year: u32,
    /// The month.
    pub month: u8,
    /// The day.
    pub day: u8,
}

impl fmt::Display for Date {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, formatter)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        macro_rules! cmp(
            ($one:expr, $two:expr) => (
                if $one > $two {
                    return Some(Ordering::Greater);
                } else if $one < $two {
                    return Some(Ordering::Less);
                }
            );
        );

        cmp!(self.year, other.year);
        cmp!(self.month, other.month);
        cmp!(self.day, other.day);

        Some(Ordering::Equal)
    }
}

impl Date {
    /// Create a date by the specified year, month, and day.
    #[inline]
    pub fn new(year: u32, month: u8, day: u8) -> Date {
        Date { year: year, month: month, day: day }
    }

    /// Return the UTC date specified in seconds counting from the Unix epoch.
    pub fn at_utc(seconds: i64) -> Date {
        let time = time::at_utc(time::Timespec { sec: seconds, nsec: 0 });
        Date::new(time.tm_year as u32 + 1900, time.tm_mon as u8 + 1, time.tm_mday as u8)
    }

}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! date(
        ($year:expr, $month:expr, $day:expr) => (Date::new($year, $month, $day));
    );

    #[test]
    fn eq() {
        assert_eq!(date!(2014, 8, 19), date!(2014, 8, 19));
    }

    #[test]
    fn ord() {
        assert!(date!(2014, 8, 19) < date!(2014, 8, 20));
        assert!(date!(2014, 8, 19) > date!(2014, 8, 18));
        assert!(date!(2014, 8, 19) < date!(2014, 9, 19));
        assert!(date!(2014, 8, 19) > date!(2014, 7, 19));
        assert!(date!(2014, 8, 19) < date!(2015, 8, 19));
        assert!(date!(2014, 8, 19) > date!(2013, 8, 19));
    }
}
