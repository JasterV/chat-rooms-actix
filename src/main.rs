mod actors;
mod constants;
mod messages;
mod models;
mod routes;

use crate::{actors::chat_server::ChatServer, models::AppState};
use actix::Actor;
use actix_web::{App, HttpServer};
use routes::connect;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat = ChatServer::new().start();
    HttpServer::new(move || {
        App::new()
            .data(AppState { chat: chat.clone() })
            .service(connect)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
