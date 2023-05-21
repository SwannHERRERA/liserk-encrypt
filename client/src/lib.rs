use config::ConfigError;
use shared::message_type::MessageTypeError;

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    TokioIoError(#[from] tokio::io::Error),
    ConfigError(#[from] ConfigError),
    SerializationError(#[from] serde_cbor::Error),
    MessageTypeError(#[from] MessageTypeError),
}

// TODO Build with that
pub struct Client {}
