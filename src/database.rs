use std::{env, fs};
use std::io::Error;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::options::ClientOptions;
use poem::{
    EndpointExt, get, handler, IntoResponse, listener::TcpListener, middleware::Tracing, Route,
    Server, web::Path,
};
use poem::web::{Html, Redirect};

use crate::{Config, CONFIG};

pub async fn init(conf_arc: Arc<Mutex<Config>>) -> Result<(), String> {
    let config = match conf_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };

    let Ok(client_options) =
        ClientOptions::parse(format!("{}?tls=true1&tlsCertificateKeyFile={}&authMechanism=MONGODB-X509&authSource=%24external",
                                     config.mongo_connection_string,
                                     config.mongo_connection_cert_file)).await else {
        return Err("Uh oh! Database bad config!".to_owned());
    };

    let Ok(client) = Client::with_options(client_options) else {
        return Err("Uh oh! Database no connection!".to_owned());
    };

    print!("Connecting to the database...");
    if let Err(_) = client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
    {
        return Err("Uh oh! Failed to ping the database!".to_owned());
    }
    println!(" Connected!");
    Ok(())
}
