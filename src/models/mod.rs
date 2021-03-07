pub mod commands;
pub mod ws;
use crate::actors::chat_server::ChatServer;
use actix::Addr;
use uuid::Uuid;

pub type SessionId = Uuid;
pub type RoomId = Uuid;

pub struct AppState {
    pub chat: Addr<ChatServer>,
}
