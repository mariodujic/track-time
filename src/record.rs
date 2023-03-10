use uuid::Uuid;

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
            time_at: chrono::offset::Utc::now().timestamp_millis(),
        };
    }
    pub fn stop(project: String) -> Record {
        return Record {
            id: Uuid::new_v4().to_string(),
            project,
            is_start: false,
            time_at: chrono::offset::Utc::now().timestamp_millis(),
        };
    }
}