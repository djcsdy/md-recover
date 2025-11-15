use chrono::{DateTime, Duration, Utc};

pub fn decode_extra_time(base: u32, extra: u32) -> DateTime<Utc> {
    let seconds = i64::from(base as i32) + ((i64::from(extra) & 3) << 32);
    let nanos = i64::from(extra >> 2);
    DateTime::UNIX_EPOCH + Duration::seconds(seconds) + Duration::nanoseconds(nanos)
}

#[cfg(test)]
mod test {
    use crate::ext4::inode::time::decode_extra_time;
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test() {
        assert_eq!(
            decode_extra_time(0x80000000, 0),
            NaiveDate::from_ymd_opt(1901, 12, 13)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(20, 45, 52).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0x80000000, 999999999 << 2),
            NaiveDate::from_ymd_opt(1901, 12, 13)
                .unwrap()
                .and_time(NaiveTime::from_hms_nano_opt(20, 45, 52, 999999999).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0xffffffff, 0),
            NaiveDate::from_ymd_opt(1969, 12, 31)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0xffffffff, 999999999 << 2),
            NaiveDate::from_ymd_opt(1969, 12, 31)
                .unwrap()
                .and_time(NaiveTime::from_hms_nano_opt(23, 59, 59, 999999999).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0, 0),
            NaiveDate::from_ymd_opt(1970, 1, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0x80000000, 1),
            NaiveDate::from_ymd_opt(2038, 1, 19)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(3, 14, 8).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0xffffffff, (999999999 << 2) | 1),
            NaiveDate::from_ymd_opt(2106, 2, 7)
                .unwrap()
                .and_time(NaiveTime::from_hms_nano_opt(6, 28, 15, 999999999).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0, 1),
            NaiveDate::from_ymd_opt(2106, 2, 7)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(6, 28, 16).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0x80000000, 2),
            NaiveDate::from_ymd_opt(2174, 2, 25)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(9, 42, 24).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0, 2),
            NaiveDate::from_ymd_opt(2242, 3, 16)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(12, 56, 32).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0x80000000, 3),
            NaiveDate::from_ymd_opt(2310, 4, 4)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(16, 10, 40).unwrap())
                .and_utc()
        );

        assert_eq!(
            decode_extra_time(0, 3),
            NaiveDate::from_ymd_opt(2378, 4, 22)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(19, 24, 48).unwrap())
                .and_utc()
        );
    }
}
