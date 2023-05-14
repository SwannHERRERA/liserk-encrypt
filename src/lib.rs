use std::{io, net::SocketAddr};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

mod message;
use message::{Message, MessageType};
use tracing::{event, Level};

#[derive(Debug, thiserror::Error)]
enum Error {}

async fn on_new_client(socket: &mut TcpStream, _addr: &SocketAddr) -> io::Result<()> {
    let _ = parse_message_from_tcp_stream(socket);
    Ok(())
}

fn parse_message_from_tcp_stream(stream: &mut TcpStream) -> Message {
    let mut message_size = [0; 1];
    let _size_error = stream.read(&mut message_size);
    let _message_type = MessageType::try_from(message_size[0]);

    let mut message_size = [0; 4];
    let _size_error = stream.read(&mut message_size);
    let decimal_size = u32::from_be_bytes(message_size);

    let mut slice = vec![0; decimal_size as usize];
    let _size_read = stream.read_exact(&mut slice);
    let message = serde_cbor::from_slice(&slice);
    match message {
        Ok(m) => m,
        Err(err) => {
            event!(Level::WARN, "Cannot parse message : {:?}", err);
            Message::Hello // TODO fix
        }
    }
}

async fn on_message(message: &[u8], socket: &mut TcpStream) -> io::Result<()> {
    let message_str = String::from_utf8_lossy(message);
    print_message(&message_str);

    let response = "Server received your message\n";
    if let Err(e) = socket.write_all(response.as_bytes()).await {
        eprintln!("Error writing to socket: {}", e);
    }
    Ok(())
}

fn print_message(message: &str) {
    println!("Received message: {}", message);
}

pub async fn run_app() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5545").await?;
    println!("Server started, listening on port 5545");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            on_new_client(&mut socket, &addr)
                .await
                .expect("Error on message");
        });
    }
}
