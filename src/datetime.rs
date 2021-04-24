use std::fmt;

/// A struct represents date/time in TOML
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DateTime {
    date: Option<Date>,
    time: Option<Time>,
    offset: Option<Offset>,
}

impl DateTime {
    pub(crate) fn new(
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<Offset>,
    ) -> Result<Self, ()> {
        if offset.is_some() && (date.is_none() || time.is_none()) {
            Err(())
        } else {
            Ok(DateTime { date, time, offset })
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(date) = self.date {
            write!(f, "{}", date)?;
        }

        if let Some(time) = self.time {
            if self.date.is_some() {
                write!(f, "T")?;
            }
            write!(f, "{}", time)?;
        }

        if let Some(offset) = self.offset {
            write!(f, "{}", offset)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub(crate) fn new(year: u16, month: u8, day: u8) -> Result<Self, ()> {
        if year == 0 || day == 0 {
            return Err(());
        }

        let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);

        let max_day = match month {
            2 if !leap_year => 28,
            2 => 29,
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => return Err(()),
        };

        if day <= max_day {
            Ok(Date { year, month, day })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl Time {
    pub(crate) fn new(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result<Self, ()> {
        // Consider leap seconds
        if hour <= 24 && minute <= 59 && second <= 60 && nanosecond < 10u32.pow(10) {
            Ok(Time {
                hour,
                minute,
                second,
                nanosecond,
            })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)?;

        if self.nanosecond != 0 {
            write!(
                f,
                ".{}",
                format!("{:09}", self.nanosecond).trim_end_matches('0')
            )?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Offset {
    hour: i8,
    minute: u8,
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hour == 0 && self.minute == 0 {
            write!(f, "Z")
        } else {
            write!(f, "{:+03}:{:02}", self.hour, self.minute)
        }
    }
}

impl Offset {
    pub(crate) fn new(hour: i8, minute: u8) -> Result<Self, ()> {
        if (-12 <= hour && hour <= 13 && minute < 60) || (hour == 14 && minute == 0) {
            Ok(Offset { hour, minute })
        } else {
            Err(())
        }
    }
}
