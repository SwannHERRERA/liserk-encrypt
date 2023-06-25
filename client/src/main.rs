use liserk_shared::message::{ClientSetupSecureConnection, Message};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use liserk_client::Error;
use pqc_kyber::keypair;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut rng = rand::thread_rng();
    let alice_keys = keypair(&mut rng);
    let first_message = ClientSetupSecureConnection::new(alice_keys.public.to_vec());
    let mut stream = TcpStream::connect("127.0.0.1:5545").await?;
    let message = Message::ClientSetup(first_message);
    let request = message.setup_for_network()?;
    stream.write_all(&request).await?;
    Ok(())
}
