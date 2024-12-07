use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub bind_address: String,
    pub paste_limit: usize,
}

impl Config {
    pub fn open_config() -> Config {
        let config = fs::read_to_string("config.toml").unwrap();
        return toml::from_str(&config).unwrap();
    }

    fn checker(config: Config) -> Config {
        return config;
    }
}
