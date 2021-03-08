use crate::{
    actors::chat_server::ChatServer,
    constants::CLIENT_TIMEOUT,
    models::{
        commands::Command,
        ws::{MessageType, WsMessage},
        RoomId, SessionId,
    },
};
use crate::{
    constants::HEARTBEAT_INTERVAL,
    messages::{
        chat_server::{ClientMessage, Connect, CreateRoom, Disconnect, JoinRoom, Leave},
        chat_session::Message,
    },
};
use actix::{
    clock::Instant, fut, ActorContext, ActorFuture, ContextFutureSpawner, Handler, Running,
    StreamHandler, WrapFuture,
};
use actix::{Actor, Addr, AsyncContext};
use actix_web_actors::ws::{self, WebsocketContext};
use std::str::FromStr;
use uuid::Uuid;

pub struct WsChatSession {
    pub id: SessionId,
    pub room: Option<RoomId>,
    pub addr: Addr<ChatServer>,
    pub hb: Instant,
}

impl WsChatSession {
    pub fn new(addr: Addr<ChatServer>) -> Self {
        WsChatSession {
            id: Uuid::new_v4(),
            room: None,
            hb: Instant::now(),
            addr,
        }
    }

    fn hb(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // notify chat server
                act.addr.do_send(Disconnect { session: act.id });
                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
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
                    session: self.id.clone(),
                    room: self.room.clone().unwrap(),
                    msg,
                });
            }
        }
    }

    fn create(&self, ctx: &mut WebsocketContext<Self>) {
        self.addr
            .send(CreateRoom {
                session: self.id.clone(),
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
                        session: self.id.clone(),
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
                session: self.id.clone(),
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
        self.hb(ctx);
        let addr = ctx.address();
        self.addr
            .send(Connect {
                id: self.id.clone(),
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                if let Err(err) = res {
                    ctx.text(WsMessage::err(err.to_string()));
                    ctx.stop();
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(Disconnect {
            session: self.id.clone(),
        });
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
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
