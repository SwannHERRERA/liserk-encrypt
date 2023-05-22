use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::message_type::MessageType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Message {
    ClientSetup(ClientSetupSecureConnection),
    ClientAuthentification(ClientAuthentication),
    EndOfCommunication,
}

impl Message {
    pub fn message_type(&self) -> MessageType {
        MessageType::Authentification
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

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Setup => write!(f, "Setup communcication"),
            MessageType::Authentification => write!(f, "Authentification"),
        }
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
