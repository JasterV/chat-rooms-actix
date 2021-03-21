pub mod command;
pub mod wsmessage;
use actix::Message as ActixMessage;
use serde::Serialize;

#[derive(Serialize, ActixMessage)]
#[rtype(result = "()")]
pub struct Message {
    pub nickname: Option<String>,
    pub msg: String,
}
