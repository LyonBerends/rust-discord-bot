use std::{fs::File, io::BufReader};
use serenity::json;

use crate::Config;

pub fn load_config() -> Result<Config, &'static str>
{
    let file = File::open("config.json");

    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            match json::from_reader(reader) {
                Ok(config) => {Ok(config)}
                Err(_) => {Err("Something wen't wrong while trying to parse JSON from config.json")}
            }
        }
        Err(_) => {Err("There is no config.json file")}
    }
}