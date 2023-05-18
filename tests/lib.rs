use futures::future::join_all;
use liserk_encrypt::message::MessageType;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

/// Those test need tcp connection with the application I will setup test container later
fn setup() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::test]
async fn single_client_say_hello() {
    setup();
    let connection = connect_client().await;
    assert!(connection.is_ok());
}

async fn connect_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5545").await?;

    let message_type = MessageType::Authentification;
    let message = "Hello";
    let message_type: u8 = message_type.try_into().expect("uknown message type");
    let request = message.as_bytes();
    let request_length = request.len() as u32;
    let request_length = request_length.to_be_bytes();
    info!("request: {:?}", request);
    println!("request_size: {:?}", request_length);
    let full_request = &[&request_length, request].concat();
    println!("request: {:?}", full_request);
    let result = stream.write(full_request);
    println!("request: {:?}", result);
    Ok(())
}

#[tokio::test]
#[ignore = "long and log a lot"]
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
