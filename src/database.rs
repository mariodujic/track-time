use std::fs::create_dir_all;

use rusqlite::{Connection, Error, Row};

use crate::record::Record;

pub fn get_connection() -> Connection {
    create_dir_all("database").unwrap();
    Connection::open("database/track_time.db").unwrap()
}

pub fn create_record_table(connection: &Connection) {
    let query = "CREATE TABLE IF NOT EXISTS records (id TEXT PRIMARY KEY, project TEXT, is_start INTEGER, time_at INTEGER);";
    connection.execute(query, ()).unwrap();
}

pub fn insert_record(connection: &Connection, record: Record) {
    let Record { id, project, is_start, time_at } = record;
    connection.execute(
        "INSERT INTO records (id, project, is_start, time_at) VALUES (?1, ?2, ?3, ?4)",
        (&id, &project, is_start, time_at),
    ).unwrap();
}

pub fn read_project_records(connection: &Connection, project: String) -> Result<Vec<Record>, Error> {
    let mut statement = connection.prepare("SELECT id, project, is_start, time_at FROM records WHERE project = :project ORDER BY time_at ASC")?;
    let result = statement.query_map(&[(":project", &project)], |row| { row_to_record(row) })?;
    result.collect()
}

fn row_to_record(row: &Row) -> Result<Record, Error> {
    Ok(Record {
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