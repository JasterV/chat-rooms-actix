use std::str::FromStr;

use crate::{
    actors::chat_server::ChatServer,
    commands::Command,
    messages::{
        chat_server::{ClientMessage, Connect, Disconnect},
        chat_session::Message,
    },
    models::{RoomId, SessionId},
};
use actix::{
    fut, ActorContext, ActorFuture, ContextFutureSpawner, Handler, Running, StreamHandler,
    WrapFuture,
};
use actix::{Actor, Addr, AsyncContext};
use actix_web_actors::ws::{self, WebsocketContext};

pub struct WsChatSession {
    pub id: Option<SessionId>,
    pub room: Option<RoomId>,
    pub addr: Addr<ChatServer>,
}

impl WsChatSession {
    pub fn new(addr: Addr<ChatServer>) -> Self {
        WsChatSession {
            id: None,
            room: None,
            addr: addr,
        }
    }

    pub fn execute(&self, cmd: Command, _ctx: &mut WebsocketContext<Self>) {
        match cmd {
            Command::Msg(msg) => {
                self.addr.do_send(ClientMessage {
                    session: self.id.clone().unwrap(),
                    room: self.room.clone().unwrap(),
                    msg: msg,
                });
            }
        }
    }
}

impl Actor for WsChatSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = Some(res),
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        if let Some(id) = self.id {
            self.addr.do_send(Disconnect { session: id });
        }
        Running::Stop
    }
}

impl Handler<Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            _ => {
                ctx.stop();
                return;
            }
        };

        match msg {
            // TODO: Deserialize string to json first, then check action type
            ws::Message::Text(msg) => match Command::from_str(&msg) {
                Ok(cmd) => self.execute(cmd, ctx),
                Err(err) => ctx.text(err.to_string()),
            },
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
