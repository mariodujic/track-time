use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::format::{DelayedFormat, StrftimeItems};
use uuid::Uuid;

pub fn timestamp_to_date_time_utc(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    DateTime::from_utc(naive, Utc)
}

pub fn date_time_to_display_date(data_time: &DateTime<Utc>) -> DelayedFormat<StrftimeItems> {
    data_time.format("%b %e %Y %H:%M")
}

pub fn now_timestamp_ms() -> i64 {
    return Utc::now().timestamp_millis();
}

pub fn is_seconds_passed(required_time_sec: i32, timestamp: i64) -> bool {
    let now = now_timestamp_ms();
    let elapsed_time = now - timestamp;
    elapsed_time >= (required_time_sec * 1000) as i64
}

pub fn get_uuid() -> String {
    Uuid::new_v4().to_string()
}