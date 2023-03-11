use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub watcher_timeout_sec: i32,
    pub time_unit: String,
}

pub fn get_config() -> Config {
    Config::from_config_file("Config.toml").unwrap()
}