use clap::Parser;

use crate::cli::{Opts, SubCommand};
use crate::database::{create_record_table, get_connection, insert_record, read_project_records};
use crate::record::Record;

mod cli;
mod record;
mod database;

fn main() {
    let connection = get_connection();
    create_record_table(&connection);

    match Opts::parse().sub_cmd {
        SubCommand::Start(param) => {
            let record = Record::start(param.project);
            insert_record(&connection, record);
        }
        SubCommand::Stop(param) => {
            let record = Record::start(param.project);
            insert_record(&connection, record);
        }
        SubCommand::Show(param) => {
            println!("{:?}", read_project_records(&connection, param.project));
        }
        SubCommand::Watch(param) => {
            println!("Project: {}", param.project);
            println!("Path: {}", param.path);
        }
    }
}