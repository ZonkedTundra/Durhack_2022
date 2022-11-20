
mod database;
mod simulation;
mod webserver;
mod system;
mod betting_maths;

use serde_derive::Deserialize;

use std::fs;
use std::process::exit;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use toml;


/// The config format from config.toml
#[derive(Deserialize)]
pub struct Config {
    mongo_connection_string: String,
    mongo_connection_cert_file: String,
    webserver_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mongo_connection_string: "".to_owned(),
            mongo_connection_cert_file: "./cert.pem".to_owned(),
            webserver_address: "127.0.0.1:3000".to_owned(),
        }
    }
}

/// The state system
#[derive(Default)]
enum State {
    #[default]
    Lobby,
    Betting,
    Racing,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
    static ref STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::default()));
}

#[tokio::main]
async fn main() {
    let config_filename = "config.toml";

    let Ok(config_content) = fs::read_to_string(config_filename) else {
        println!("'{config_filename}' configuration file not found!");
        return
    };

    let Ok(config) = toml::from_str::<Config>(&config_content) else {
        println!("'{config_filename}' configuration is not valid!");
        return;
    };

    let global_config_arc = CONFIG.clone();
    let mut global_config = match global_config_arc.lock() {
        Ok(global_config) => global_config,
        Err(global_config) => global_config.into_inner(),
    };

    *global_config = config;
    drop(global_config);

    if let Err(err) = database::init(CONFIG.clone()).await {
        println!("TODO: panic message");
        println!("{}", err.to_owned());
        return;
    };
    webserver::init(CONFIG.clone()).await;
}
