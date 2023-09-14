#[cfg(test)]
mod tests {
    use crate::replication::{format_zero_time, FracTime};
    use chrono::{FixedOffset, NaiveDate, TimeZone};

    #[test]
    fn test_time() {
        struct Tbl {
            pub year: isize,
            pub month: isize,
            pub day: isize,
            pub hour: isize,
            pub min: isize,
            pub sec: isize,
            pub micro_sec: isize,
            pub frac: isize,
            pub expected: String,
        }
        let tbls = vec![
            Tbl {
                year: 2000,
                month: 1,
                day: 1,
                hour: 1,
                min: 1,
                sec: 1,
                micro_sec: 1,
                frac: 0,
                expected: "2000-01-01 01:01:01".to_string(),
            },
            Tbl {
                year: 2000,
                month: 1,
                day: 1,
                hour: 1,
                min: 1,
                sec: 1,
                micro_sec: 1,
                frac: 1,
                expected: "2000-01-01 01:01:01.0".to_string(),
            },
            Tbl {
                year: 2000,
                month: 1,
                day: 1,
                hour: 1,
                min: 1,
                sec: 1,
                micro_sec: 1,
                frac: 6,
                expected: "2000-01-01 01:01:01.000001".to_string(),
            },
        ];

        for t in &tbls {
            let naive_date = NaiveDate::from_ymd_opt(t.year as i32, t.month as u32, t.day as u32)
                .unwrap()
                .and_hms_micro_opt(
                    t.hour as u32,
                    t.min as u32,
                    t.sec as u32,
                    t.micro_sec as u32,
                )
                .unwrap();
            let t1 = FracTime {
                f_time: naive_date,
                dec: t.frac,
                timestamp_string_location: None,
            };
            assert_eq!(t.expected, t1.to_string())
        }

        struct ZeroTbl {
            pub frac: isize,
            pub dec: isize,
            pub expected: String,
        }
        let zero_tbls = vec![
            ZeroTbl {
                frac: 0,
                dec: 1,
                expected: "0000-00-00 00:00:00.0".to_string(),
            },
            ZeroTbl {
                frac: 1,
                dec: 1,
                expected: "0000-00-00 00:00:00.0".to_string(),
            },
            ZeroTbl {
                frac: 123,
                dec: 3,
                expected: "0000-00-00 00:00:00.000".to_string(),
            },
            ZeroTbl {
                frac: 123000,
                dec: 3,
                expected: "0000-00-00 00:00:00.123".to_string(),
            },
            ZeroTbl {
                frac: 123,
                dec: 6,
                expected: "0000-00-00 00:00:00.000123".to_string(),
            },
            ZeroTbl {
                frac: 123000,
                dec: 6,
                expected: "0000-00-00 00:00:00.123000".to_string(),
            },
        ];

        for t in &zero_tbls {
            assert_eq!(t.expected, format_zero_time(t.frac, t.dec))
        }
    }

    #[test]
    fn test_time_string_location() {
        let naive_datetime = NaiveDate::from_ymd_opt(2018, 7, 30)
            .unwrap()
            .and_hms_micro_opt(10, 0, 0, 0)
            .unwrap();
        let offset = FixedOffset::east_opt(-5 * 3600).unwrap();
        let datetime = offset.from_local_datetime(&naive_datetime).unwrap();
        let t = FracTime {
            f_time: datetime.naive_local(),
            dec: 0,
            timestamp_string_location: None,
        };
        assert_eq!("2018-07-30 10:00:00", t.to_string());

        let naive_datetime = NaiveDate::from_ymd_opt(2018, 7, 30)
            .unwrap()
            .and_hms_micro_opt(10, 0, 0, 0)
            .unwrap();
        let offset = FixedOffset::east_opt(-5 * 3600).unwrap();
        let datetime = offset.from_local_datetime(&naive_datetime).unwrap();
        let t = FracTime {
            f_time: datetime.naive_utc(),
            dec: 0,
            timestamp_string_location: Some(chrono_tz::Tz::UTC),
        };
        assert_eq!("2018-07-30 15:00:00", t.to_string());
    }

    #[test]
    fn d() {
        let offset = FixedOffset::east_opt(-5 * 3600).unwrap();
        let naive_datetime = NaiveDate::from_ymd_opt(2018, 7, 30)
            .unwrap()
            .and_hms_micro_opt(10, 0, 0, 0)
            .unwrap();
        let datetime = offset.from_local_datetime(&naive_datetime).unwrap();
        println!("{}", datetime);
        println!("{}", datetime.naive_local());

        let timezone = chrono_tz::Tz::UTC;
        let tz_datetime = timezone.from_utc_datetime(&naive_datetime);
        println!("{}", tz_datetime);
    }

    #[test]
    fn c() {
        let offset = FixedOffset::east_opt(-5 * 3600).unwrap();
        let naive_datetime = NaiveDate::from_ymd_opt(2018, 7, 30)
            .unwrap()
            .and_hms_micro_opt(10, 0, 0, 0)
            .unwrap();
        let datetime = offset.from_local_datetime(&naive_datetime).unwrap();
        println!("{}", datetime.format("%Y-%m-%d %H:%M:%S").to_string());
        println!(
            "{}",
            datetime
                .naive_local()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        );

        let timezone = chrono_tz::Tz::UTC;
        let datetime_in_timezone = datetime.with_timezone(&timezone);
        println!(
            "{}",
            datetime_in_timezone.format("%Y-%m-%d %H:%M:%S").to_string()
        );
    }
}
