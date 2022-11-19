use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server, EndpointExt, middleware::Tracing};
use std::{env, fs};
use std::io::Error;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use poem::web::Html;

const WEBPAGE_PATH: &str = "index.html";

lazy_static!{
    static ref WEBPAGE: Arc<Mutex<String>> = Arc::new(Mutex::new({
        fs::read_to_string(WEBPAGE_PATH).expect("Failed to read file.")
    }));
}

#[handler]
fn hello() -> impl IntoResponse {
    let webpage_arc = WEBPAGE.clone();
    let x = match webpage_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner()
    }.to_owned();
    Html(x)
}

pub async fn init() {
    start().await.expect("TODO: panic message");
}

async fn start() -> Result<(), Error> {
    let app = Route::new().at("*", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("hello-world")
        .run(app)
        .await
}