use liserk_encrypt::run_app;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let _ = run_app().await;
    Ok(())
}
