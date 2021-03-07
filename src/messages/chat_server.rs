use super::chat_session::Message;
use crate::models::SessionId;
use actix::{Message as ActixMessage, Recipient};
use uuid::Uuid;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub session: SessionId,
    pub room: String,
    pub msg: String,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub session: SessionId,
    pub room: String,
}

#[derive(ActixMessage)]
#[rtype(result = "Uuid")]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session: SessionId,
}
