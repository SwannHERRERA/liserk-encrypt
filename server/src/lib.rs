use serde::{Deserialize, Serialize};
use shared::message::Message;
use shared::message_type::MessageType;
use std::fmt::Display;
use std::{io, net::SocketAddr};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};
use uuid::Uuid;

use crate::command::Command;
use crate::message_parsing::parse_message;

pub const BINDED_URL_PORT: &str = "127.0.0.1:5545";

mod command;
mod config;
mod insert;
mod message_parsing;
mod query_engine;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    TokioIo(#[from] tokio::io::Error),
    Parsing(#[from] serde_cbor::Error),
    Storage(#[from] tikv_client::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TokioIo(_) => write!(f, "Tokio IO Error"),
            Error::Parsing(_) => write!(f, "Parsing Error serde"),
            Error::Storage(err) => write!(f, "Error with storage layer {}", err),
        }
    }
}

async fn on_new_client(socket: &mut TcpStream, _addr: &SocketAddr) -> Result<(), Error> {
    loop {
        let message = parse_message_from_tcp_stream(socket).await?;
        let command = parse_message(message, socket).await;
        info!("end with this message");
        if command == Command::Exit {
            break;
        }
    }
    Ok(())
}

async fn parse_message_from_tcp_stream(stream: &mut TcpStream) -> Result<Message, Error> {
    let mut buffer = [0; 1];
    let _ = stream.read(&mut buffer).await;
    let message_type = MessageType::try_from(buffer[0]);
    info!("messageType: {:?}", message_type);

    let mut message_size = [0; 4];
    let _size_error = stream.read(&mut message_size).await;
    let decimal_size = u32::from_be_bytes(message_size);
    debug!("{}", decimal_size);

    let mut slice = vec![0; decimal_size as usize];
    let _size_read = stream.read_exact(&mut slice).await;
    info!("slice: {:?}", slice);
    let message: Message = serde_cbor::from_slice(&slice)?;
    debug!("parsed message: {:#?}", message);
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
    let listener = TcpListener::bind(BINDED_URL_PORT).await?;
    info!("Server started, listening on {}", BINDED_URL_PORT);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            match on_new_client(&mut socket, &addr).await {
                Ok(_) => println!("c'est ok"),
                Err(err) => eprintln!("err: {}", err),
            };
        });
    }
}
