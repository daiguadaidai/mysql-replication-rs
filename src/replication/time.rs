use chrono::{NaiveDateTime, TimeZone};
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};

pub const FRAC_TIME_FORMAT_6: &str = "%Y-%m-%d %H:%M:%S.%6f";
#[derive(Debug, Clone, PartialEq)]
pub struct FracTime {
    pub f_time: NaiveDateTime,

    // Dec must in [0, 6]
    pub dec: isize,

    pub timestamp_string_location: Option<chrono_tz::Tz>,
}

impl Display for FracTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut time = if let Some(timezone) = self.timestamp_string_location {
            timezone
                .from_local_datetime(&self.f_time)
                .unwrap()
                .format(FRAC_TIME_FORMAT_6)
                .to_string()
        } else {
            self.f_time.format(FRAC_TIME_FORMAT_6).to_string()
        };

        time = if self.dec == 0 {
            time.split(".").collect::<Vec<&str>>()[0].to_string()
        } else if self.dec <= 5 {
            time[..time.len() - (6 - self.dec as usize)].to_string()
        } else {
            time
        };

        write!(f, "{}", time)
    }
}

impl Serialize for FracTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub fn format_zero_time(frac: isize, dec: isize) -> String {
    if dec == 0 {
        return String::from("0000-00-00 00:00:00");
    }

    let s = format!("0000-00-00 00:00:00.{:06}", frac);
    // dec must < 6, if frac is 924000, but dec is 3, we must output 924 here.
    s[..s.len() - (6 - dec as usize)].to_string()
}

pub fn format_before_unix_zero_time(
    year: isize,
    month: isize,
    day: isize,
    hour: isize,
    minute: isize,
    second: isize,
    frac: isize,
    dec: isize,
) -> String {
    if dec == 0 {
        return format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month, day, hour, minute, second
        );
    }

    let s = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
        year, month, day, hour, minute, second, frac
    );
    s[..s.len() - (6 - dec as usize)].to_string()
}

pub fn micro_sec_timestamp_to_time(ts: u64) -> NaiveDateTime {
    if ts == 0 {
        return NaiveDateTime::from_timestamp_opt(0, 0).unwrap();
    }

    let ts = ts as i64;
    return NaiveDateTime::from_timestamp_opt(ts / 1000000, (ts % 1000000 * 1000) as u32).unwrap();
}
