mod database;
mod simulation;

use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

/// The config format from config.toml
#[derive(Deserialize)]
struct Config {
    mongo_connection_string: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mongo_connection_string: "".to_owned(),
        }
    }
}

/// The state system
#[derive(Default)]
enum State {
    #[default]
    Lobby,
    Betting,
    Racing
}

lazy_static!{
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
    static ref STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::default()));
}

fn main() {
    let config_filename = "config.toml";
    
    let Ok(config_content) = fs::read_to_string(config_filename) else {
        println!("'{config_filename}' configuration file not found!");
        exit(1);
    };
    
    let Ok(config) = toml::from_str::<Config>(&config_content) else {
        println!("'{config_filename}' configuration is not valid!");
        exit(1);
    };
    
    let global_config_arc = CONFIG.clone();
    let mut global_config = match global_config_arc.lock() {
        Ok(global_config) => global_config,
        Err(global_config) => global_config.into_inner()
    };
    
    *global_config = config;
    
    println!("Hello, world!");
}
