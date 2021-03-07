use actix::{Message as ActixMessage, Recipient};
use uuid::Uuid;

use crate::models::{RoomId, SessionId};

use super::chat_session::Message;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub session: SessionId,
    pub room: RoomId,
    pub msg: String,
}

#[derive(ActixMessage)]
#[rtype(result = "Uuid")]
pub struct CreateRoom {
    pub session: SessionId,
}
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub session: SessionId,
    pub room: RoomId,
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
