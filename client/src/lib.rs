use config::ConfigError;
use shared::{
    message::{ClientAuthentication, ClientSetupSecureConnection, Insertion, Message},
    message_type::{MessageType, MessageTypeError},
    query::Query,
};
use tokio::{
    io::AsyncReadExt,
    net::{tcp::OwnedReadHalf, TcpStream},
};
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf};
use tracing::{debug, info, trace};

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    TokioIoError(#[from] tokio::io::Error),
    ConfigError(#[from] ConfigError),
    SerializationError(#[from] serde_cbor::Error),
    MessageTypeError(#[from] MessageTypeError),
}

// TODO Build with that
#[derive(Debug, Default)]
pub struct UnconnectedClient;

#[derive(Debug)]
pub struct ConnectedClient {
    pub stream: TcpStream,
}

#[derive(Debug)]
pub struct AuthenticatedClient {
    pub read: OwnedReadHalf,
    pub write: OwnedWriteHalf,
}

impl UnconnectedClient {
    pub async fn connect(self, url: &str) -> Result<ConnectedClient, Error> {
        let mut rng = rand::thread_rng();
        let kyber_key = pqc_kyber::keypair(&mut rng);
        let mut stream = TcpStream::connect(url).await?;
        let setup_security = Message::ClientSetup(ClientSetupSecureConnection::new(
            kyber_key.public.to_vec(),
        ));
        let message = setup_security.setup_for_network()?;
        // debug!("message {:?}", message);
        stream.write_all(&message).await?;
        Ok(ConnectedClient { stream })
    }
}

impl ConnectedClient {
    pub async fn authenticate(
        mut self,
        username: String,
        password: String,
    ) -> Result<AuthenticatedClient, Error> {
        let client_authentication = ClientAuthentication { username, password };
        let message = Message::ClientAuthentification(client_authentication);
        let message = message.setup_for_network()?;
        // debug!("message {:?}", message);
        self.stream.write_all(&message).await?;

        let (read, write) = self.stream.into_split();
        let auth_client = AuthenticatedClient { read, write };
        Ok(auth_client)
    }
}

impl AuthenticatedClient {
    pub fn is_alive(&self) -> bool {
        true
    }
    pub async fn terminate_connection(&mut self) -> Result<(), Error> {
        let message = Message::EndOfCommunication;
        let message = message.setup_for_network()?;
        // debug!("terminate Connection {:?}", message);
        self.write.write_all(&message).await?;
        Ok(())
    }

    pub async fn insert(
        &mut self,
        collection: String,
        data: Vec<u8>,
        acl: Vec<String>,
        usecases: Vec<String>,
    ) -> Result<(), Error> {
        let message = Message::Insert(Insertion { acl, collection, data, usecases });
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;
        info!("message: {:?}", message);
        Ok(())
    }

    pub async fn query(&mut self, query: Query) -> Result<(), Error> {
        let message = Message::Query(query);
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;
        info!("message: {:?}", message);
        Ok(())
    }
}

async fn parse_message_from_tcp_stream(
    stream: &mut OwnedReadHalf,
) -> Result<Message, Error> {
    let mut buffer = [0; 1];
    let _ = stream.read(&mut buffer).await;
    let message_type = MessageType::try_from(buffer[0]);
    info!("messageType: {:?}", message_type);

    let mut message_size = [0; 4];
    let _size_error = stream.read(&mut message_size).await;
    let decimal_size = u32::from_be_bytes(message_size);
    trace!("message size: {}", decimal_size);

    let mut slice = vec![0; decimal_size as usize];
    let _size_read = stream.read_exact(&mut slice).await;
    trace!("slice: {:?}", slice);
    let message: Message = serde_cbor::from_slice(&slice)?;
    debug!("parsed message: {:#?}", message);
    Ok(message)
}
