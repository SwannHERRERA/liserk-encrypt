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

impl TryFrom<u8> for MessageType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            _ => Err(()),
        }
    }
}
