use clap::Parser;

use crate::commands::{Opts, Command};
use crate::database::{create_record_table, get_connection};

mod commands;
mod record;
mod database;
mod utils;
mod config;

fn main() {
    let connection = get_connection();
    create_record_table(&connection);

    match Opts::parse().command {
        Command::Start(command) => {
            command.invoke(&connection)
        }
        Command::Stop(command) => {
            command.invoke(&connection)
        }
        Command::Show(command) => {
            command.invoke(&connection);
        }
        Command::Watch(command) => {
            command.invoke(&connection);
        }
        Command::Projects(command) => {
            command.invoke(&connection);
        }
        Command::Delete(command) => {
            command.invoke(&connection)
        }
    }
}