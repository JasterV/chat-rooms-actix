use actix::Message as ActixMessage;
use derive_more::{Display, Error};
use std::convert::Into;
use std::str::FromStr;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub enum Command {
    Msg(String),
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Invalid command: {}", msg)]
pub struct CommandError {
    msg: &'static str,
}

// TODO: IMPLEMENT MORE COMMANDS
impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Command::Msg(s.into()))
    }
}
