extern crate pqcrypto_falcon;

use std::net::SocketAddr;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};
use certificate::create_certificate;
use settings::SETTINGS;

mod certificate;
mod error;
mod settings;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let app = Router::new().nest(
        "/certificate",
        Router::new()
            .route("/create_certificate", post(create_certificate_handler))
            .route("/verify_certificate", post(verify_certificate_handler)),
    );

    println!("Listening on {}", addr);

    // Lance le serveur
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_certificate_handler() -> impl IntoResponse {
    match create_certificate() {
        Ok(_) => {
            let certificates_path = &SETTINGS.cipher.certificates_path;
            let message = format!("Certificate created successfully at {}/certificate.crt and {}/encrypted.kyber", certificates_path, certificates_path);
            (StatusCode::OK, message)
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create certificate: {:?}", e),
        ),
    }
}

async fn verify_certificate_handler() -> impl IntoResponse {
    // TODO verify cert
    (StatusCode::OK, "Verification endpoint")
}
