use crate::actors::chat_server::ChatServer;
use actix::Addr;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use uuid::Uuid;

pub type SessionId = Uuid;
pub type RoomId = Uuid;

pub struct AppState {
    pub chat: Addr<ChatServer>,
}

pub struct UserInfo {
    pub nickname: String,
}

impl Default for UserInfo {
    fn default() -> Self {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect();

        let nickname = format!("User-{}", rand_string);
        
        UserInfo { nickname }
    }
}
