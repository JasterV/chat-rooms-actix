use crate::{
    actors::chat_server::ChatServer,
    models::messages::chat_server::{Connect, Disconnect, Message},
};
use actix::*;
use actix::{Actor, Addr, AsyncContext};
use actix_web_actors::ws::{self, WebsocketContext};
use uuid::Uuid;

pub struct WsChatSession {
    pub id: Option<Uuid>,
    pub room: Option<Uuid>,
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
        //TODO: Implement receiving message
    }
}
