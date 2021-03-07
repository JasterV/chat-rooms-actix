use actix::{Actor, Context, Handler, MessageResult, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::models::{
    messages::chat_server::{Connect, Disconnect, Message},
    RoomId, SessionId,
};

pub struct ChatServer {
    sessions: HashMap<SessionId, Recipient<Message>>,
    rooms: HashMap<RoomId, HashSet<SessionId>>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    pub fn send_message(&self, room: &Uuid, message: &'static str, skip_id: &Uuid) {
        self.rooms.get(room).map(|sessions| {
            sessions.iter().for_each(|id| {
                if id != skip_id {
                    self.sessions
                        .get(id)
                        .map(|addr| addr.do_send(Message(message)));
                }
            });
        });
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = MessageResult<Connect>;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let session_id = Uuid::new_v4();
        self.sessions.insert(session_id, msg.addr);
        MessageResult(session_id)
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(
        &mut self,
        Disconnect { session }: Disconnect,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        for (_id, sessions) in self.rooms.iter_mut() {
            sessions.remove(&session);
        }
        let _ = self.sessions.remove(&session);
    }
}
