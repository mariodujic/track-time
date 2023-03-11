use crate::utils::{get_uuid, now_timestamp_ms};

#[derive(Debug)]
pub struct TrackingEntry {
    pub id: String,
    pub project: String,
    pub is_start: bool,
    pub time_at: i64,
}

impl TrackingEntry {
    pub fn start(project: String) -> TrackingEntry {
        return TrackingEntry {
            id: get_uuid(),
            project,
            is_start: true,
            time_at: now_timestamp_ms(),
        };
    }
    pub fn stop(project: String) -> TrackingEntry {
        return TrackingEntry {
            id: get_uuid(),
            project,
            is_start: false,
            time_at: now_timestamp_ms(),
        };
    }
}