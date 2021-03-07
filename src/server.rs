use crate::{actors::chat_server::ChatServer, models::AppState, routes::ws::connect};
use actix::Actor;
use actix_web::web;

pub fn init(app: &mut web::ServiceConfig) {
    let chat = ChatServer::new().start();

    app.data(AppState { chat })
        .service(web::resource("/ws/").to(connect));
}
