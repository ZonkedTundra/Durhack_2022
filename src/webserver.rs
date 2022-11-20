use std::{env, fs, thread};
use std::borrow::ToOwned;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use futures_util::future::err;
use lazy_static::lazy_static;
use poem::{get, handler, IntoResponse, Route, Server, web::websocket::{Message, WebSocket}};
use poem::listener::TcpListener;
use poem::web::{Data, Html, Redirect};
use poem::web::websocket::WebSocketStream;
use tokio::io::AsyncBufReadExt;
use tokio::time;
use tokio::time::interval;

use crate::{Config, CONFIG};
use crate::system::handle;

const WEBPAGE_PATH: &str = "web_scenes/index.html";

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
    drop(webpage);
    Redirect::see_other("/")
}

#[handler]
async fn create_websocket(ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
            while let Some(Ok(res)) = socket.next().await {
                            match res {
                                Message::Text(text) => {
                                    let st = handle(text);
                                    println!("{}", st);
                                    socket.send(Message::Text(st)).await;
                                }
                                Message::Binary(_) => {}
                                Message::Ping(_) => {}
                                Message::Pong(_) => {}
                                Message::Close(_) => {}
                            }
                        }

    })
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
    let app = Route::new()
        .at("/ws", get(create_websocket))
        .at("/reload", get(reload))
        .at("*", get(serve));
        //.at("/ws/:name", get(ws.data(tokio::sync::broadcast::channel::<String>(32).0)));
        ;
    Server::new(TcpListener::bind(address.to_owned()))
        .name("hello-world")
        .run(app)
        .await
}
