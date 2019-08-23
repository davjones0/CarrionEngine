use std::fs::File;
use std::io::prelude::*;
use toml::{ Value, Deserializer };
use toml;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub hello: String
}

impl Config {
    pub fn new() -> Config {
        Config {
            hello: String::from("welp")
        }
    }

    pub fn parse(path: String) -> Config {
        let mut config_toml = String::new();

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                //error!("could not find config file. Using default!");
                return Config::new();
            }
        };

        file.read_to_string(&mut config_toml).unwrap_or_else(|err| panic!("error while reading config [{}]", err));
        let file_info: Config = toml::from_str(&config_toml).unwrap();

        return file_info;
    }
}