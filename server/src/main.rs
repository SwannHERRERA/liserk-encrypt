use liserk_encrypt::run_app;
use pqc_kyber::{keypair, Ake, KyberError};
use std::io;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> io::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    print_keys();
    match run_app().await {
        Ok(_) => {} // Do nothing
        Err(err) => error!("{:?}", err),
    }
    Ok(())
}

fn print_keys() {
    let mut rng = rand::thread_rng();
    let alice_keys = keypair(&mut rng);
    println!("{:?}", alice_keys.secret);
}

fn generate_secret_key() -> Result<(), KyberError> {
    let mut alice = Ake::new();
    let mut bob = Ake::new();
    let mut rng = rand::thread_rng();
    let alice_keys = keypair(&mut rng);
    let bob_keys = keypair(&mut rng);

    let client_init = alice.client_init(&bob_keys.public, &mut rng);

    let server_response =
        bob.server_receive(client_init, &alice_keys.public, &bob_keys.secret, &mut rng)?;

    alice.client_confirm(server_response, &alice_keys.secret)?;

    Ok(())
}
