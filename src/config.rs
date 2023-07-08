use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub watcher_timeout_sec: i32,
    pub time_unit: String,
}

pub fn get_config() -> Config {
    let config_str = include_str!("../Config.toml");
    toml::from_str(config_str).unwrap()
}