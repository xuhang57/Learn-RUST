use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // ...
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ...
}