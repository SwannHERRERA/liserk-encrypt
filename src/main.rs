use std::{io, net::SocketAddr};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, thiserror::Error)]
enum Error {}

async fn on_new_client(socket: &mut TcpStream, _addr: &SocketAddr) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut message = Vec::new();

    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        };

        message.extend_from_slice(&buffer[..n]);

        if message.ends_with(&[b'\n']) {
            on_end_message(&message, socket).await?;
            break;
        }
    }
    Ok(())
}

async fn on_end_message(message: &[u8], socket: &mut TcpStream) -> io::Result<()> {
    let message_str = String::from_utf8_lossy(message);
    process_message(&message_str);

    let response = "Server received your message\n";
    if let Err(e) = socket.write_all(response.as_bytes()).await {
        eprintln!("Error writing to socket: {}", e);
    }
    Ok(())
}

fn process_message(message: &str) {
    println!("Received message: {}", message);
}

#[tokio::main]
async fn main() -> io::Result<()> {
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
