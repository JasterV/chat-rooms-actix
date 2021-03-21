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

    let port = std::env::var("PORT").expect("PORT env variable not found");
    let addr = format!("127.0.0.1:{}", port);

    let chat = ChatServer::new().start();
    HttpServer::new(move || {
        App::new()
            .data(AppState { chat: chat.clone() })
            .service(connect)
    })
    .bind(&addr)?
    .run()
    .await
}
