use derive_more::{Display, Error};
use std::str::FromStr;
use uuid::Uuid;

use super::RoomId;

#[derive(Debug, Display, Error)]
#[display(fmt = "Invalid command")]
pub struct CommandError;

pub enum Command {
    Create,
    Join(RoomId),
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("/create") {
            Ok(Command::Create)
        } else if s.starts_with("/join") {
            let words: Vec<&str> = s.split_whitespace().into_iter().collect();
            let uuid = words.last().map(|&v| v).unwrap_or("");
            let uuid = Uuid::from_str(uuid).map_err(|_| CommandError)?;
            Ok(Command::Join(uuid))
        } else {
            Err(CommandError)
        }
    }
}
