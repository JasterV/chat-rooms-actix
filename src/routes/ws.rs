use crate::{actors::chat_session::WsChatSession, models::AppState};
use actix_web::{get, web, HttpRequest, Responder};
use actix_web_actors::ws;

#[get("/ws")]
pub async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> impl Responder {
    let chat = state.chat.clone();
    ws::start(WsChatSession::new(chat), &req, stream)
}
