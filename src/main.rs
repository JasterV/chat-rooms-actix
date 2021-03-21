mod actors;
mod constants;
mod messages;
mod models;
mod routes;

use crate::{actors::chat_server::ChatServer, models::AppState};
use actix::Actor;
use actix_web::{App, HttpServer};
use routes::connect;

fn get_server_addr() -> String {
    let port = std::env::var("PORT").expect("PORT env variable not found");
    format!("0.0.0.0:{}", port)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = get_server_addr();
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
