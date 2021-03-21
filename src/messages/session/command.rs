use actix::Message as ActixMessage;
use derive_more::{Display, Error};
use std::convert::Into;
use std::str::FromStr;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub enum Command {
    Msg(String),
    GetRoomId,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Invalid command: {}", msg)]
pub struct CommandError {
    msg: &'static str,
}

// TODO: IMPLEMENT MORE COMMANDS
impl FromStr for Command {
    type Err = CommandError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = data.trim().split_whitespace().collect();
        let first_word = words.first();

        if let Some(&word) = first_word {
            return match word {
                "/roomId" => Ok(Command::GetRoomId),
                _ => Ok(Command::Msg(data.into())),
            };
        }
        Ok(Command::Msg(data.into()))
    }
}
