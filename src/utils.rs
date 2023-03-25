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

#[cfg(test)]
mod tests {
    use crate::utils::{date_time_to_display_date, is_seconds_passed, now_timestamp_ms, timestamp_to_date_time_utc};

    #[test]
    fn should_return_timestamp() {
        let expected_date = "1971-10-08 20:00:00 UTC";
        let actual_date = &*timestamp_to_date_time_utc(55800000000).to_string();
        assert_eq!(expected_date, actual_date)
    }

    #[test]
    fn should_return_display_date() {
        let expected_display_date = "Oct  8 1971 20:00";
        let date_time = timestamp_to_date_time_utc(55800000000);
        let actual_display_date = &*date_time_to_display_date(&date_time).to_string();
        assert_eq!(expected_display_date, actual_display_date)
    }

    #[test]
    fn should_return_true_when_seconds_passed() {
        let timestamp = now_timestamp_ms() - 11_000;
        let passed = is_seconds_passed(10, timestamp);
        assert!(passed)
    }

    #[test]
    fn should_return_false_when_seconds_not_passed() {
        let timestamp = now_timestamp_ms() - 9_000;
        let passed = is_seconds_passed(10, timestamp);
        assert!(!passed)
    }
}
