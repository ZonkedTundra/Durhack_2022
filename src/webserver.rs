use std::{env, fs};
use std::io::Error;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use poem::{
    EndpointExt, get, handler, IntoResponse, listener::TcpListener, middleware::Tracing, Route,
    Server, web::Path,
};
use poem::web::{Html, Redirect};

use crate::{Config, CONFIG};

const WEBPAGE_PATH: &str = "index.html";

lazy_static! {
    static ref WEBPAGE: Arc<Mutex<String>> = Arc::new(Mutex::new({
        fs::read_to_string(WEBPAGE_PATH).expect("Failed to read file.")
    }));
}

#[handler]
fn serve() -> impl IntoResponse {
    let webpage_arc = WEBPAGE.clone();
    let webpage = match webpage_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    }
    .to_owned();
    Html(webpage)
}

#[handler]
fn reload() -> Redirect {
    let webpage_arc = WEBPAGE.clone();
    let mut webpage = match webpage_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };
    *webpage = fs::read_to_string(WEBPAGE_PATH).expect("Failed to read file.");

    Redirect::see_other("/")
}

pub async fn init(conf_arc: Arc<Mutex<Config>>) {
    let config = match conf_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };
    start(&config.webserver_address)
        .await
        .expect("TODO: panic message");
}

async fn start(address: &String) -> Result<(), Error> {
    let app = Route::new().at("/reload", get(reload)).at("*", get(serve));
    Server::new(TcpListener::bind(address.to_owned()))
        .name("hello-world")
        .run(app)
        .await
}
