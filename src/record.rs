use uuid::Uuid;

use crate::utils::now_timestamp_ms;

#[derive(Debug)]
pub struct Record {
    pub id: String,
    pub project: String,
    pub is_start: bool,
    pub time_at: i64,
}

impl Record {
    pub fn start(project: String) -> Record {
        return Record {
            id: Uuid::new_v4().to_string(),
            project,
            is_start: true,
            time_at: now_timestamp_ms(),
        };
    }
    pub fn stop(project: String) -> Record {
        return Record {
            id: Uuid::new_v4().to_string(),
            project,
            is_start: false,
            time_at: now_timestamp_ms(),
        };
    }
}