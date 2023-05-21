use liserk_encrypt::run_app;
use std::io;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> io::Result<()> {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    match run_app().await {
        Ok(_) => {} // Do nothing
        Err(err) => error!("{:?}", err),
    }
    Ok(())
}
