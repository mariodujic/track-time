use clap::Parser;

use crate::commands::{Opts, SubCommand};
use crate::commands::SubCommand::Show;
use crate::database::{create_record_table, get_connection};

mod commands;
mod record;
mod database;
mod utils;

fn main() {
    let connection = get_connection();
    create_record_table(&connection);

    match Opts::parse().sub_cmd {
        SubCommand::Start(command) => {
            command.invoke(&connection)
        }
        SubCommand::Stop(command) => {
            command.invoke(&connection)
        }
        Show(param) => {
            param.invoke(&connection);
        }
        SubCommand::Watch(command) => {
            println!("Project: {}", command.project);
            println!("Path: {}", command.path);
        }
        SubCommand::Projects(command) => {
            command.invoke(&connection);
        }
    }
}