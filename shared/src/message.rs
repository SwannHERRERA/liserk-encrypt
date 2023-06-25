use crate::{message_type::MessageType, query::Query};
use serde::{Deserialize, Serialize};

/// Enum representing different types of messages exchanged between the client and server.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Message {
    /// Message sent by the client when setting up a secure connection.
    /// The associated `ClientSetupSecureConnection` contains the necessary information for establishing the secure connection.
    ClientSetup(ClientSetupSecureConnection),

    /// Message used for client authentication.
    /// The associated `ClientAuthentication` typically contains the credentials needed for authentication.
    ClientAuthentification(ClientAuthentication),

    /// Used by the client to insert data into the database.
    /// The `Insertion` structure typically contains the data to be inserted along with metadata such as the collection in which the data should be stored.
    Insert(Insertion),

    /// Similar to `Insert`, but used specifically for inserting data that is encrypted using Order-Preserving Encryption (OPE).
    InsertOpe(Insertion),

    /// Sent by the server in response to an `Insert` message to acknowledge that the data has been inserted.
    /// Contains the ID of the inserted data.
    InsertResponse { inserted_id: String },

    /// Used by the client to query data from the database.
    /// The `Query` structure contains the necessary information to perform the data query.
    Query(Query),

    /// Sent by the server in response to a `Query` message.
    /// Contains the data retrieved as a result of the query.
    QueryResponse { data: Vec<Vec<u8>> },

    /// Sent by the server in response to a query that requests a single value.
    /// Contains the requested data, or None if it doesn't exist.
    SingleValueResponse { data: Option<Vec<u8>> },

    /// Message sent by the client to request a count of documents that meet certain criteria.
    /// The `CountSubject` structure defines the criteria for counting.
    Count(CountSubject),

    /// Sent by the server in response to a `Count` message.
    /// Contains the number of documents that meet the specified criteria.
    CountResponse(u32),

    /// Message sent by the client to request an update to existing data.
    /// The `Update` structure contains the details of what data should be updated and how.
    Update(Update),

    /// Sent by the server in response to an `Update` message to indicate the status of the update operation.
    UpdateResponse { status: UpdateStatus },

    /// Message sent by the client to request the deletion of data.
    /// The `Delete` structure contains the details of what data should be deleted.
    Delete(Delete),

    /// Sent by the server to indicate the result of a deletion request.
    DeleteResult(bool),

    /// Message sent by the client to delete data for a specific use case.
    /// Contains the collection name and the ID of the document to be deleted.
    DeleteForUsecase { collection: String, id: String },

    /// Message sent by the client to request the deletion of an entire collection or use case.
    /// The `DropSubject` structure defines what should be dropped.
    Drop(DropSubject),

    /// Sent by the server to indicate the result of a drop request.
    DropResult(bool),

    /// Message indicating the end of a communication sequence.
    EndOfCommunication,

    /// Message requesting the termination of the communication channel.
    CloseCommunication,
}

impl Message {
    pub fn message_type(&self) -> MessageType {
        match self {
            Message::ClientSetup(_) => MessageType::Setup,
            Message::ClientAuthentification(_) => MessageType::Authentification,
            Message::Insert(_) => MessageType::Insert,
            Message::InsertOpe(_) => MessageType::InsertOpe,
            Message::InsertResponse { .. } => MessageType::InsertResponse,
            Message::Query(_) => MessageType::Query,
            Message::QueryResponse { .. } => MessageType::QueryResponse,
            Message::SingleValueResponse { .. } => MessageType::SingleValueResponse,
            Message::Count(_) => MessageType::Count,
            Message::CountResponse(_) => todo!(),
            Message::Update { .. } => MessageType::Update,
            Message::UpdateResponse { .. } => MessageType::UpdateResponse,
            Message::Delete(_) => MessageType::Delete,
            Message::DeleteResult(_) => MessageType::DeleteResult,
            Message::DeleteForUsecase { .. } => MessageType::DeleteForUsecase,
            Message::Drop(_) => MessageType::Drop,
            Message::DropResult(_) => MessageType::DropResult,
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
pub enum CountSubject {
    Collection(String),
    Usecase { collection: String, usecase: String },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum DropSubject {
    Collection(String),
    Usecase { collection: String, usecase: String },
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
