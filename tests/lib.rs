use std::time::Duration;

use liserk_encrypt::message::MessageType;
use liserk_encrypt::run_app;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::sleep;

#[tokio::test]
async fn single_client_say_hello_world() -> io::Result<()> {
    let _handle = tokio::spawn(async {
        let _ = run_app().await;
    });
    sleep(Duration::new(0, 100)).await;
    let mut stream = TcpStream::connect("127.0.0.1:5545").await?;

    let message_type = MessageType::Authentification;
    let message = "Hello";
    let message_type: u8 = message_type.try_into().expect("uknown message type");
    let request = message.as_bytes();
    let request_size = request.len() as u32;
    let mut full_request = Vec::with_capacity(100);
    full_request.push(message_type);
    for octet in request_size.to_be_bytes() {
        full_request.push(octet);
    }
    let full_request = [full_request.as_slice(), request].concat();
    stream.write_all(&full_request).await?;
    Ok(())
}
