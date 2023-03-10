use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::format::{DelayedFormat, StrftimeItems};

pub fn timestamp_to_date_time_utc(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    DateTime::from_utc(naive, Utc)
}

pub fn date_time_to_display_date(data_time: &DateTime<Utc>) -> DelayedFormat<StrftimeItems> {
    data_time.format("%b %e %Y %H:%M")
}