use serde::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ip_address : String,
}

impl Config {
    pub fn load_file(path: &str)->Result<(Self) , Box<dyn std::error::Error>>  {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    } 
}
