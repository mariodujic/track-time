use clap::Parser;

use crate::commands::{Opts, SubCommand};
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
        SubCommand::Show(command) => {
            command.invoke(&connection);
        }
        SubCommand::Watch(command) => {
            command.invoke(&connection);
        }
        SubCommand::Projects(command) => {
            command.invoke(&connection);
        }
        SubCommand::Delete(command) => {
            command.invoke(&connection)
        }
    }
}