use std::fmt;

/// A struct represents date/time in TOML
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DateTime {
    date: Option<Date>,
    time: Option<Time>,
    offset: Option<Offset>,
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
struct Date {
    year: u16,
    month: u8,
    day: u8
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)?;

        if self.nanosecond != 0 {
            write!(f, ".{}", format!("{:09}", self.nanosecond).trim_end_matches('0'))?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Offset {
    hour: i8,
    minute: u8
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
