mod actors;
mod messages;
mod models;
mod routes;
mod server;

use crate::{actors::chat_server::ChatServer, models::AppState, server::init};
use actix::Actor;
use actix_web::{App, HttpServer};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat = ChatServer::new().start();

    HttpServer::new(move || {
        App::new()
            .data(AppState { chat: chat.clone() })
            .configure(init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
