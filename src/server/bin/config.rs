use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub bind_address: String,
}

impl Config {
    pub fn open_config() -> Config {
        let config = fs::read_to_string("config.toml").unwrap();
        toml::from_str(&config).unwrap()
    }
}
