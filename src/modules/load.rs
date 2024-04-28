use std::{fs::File, io::BufReader};
use serenity::json;

use crate::Config;

pub fn load_config() -> Result<Config, &'static str>
{
    let file = File::open("config.json");

    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let config : Config = json::from_reader(reader).unwrap();
            return Ok(config);
        }
        Err(_) => {return Err("Failed to read/parse config.json");}
    }
}