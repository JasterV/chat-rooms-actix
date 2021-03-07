use derive_more::{Display, Error};
use std::convert::Into;
use std::str::FromStr;

#[derive(Debug, Display, Error)]
#[display(fmt = "Invalid command: {}", msg)]
pub struct CommandError {
    msg: &'static str,
}

pub enum Command {
    Join(String),
    Msg(String),
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim().split_whitespace().into_iter().collect();
        let first = words.first().map(|&v| v).unwrap_or("");
        if first == "/join" {
            let name = words.last().map(|&v| v).unwrap_or("");
            Ok(Command::Join(name.into()))
        } else {
            Ok(Command::Msg(s.into()))
        }
    }
}
