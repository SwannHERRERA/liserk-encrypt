use serde::{Deserialize, Serialize};
use shared::message::Message;
use shared::message_type::MessageType;
use std::fmt::Display;
use std::{io, net::SocketAddr};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
enum Error {
    TokioIoError(#[from] tokio::io::Error),
    ParsingError(#[from] serde_cbor::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TokioIoError(_) => write!(f, "Tokio IO Error"),
            Error::ParsingError(_) => write!(f, "Parsing Error serde"),
        }
    }
}

async fn on_new_client(socket: &mut TcpStream, _addr: &SocketAddr) -> Result<(), Error> {
    let message = parse_message_from_tcp_stream(socket).await?;
    info!("{:?}", message);
    Ok(())
}

async fn parse_message_from_tcp_stream(stream: &mut TcpStream) -> Result<Message, Error> {
    let mut buffer = [0; 1];
    let _ = stream.read(&mut buffer).await;
    let _message_type = MessageType::try_from(buffer[0]);
    debug!("{:?}", buffer);
    //println!("type: {}", message_type.unwrap());

    let mut message_size = [0; 4];
    let _size_error = stream.read(&mut message_size).await;
    let decimal_size = u32::from_be_bytes(message_size);
    info!("taille du message: {}", decimal_size);

    let mut slice = vec![0; decimal_size as usize];
    let _size_read = stream.read_exact(&mut slice);
    let message: Message = serde_cbor::from_slice(&slice)?;
    Ok(message)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct Authentification {
    protocol_version: u32,
    server_public_key: Vec<u8>,
    session_id: Uuid,
    cipher_suits: Vec<String>,
    compression: String,
}

pub async fn run_app() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5545").await?;
    info!("Server started, listening on port 5545");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            on_new_client(&mut socket, &addr).await.expect("Error on message");
        });
    }
}
