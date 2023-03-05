use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Start(StartCommand),
    Stop(StopCommand),
    Show(ShowCommand),
    Watch(WatchCommand),
}

#[derive(Parser, Debug)]
#[command(about = "Starts tracking time for a unique project")]
pub struct StartCommand {
    #[arg(short, long)]
    pub project: String,
}

#[derive(Parser, Debug)]
#[command(about = "Stops tracking time for a unique project")]
pub struct StopCommand {
    #[arg(short, long)]
    pub project: String,
}

#[derive(Parser, Debug)]
#[command(about = "Show tracked times for all projects")]
pub struct ShowCommand {
    #[arg(short, long)]
    pub project: String,
}

#[derive(Parser, Debug)]
#[command(about = "Track time by watching directory changes")]
pub struct WatchCommand {
    #[arg(short, long)]
    pub project: String,
    #[arg(long)]
    pub path: String,
}