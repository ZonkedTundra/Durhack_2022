use std::{env, fs};
use std::io::Error;
use std::sync::{Arc, Mutex};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use mongodb::{bson, Client, ClientSession, Database};
use mongodb::bson::{doc, Document};
use mongodb::options::{ClientOptions, Predicate};
use poem::{
    EndpointExt, get, handler, IntoResponse, listener::TcpListener, middleware::Tracing, Route,
    Server, web::Path,
};
use poem::web::{Html, Redirect};

use crate::{Config, CONFIG};

lazy_static!(
    static ref DATABASE: Arc<Mutex<Option<Database>>> = Arc::new(Mutex::new(Option::None));
);

pub async fn init(conf_arc: Arc<Mutex<Config>>) -> Result<(), String> {
    let config = match conf_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };

    let Ok(client_options) =
        ClientOptions::parse(format!("{}?tls=true&tlsCertificateKeyFile={}&authMechanism=MONGODB-X509&authSource=%24external",
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

    let db = client.database("compiling");
    db.create_collection("players", None);

    let database_arc = DATABASE.clone();
    let mut database = match database_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };
    *database = Option::from(db);
    drop(database);
    
    Ok(())
}

pub async fn balance_get(token: &str) -> i32 {
    if let Some(player) = get_player(token).await {
        player.get("balance").unwrap().as_i32().unwrap()
    } else {
        -1
    }
}

pub async fn balance_set(token: &str, value: i32) {
    let database_arc = DATABASE.clone();
    let mut db = match database_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    }.to_owned().unwrap();
    db.collection::<Document>("players").update_one(doc! {"token": {"$eq": token}}, doc! {"$set": {"balance": value}}, None).await.expect("TODO: panic message");
    drop(db);
}

pub async fn balance_add(token: &str, value: i32) {
    balance_set(token, balance_get(token).await + value).await;
}

pub async fn balance_sub(token: &str, value: i32) -> bool{
    let prev:i32 = balance_get(token).await;
    if prev < value {
        return false;
    }
    // race conditions galore
    balance_set(token, prev - value).await;
    true
}

pub async fn get_player(token: &str) -> Option<Document> {
    let database_arc = DATABASE.clone();
    let db = match database_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    }.to_owned().unwrap();
    db.collection::<Document>("players").find_one(doc! {"token": {"$eq": token}}, None).await.unwrap()
}