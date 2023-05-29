use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize};
use tracing::debug;

#[derive(Debug, Serialize)]
#[repr(u8)]
pub enum MessageType {
    Setup,
    Authentification,
    EndOfCommunication,
    Insert,
    Query,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Setup => write!(f, "Setup communcication"),
            MessageType::Authentification => write!(f, "Authentification"),
            MessageType::EndOfCommunication => write!(f, "EndOfCommunication"),
            MessageType::Insert => write!(f, "Insert"),
            MessageType::Query => write!(f, "Query"),
        }
    }
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        debug!("parsing message type: {}", s);

        if s == "Setup" {
            return Ok(MessageType::Setup);
        }

        if s == "Authentification" {
            return Ok(MessageType::Authentification);
        }

        if s == "Insert" {
            return Ok(MessageType::Insert);
        }

        if s == "EndOfCommunication" {
            return Ok(MessageType::EndOfCommunication);
        }

        if s == "Query" {
            return Ok(MessageType::Query);
        }
        panic!("panic deserialize message type");
    }
}

#[derive(Debug, Default, thiserror::Error)]
#[error("fail to parse MessageType")]
pub struct MessageTypeError {}

impl TryFrom<u8> for MessageType {
    type Error = MessageTypeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(MessageType::Setup),
            1 => Ok(MessageType::Authentification),
            2 => Ok(MessageType::EndOfCommunication),
            3 => Ok(MessageType::Insert),
            4 => Ok(MessageType::Query),
            _ => Err(MessageTypeError::default()),
        }
    }
}
