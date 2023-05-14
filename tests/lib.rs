use std::time::Duration;

use liserk_encrypt::run_app;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::sleep;

#[tokio::test]
async fn single_client_say_hello_world() -> io::Result<()> {
    let _handle = tokio::spawn(async {
        let _ = run_app().await;
    });
    sleep(Duration::new(0, 100)).await;
    let mut stream = TcpStream::connect("127.0.0.1:5545").await?;

    let request = "user@password\n";
    stream.write_all(request.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Server response: {}", response);

    Ok(())
}
