use once_cell::sync::Lazy;
use std::{collections::HashMap, fs};
use toml::Value;

use crate::{command::CommandStructure, os_info};

static ALIASES: Lazy<HashMap<String, Value>> = Lazy::new(|| {
    let filename = os_info::current_alias_path();
    create_file_if_not_exists(&filename);
    let file_as_string = fs::read_to_string(&filename);
    let parsed_toml: Value = toml::from_str(&file_as_string.unwrap()).unwrap();
    return value_to_hashmap(&parsed_toml["aliases"]).unwrap();
});

fn create_file_if_not_exists(filename: &str) {
    let data = fs::read(format!("{}", filename));
    match data {
        Ok(_) => {}
        Err(_) => {
            fs::File::create(filename).expect("FATAL ERROR");
        }
    }
}

pub fn check_if_alias(command: &String) -> bool {
    match ALIASES.get(command) {
        Some(_) => {
            return true;
        }
        None => {
            return false;
        }
    }
}

pub fn get_real_command(command: &CommandStructure) -> String {
    let mut real_command = String::new();
    let joined_args = command.args.join(" ");
    real_command.push_str(ALIASES.get(&command.cmd).unwrap().as_str().unwrap());
    real_command.push_str(&joined_args);
    return real_command;
}

fn value_to_hashmap(value: &Value) -> Option<HashMap<String, Value>> {
    match value {
        Value::Table(table) => {
            let mut result = HashMap::new();
            for (key, val) in table.iter() {
                result.insert(key.to_string(), val.clone());
            }
            Some(result)
        }
        _ => None,
    }
}
