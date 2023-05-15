use futures::future::join_all;
use liserk_encrypt::message::MessageType;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::info;

/// Those test need tcp connection with the application I will setup test container later

#[tokio::test]
async fn single_client_say_hello() {
    let connection = connect_client().await;
    assert!(connection.is_ok());
}

async fn connect_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5545").await?;

    let message_type = MessageType::Authentification;
    let message = "Hello";
    let message_type: u8 = message_type.try_into().expect("uknown message type");
    let request = message.as_bytes();
    let request_size = request.len() as u32;
    info!("request_size: {}", request_size);
    let mut full_request = Vec::with_capacity(100);
    full_request.push(message_type);
    for octet in request_size.to_be_bytes() {
        full_request.push(octet);
    }
    let full_request = [full_request.as_slice(), request].concat();
    stream.write_all(&full_request).await?;
    Ok(())
}

#[tokio::test]
async fn multiple_client_say_hello() {
    let mut handles = vec![];
    for _ in 0..1000 {
        handles.push(tokio::spawn(async {
            let connection = connect_client().await;
            assert!(connection.is_ok());
        }));
    }
    join_all(handles).await;
}
