use config::ConfigError;
use shared::{
    message::{ClientAuthentication, Message},
    message_type::MessageTypeError,
};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    TokioIoError(#[from] tokio::io::Error),
    ConfigError(#[from] ConfigError),
    SerializationError(#[from] serde_cbor::Error),
    MessageTypeError(#[from] MessageTypeError),
}

// TODO Build with that
pub struct UnconnectedClient;

#[derive(Debug)]
pub struct ConnectedClient {
    pub stream: TcpStream,
}

#[derive(Debug)]
pub struct AuthenticatedClient {
    pub stream: TcpStream,
}

impl UnconnectedClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn connect(self, url: String) -> Result<ConnectedClient, Error> {
        let stream = TcpStream::connect(url).await?;
        Ok(ConnectedClient { stream })
    }
}

impl ConnectedClient {
    pub async fn authenticate(
        &mut self,
        username: String,
        password: String,
    ) -> Result<AuthenticatedClient, Error> {
        let client_authentication = ClientAuthentication { username, password };
        let message = Message::ClientAuthentification(client_authentication);
        self.stream.write(&message.setup_for_network()?).await?;

        let stream = TcpStream::connect("").await?;
        Ok(AuthenticatedClient { stream })
    }
}

impl AuthenticatedClient {}
