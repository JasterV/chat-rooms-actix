mod actors;
mod constants;
mod messages;
mod models;
mod routes;

use crate::{actors::chat_server::ChatServer, models::AppState};
use actix::Actor;
use actix_web::{middleware, App, HttpServer};
use routes::connect;

fn get_server_addr() -> String {
    let port = std::env::var("PORT").expect("PORT env variable not found");
    format!("0.0.0.0:{}", port)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let addr = get_server_addr();
    let chat = ChatServer::new().start();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(AppState { chat: chat.clone() })
            .service(connect)
    })
    .bind(&addr)?
    .run()
    .await
}
