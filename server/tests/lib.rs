use futures::future::join_all;
use liserk_shared::message_type::MessageType;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

/// Those test need tcp connection with the application I will setup test container later
fn setup() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

#[tokio::test]
#[ignore = "need the server to run"]
async fn single_client_say_hello() {
    setup();
    let connection = connect_client().await;
    assert!(connection.is_ok());
}

async fn connect_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5545").await?;

    let message_type = MessageType::Authentification;
    let message = "Hello";
    let message_type: u8 = message_type as u8;
    let request = message.as_bytes();
    let request_length = request.len() as u32;
    let request_length = request_length.to_be_bytes();
    let message_type_as_bytes = [message_type];
    let full_request = &[&message_type_as_bytes[..], &request_length, request].concat();
    info!("request: {:?}", full_request);
    let x = stream.write(full_request).await?;
    debug!("trace: Result {}", x);
    Ok(())
}

#[tokio::test]
#[ignore = "need the server to run"]
async fn multiple_client_say_hello() {
    let mut handles = vec![];
    for _ in 0..100 {
        handles.push(tokio::spawn(async {
            let connection = connect_client().await;
            assert!(connection.is_ok());
        }));
    }
    join_all(handles).await;
}
