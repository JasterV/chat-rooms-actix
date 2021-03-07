use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Join,
    Create,
    Leave,
    Msg,
    Err,
}

impl Into<String> for WsMessage {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
