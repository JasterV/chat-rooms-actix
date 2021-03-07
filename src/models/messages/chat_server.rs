use actix::{Message as ActixMessage, Recipient};
use uuid::Uuid;
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub &'static str);

#[derive(ActixMessage)]
#[rtype(result = "Uuid")]
pub struct CreateRoom {
    pub session: Uuid,
}
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub session: Uuid,
    pub room: Uuid,
}

#[derive(ActixMessage)]
#[rtype(result = "Uuid")]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session: Uuid,
}
