use std::path::Path;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::thread;

use chrono::{DateTime, Utc};
use clap::Parser;
use notify::{RecursiveMode, Watcher};
use rusqlite::Connection;

use crate::database::{delete_project, insert_record, read_project_records, read_projects};
use crate::record::Record;
use crate::utils::{date_time_to_display_date, is_seconds_passed, now_timestamp_ms, timestamp_to_date_time_utc};

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(about = "Starts tracking time for a unique project.")]
    Start(StartCommand),
    #[command(about = "Stops tracking time for a unique project.")]
    Stop(StopCommand),
    #[command(about = "Show tracked times for all projects.")]
    Show(ShowCommand),
    #[command(about = "Track time by watching directory changes.")]
    Watch(WatchCommand),
    #[command(about = "List all projects.")]
    Projects(ProjectsCommand),
    #[command(about = "Delete project.")]
    Delete(DeleteCommand),
}

#[derive(Parser, Debug)]
pub struct StartCommand {
    #[arg(short, long)]
    pub project: String,
}

impl StartCommand {
    pub fn invoke(self, connection: &Connection) {
        let record = Record::start(self.project);
        insert_record(&connection, record);
    }
}

#[derive(Parser, Debug)]
pub struct StopCommand {
    #[arg(short, long)]
    pub project: String,
}

impl StopCommand {
    pub fn invoke(self, connection: &Connection) {
        let record = Record::stop(self.project);
        insert_record(&connection, record);
    }
}

#[derive(Parser, Debug)]
pub struct ShowCommand {
    #[arg(short, long)]
    pub project: String,
}

impl ShowCommand {
    pub fn invoke(self, connection: &Connection) {
        println!("{:18} {:18} {:12} {:5}", "Started (UTC)", "Stopped (UTC)", "Project", "Duration (min)");
        let records = read_project_records(&connection, self.project.clone()).unwrap();
        let mut total_duration_min = 0;
        let mut start_date: Option<DateTime<Utc>> = None;
        for record in records.into_iter() {
            // Total length of a single session is initial start time until fist end time. This
            // means that if there are consecutive start times, only first one will be taken into
            // consideration.
            if record.is_start {
                if start_date.is_none() {
                    let start_time_at = timestamp_to_date_time_utc(record.time_at);
                    start_date = Some(start_time_at);
                }
            } else if start_date.is_some() {
                let end_time_at = timestamp_to_date_time_utc(record.time_at);
                let start_time_at_fmt = format!("{:18}", date_time_to_display_date(&start_date.unwrap()));
                let end_time_at_fmt = format!("{:18}", date_time_to_display_date(&end_time_at));
                let mut project_trunc = self.project.clone();
                project_trunc.truncate(12);
                let project_fmt = format!("{:12}", project_trunc);
                let duration = end_time_at - start_date.unwrap();
                let duration_min = duration.num_minutes();
                let duration_fmt = format!("{:5}", duration_min);
                println!("{} {} {} {}", start_time_at_fmt, end_time_at_fmt, project_fmt, duration_fmt);
                start_date = None;
                total_duration_min += duration_min;
            }
        }
        println!("------------------");
        println!("Total: {} min", total_duration_min);
    }
}

#[derive(Parser, Debug)]
pub struct WatchCommand {
    #[arg(short, long)]
    pub project: String,
    #[arg(long)]
    pub path: String,
}

impl WatchCommand {
    pub fn invoke(self, connection: &Connection) {
        println!("Watching project '{}', in path '{}'", self.project, self.path);

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        // Allows termination of while loop and execution of the rest of the code in this function.
        ctrlc::set_handler(move || {
            running_clone.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
        // Flag that connects watcher thread and while loop in order to record new log.
        let start = Arc::new(AtomicBool::new(false));
        let start_clone = start.clone();
        let started_timestamp = Arc::new(AtomicI64::new(0));
        let started_timestamp_clone = started_timestamp.clone();
        // Watches for any events in a given path.
        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(_) => {
                    started_timestamp.store(now_timestamp_ms(), Ordering::SeqCst);
                    start.store(true, Ordering::SeqCst);
                }
                Err(e) => println!("Watch error: {:?}", e),
            }
        }).unwrap();
        let path = Path::new(&self.path);
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
        // Writes start and stop logs in case of an event.
        let mut working: bool = false;
        while running.load(Ordering::SeqCst) {
            if start_clone.load(Ordering::SeqCst) {
                start_clone.store(false, Ordering::SeqCst);
                if !working {
                    working = true;
                    println!("Activity detected..");
                    let record = Record::start(self.project.clone());
                    insert_record(connection, record);
                }
            }
            let last_timestamp = started_timestamp_clone.load(Ordering::SeqCst);
            if working && is_seconds_passed(120, last_timestamp) {
                start_clone.store(false, Ordering::SeqCst);
                working = false;
                println!("Paused (inactive)..");
                let record = Record::stop(self.project.clone());
                insert_record(connection, record);
            }
            thread::sleep(core::time::Duration::from_millis(500));
        }
        // Will run if user terminates a program with Ctrl+C.
        let record = Record::stop(self.project);
        insert_record(connection, record);
    }
}

#[derive(Parser, Debug)]
pub struct ProjectsCommand;

impl ProjectsCommand {
    pub fn invoke(self, connection: &Connection) {
        println!("{:<5}{:<3}", "", "Projects");
        let projects = read_projects(&connection).unwrap();
        for (index, project) in projects.into_iter().enumerate() {
            println!("{:?}.{:<3}{:<3}", index + 1, "", project);
        }
    }
}

#[derive(Parser, Debug)]
pub struct DeleteCommand {
    #[arg(short, long)]
    pub project: String,
}

impl DeleteCommand {
    pub fn invoke(self, connection: &Connection) {
        delete_project(connection, self.project);
        println!("Deleted project");
    }
}