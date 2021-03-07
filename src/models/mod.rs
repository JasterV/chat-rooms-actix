use actix::Addr;
use uuid::Uuid;

use crate::actors::chat_server::ChatServer;

pub type SessionId = Uuid;
pub type RoomId = Uuid;

pub struct AppState {
    pub chat: Addr<ChatServer>,
}

