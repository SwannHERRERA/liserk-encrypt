use config::ConfigError;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use pqc_kyber::keypair;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let alice_keys = keypair(&mut rng);
    let first_message = ClientAuthentification::new(alice_keys.public.to_vec());
    let mut stream = TcpStream::connect("127.0.0.1:5545").await?;
    let message = Message::Authentification(first_message);
    let request = message.setup_for_network()?;
    stream.write(&request).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Message {
    Authentification(ClientAuthentification),
}

impl Message {
    fn message_type(&self) -> MessageType {
        MessageType::Authentification
    }

    fn setup_for_network(&self) -> Result<Vec<u8>, Error> {
        let message_type: MessageType = self.message_type();
        let message_type: u8 = message_type as u8;
        let message = serde_cbor::to_vec(&self)?;
        let message_length = message.len() as u32;
        let message_length = message_length.to_be_bytes();

        let message_type_as_bytes = [message_type];
        Ok([&message_type_as_bytes[..], &message_length, &message].concat())
    }
}

#[repr(u8)]
pub enum MessageType {
    Authentification,
}

#[derive(Debug, Default, thiserror::Error)]
#[error("fail to parse MessageType")]
pub struct MessageTypeError {}

impl TryFrom<u8> for MessageType {
    type Error = MessageTypeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(MessageType::Authentification),
            _ => Err(MessageTypeError::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ClientAuthentification {
    protocol_version: String,
    client_public_key: Vec<u8>,
    cipher_suits: Vec<String>,
    compression: String,
}

impl ClientAuthentification {
    fn new(public_key: Vec<u8>) -> Self {
        Self {
            protocol_version: String::from("0.1.0"),
            client_public_key: public_key,
            cipher_suits: vec![String::from("kyber768"), String::from("falcon")],
            compression: String::from("0"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    TokioIoError(#[from] tokio::io::Error),
    ConfigError(#[from] ConfigError),
    SerializationError(#[from] serde_cbor::Error),
    MessageTypeError(#[from] MessageTypeError),
}
