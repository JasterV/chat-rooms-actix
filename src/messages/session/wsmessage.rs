use actix::Message as ActixMessage;
use serde::{Deserialize, Serialize};
use std::convert::Into;

#[derive(Serialize, Deserialize, ActixMessage)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub ty: MessageType,
    pub data: Option<String>,
}

impl WsMessage {
    pub fn err(msg: String) -> Self {
        WsMessage {
            ty: MessageType::Err,
            data: Some(msg),
        }
    }

    pub fn info(msg: String) -> Self {
        WsMessage {
            ty: MessageType::Info,
            data: Some(msg),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Join,
    Create,
    Leave,
    Msg,
    Err,
    Info,
}

impl Into<String> for WsMessage {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}