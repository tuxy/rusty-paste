use serde::Deserialize;
use std::fs;

// All things related to opening and parsing config.toml
#[derive(Deserialize, Clone)]
pub struct Config {
    pub bind_address: String,
    pub time_limit: u64,
}

impl Config {
    pub fn open_config() -> Config {
        let config = fs::read_to_string("config.toml").unwrap();
        check_config(config)
    }
}

// Check if config has been written correctly
fn check_config(file: String) -> Config {
    let config: Config = match toml::from_str(&file) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Failed to parse config file: {err}");
            panic!()
        }
    };
    config
}