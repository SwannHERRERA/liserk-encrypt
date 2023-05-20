use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    CommunicationVersion,
    AuthetificationVersion,
    Authentification,
    ErrorResponse,
    ErrorCommunication,
    CommandComplet,
    ReadyForQuery,
    EmptyQueryResponse,
    QueryResponse,
    EndOfCommunication,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::CommunicationVersion => write!(f, "CommunicationVersion"),
            MessageType::AuthetificationVersion => write!(f, "AuthetificationVersion"),
            MessageType::Authentification => write!(f, "Authentification"),
            MessageType::ErrorResponse => write!(f, "ErrorResponse"),
            MessageType::ErrorCommunication => write!(f, "ErrorCommunication"),
            MessageType::CommandComplet => write!(f, "CommandComplet"),
            MessageType::ReadyForQuery => write!(f, "ReadyForQuery"),
            MessageType::EmptyQueryResponse => write!(f, "EmptyQueryResponse"),
            MessageType::QueryResponse => write!(f, "QueryResponse"),
            MessageType::EndOfCommunication => write!(f, "EndOfCommunication"),
        }
    }
}

impl TryFrom<u8> for MessageType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(MessageType::Authentification),
            _ => Err(()),
        }
    }
}

impl TryInto<u8> for MessageType {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            MessageType::Authentification => Ok(1),
            _ => Err(()),
        }
    }
}
