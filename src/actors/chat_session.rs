use crate::messages::{
    chat_server::{ClientMessage, Connect, CreateRoom, Disconnect, JoinRoom, Leave},
    chat_session::Message,
};
use crate::{
    actors::chat_server::ChatServer,
    models::{
        commands::Command,
        ws::{MessageType, WsMessage},
        RoomId, SessionId,
    },
};
use actix::{
    fut, ActorContext, ActorFuture, ContextFutureSpawner, Handler, Running, StreamHandler,
    WrapFuture,
};
use actix::{Actor, Addr, AsyncContext};
use actix_web_actors::ws::{self, WebsocketContext};
use std::str::FromStr;
use uuid::Uuid;

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
            addr,
        }
    }

    pub fn handle_msg(&self, msg: WsMessage, ctx: &mut WebsocketContext<Self>) {
        let data = msg.data.unwrap_or("".into());
        match msg.ty {
            MessageType::Create => self.create(ctx),
            MessageType::Join => self.join(data, ctx),
            MessageType::Msg => self.msg(data, ctx),
            MessageType::Leave => self.leave(ctx),
            MessageType::Err => (),
        }
    }

    pub fn execute(&self, cmd: Command, _ctx: &mut WebsocketContext<Self>) {
        match cmd {
            Command::Msg(msg) => {
                self.addr.do_send(ClientMessage {
                    session: self.id.clone().unwrap(),
                    room: self.room.clone().unwrap(),
                    msg,
                });
            }
        }
    }

    fn create(&self, ctx: &mut WebsocketContext<Self>) {
        self.addr
            .send(CreateRoom {
                session: self.id.clone().unwrap(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {
                        act.room = Some(res.clone());
                        ctx.text(WsMessage {
                            ty: MessageType::Create,
                            data: Some(res.to_string()),
                        });
                    }
                    // something is wrong with chat server
                    Err(err) => {
                        ctx.text(WsMessage::err(err.to_string()));
                        ctx.stop();
                    }
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn join(&self, room_id: String, ctx: &mut WebsocketContext<Self>) {
        match Uuid::from_str(&room_id) {
            Ok(uuid) => {
                self.addr
                    .send(JoinRoom {
                        room: uuid,
                        session: self.id.clone().unwrap(),
                    })
                    .into_actor(self)
                    .then(move |res, act, ctx| {
                        match res {
                            Ok(res) => match res {
                                Ok(_) => {
                                    act.room = Some(uuid.clone());
                                    ctx.text(WsMessage {
                                        ty: MessageType::Msg,
                                        data: Some("Joined!".into()),
                                    })
                                }
                                Err(err) => ctx.text(WsMessage::err(err.to_string())),
                            },
                            // something is wrong with chat server
                            Err(err) => {
                                ctx.text(WsMessage::err(err.to_string()));
                                ctx.stop();
                            }
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
            Err(err) => ctx.text(WsMessage::err(err.to_string())),
        }
    }

    fn msg(&self, msg: String, ctx: &mut WebsocketContext<Self>) {
        match Command::from_str(&msg) {
            Ok(cmd) if self.room.is_some() => self.execute(cmd, ctx),
            Ok(_) => ctx.text(WsMessage::err("You are not in a room yet".into())),
            Err(err) => ctx.text(WsMessage::err(err.to_string())),
        }
    }

    fn leave(&self, ctx: &mut WebsocketContext<Self>) {
        self.addr
            .send(Leave {
                session: self.id.clone().unwrap(),
            })
            .into_actor(self)
            .then(move |res, act, ctx| {
                match res {
                    Ok(_) => {
                        act.room = None;
                        ctx.text(WsMessage {
                            ty: MessageType::Leave,
                            data: Some("Room leaved".into()),
                        })
                    }
                    // something is wrong with chat server
                    Err(err) => {
                        ctx.text(WsMessage::err(err.to_string()));
                        ctx.stop();
                    }
                }
                fut::ready(())
            })
            .wait(ctx);
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
                    Err(err) => {
                        ctx.text(WsMessage::err(err.to_string()));
                        ctx.stop();
                    }
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
            Err(err) => {
                ctx.text(WsMessage::err(err.to_string()));
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Text(msg) => match serde_json::from_str::<WsMessage>(&msg) {
                Ok(content) => self.handle_msg(content, ctx),
                Err(err) => ctx.text(WsMessage::err(err.to_string())),
            },
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
