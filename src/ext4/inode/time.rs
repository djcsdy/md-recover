use chrono::{DateTime, Duration, Utc};

pub fn decode_extra_time(base: u32, extra: u32) -> DateTime<Utc> {
    let seconds = i64::from(base) + ((i64::from(extra) & 3) << 32);
    let nanos = i64::from(extra) >> 2;
    DateTime::UNIX_EPOCH + Duration::seconds(seconds) + Duration::nanoseconds(nanos)
}
