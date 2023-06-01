use crate::{message_type::MessageType, query::Query};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Message {
    ClientSetup(ClientSetupSecureConnection),
    ClientAuthentification(ClientAuthentication),
    Insert(Insertion),
    InsertResponse { inserted_id: String },
    Query(Query),
    QueryResponse { data: Vec<Vec<u8>> },
    SingleValueResponse { data: Option<Vec<u8>> },
    Update(Update),
    UpdateResponse { status: UpdateStatus },
    Delete(Delete),
    DeleteResult(bool),
    EndOfCommunication,
    CloseCommunication, // This is probably a bad idea
}

impl Message {
    pub fn message_type(&self) -> MessageType {
        match self {
            Message::ClientSetup(_) => MessageType::Setup,
            Message::ClientAuthentification(_) => MessageType::Authentification,
            Message::Insert(_) => MessageType::Insert,
            Message::InsertResponse { .. } => MessageType::InsertResponse,
            Message::Query(_) => MessageType::Query,
            Message::QueryResponse { .. } => MessageType::QueryResponse,
            Message::SingleValueResponse { .. } => MessageType::SingleValueResponse,
            Message::Update { .. } => MessageType::Update,
            Message::UpdateResponse { .. } => MessageType::UpdateResponse,
            Message::Delete(_) => MessageType::Delete,
            Message::DeleteResult(_) => MessageType::DeleteResult,
            Message::EndOfCommunication => MessageType::EndOfCommunication,
            Message::CloseCommunication => MessageType::CloseCommunication,
        }
    }

    pub fn setup_for_network(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        let message_type: MessageType = self.message_type();
        let message_type: u8 = message_type as u8;
        let message = serde_cbor::to_vec(&self)?;
        let message_length = message.len() as u32;
        let message_length = message_length.to_be_bytes();

        let message_type_as_bytes = [message_type];
        Ok([&message_type_as_bytes[..], &message_length, &message].concat())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ClientSetupSecureConnection {
    protocol_version: String,
    client_public_key: Vec<u8>,
    cipher_suits: Vec<String>,
    compression: String,
}

impl ClientSetupSecureConnection {
    pub fn new(public_key: Vec<u8>) -> Self {
        Self {
            protocol_version: String::from("0.1.0"),
            client_public_key: public_key,
            cipher_suits: vec![String::from("kyber768"), String::from("falcon")],
            compression: String::from("0"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ClientAuthentication {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Update {
    pub collection: String,
    pub id: String,
    pub new_value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum UpdateStatus {
    Success,
    Failure,
    KeyNotFound,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Insertion {
    pub collection: String,
    pub acl: Vec<String>,
    pub data: Vec<u8>,
    pub usecases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Delete {
    pub collection: String,
    pub id: String,
}
