mod actors;
mod constants;
mod messages;
mod models;
mod routes;

use crate::{actors::chat_server::ChatServer, models::AppState};
use actix::Actor;
use actix_web::{App, HttpServer, get};
use routes::connect;

#[get("/hi")]
async fn hi() -> &'static str {
    "Hello, World!"
}

fn get_server_addr() -> String {
    let port = std::env::var("PORT").expect("PORT env variable not found");
    format!("127.0.0.1:{}", port)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = get_server_addr();
    let chat = ChatServer::new().start();
    HttpServer::new(move || {
        App::new()
            .data(AppState { chat: chat.clone() })
            .service(connect)
            .service(hi)
    })
    .bind(&addr)?
    .run()
    .await
}
