use config::ConfigError;
use liserk_shared::message_type::MessageTypeError;

/// Enum representing the possible errors that can be encountered by the client.
#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    /// Represents an I/O error from the Tokio runtime.
    TokioIoError(#[from] tokio::io::Error),

    /// Represents a configuration error.
    ConfigError(#[from] ConfigError),

    /// Represents an error encountered during serialization using CBOR format.
    SerializationError(#[from] serde_cbor::Error),

    /// Represents an error regarding the type of message.
    MessageTypeError(#[from] MessageTypeError),

    /// Represents an encryption error when using AES-GCM-SIV.
    EcryptionError(AesError),
}

#[derive(Debug)]
pub enum AesError {
    Encrypt,
    Decrypt,
}
