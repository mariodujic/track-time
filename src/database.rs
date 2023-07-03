use std::fs::create_dir_all;

use rusqlite::{Connection, Error, Row};

use crate::tracking_entry::TrackingEntry;

pub fn get_connection() -> Connection {
    create_dir_all("database").unwrap();
    Connection::open("database/track_time.db").unwrap()
}

pub fn create_record_table(connection: &Connection) {
    let query = "CREATE TABLE IF NOT EXISTS records (id TEXT PRIMARY KEY, project TEXT, is_start INTEGER, time_at INTEGER);";
    connection.execute(query, ()).unwrap();
}

pub fn insert_record(connection: &Connection, record: TrackingEntry) {
    let TrackingEntry { id, project, is_start, time_at } = record;
    connection.execute(
        "INSERT INTO records (id, project, is_start, time_at) VALUES (?1, ?2, ?3, ?4)",
        (&id, &project, is_start, time_at),
    ).unwrap();
}

pub fn delete_project(connection: &Connection, project: String) {
    connection.execute(
        "DELETE FROM records WHERE project = ?",
        [project],
    ).unwrap();
}

pub fn rename_project(connection: &Connection, project: String, new_project: String) {
    connection.execute(
        "UPDATE records SET project = ? WHERE project = ?",
        [new_project, project],
    ).unwrap();
}

pub fn read_project_records(connection: &Connection, project: String) -> Result<Vec<TrackingEntry>, Error> {
    let mut statement = connection.prepare("SELECT id, project, is_start, time_at FROM records WHERE project = :project ORDER BY time_at ASC")?;
    let result = statement.query_map(&[(":project", &project)], |row| { row_to_record(row) })?;
    result.collect()
}

fn row_to_record(row: &Row) -> Result<TrackingEntry, Error> {
    Ok(TrackingEntry {
        id: row.get(0)?,
        project: row.get(1)?,
        is_start: row.get(2)?,
        time_at: row.get(3)?,
    })
}

pub fn read_projects(connection: &Connection) -> Result<Vec<String>, Error> {
    let mut statement = connection.prepare("SELECT DISTINCT project FROM records")?;
    let result = statement.query_map([], |row| row.get(0))?;
    result.collect()
}