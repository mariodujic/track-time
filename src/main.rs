use clap::Parser;

use crate::cli::{Opts, SubCommand};
use crate::record::Record;

mod cli;
mod record;

fn main() {
    match Opts::parse().sub_cmd {
        SubCommand::Start(param) => {
            let record = Record::start(param.project);
            println!("{:?}", record);
        }
        SubCommand::Stop(param) => {
            let record = Record::start(param.project);
            println!("{:?}", record);
        }
        SubCommand::Show(param) => {
            println!("Project: {}", param.project);
        }
    }
}