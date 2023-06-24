use config::ConfigError;
use liserk_shared::{
    message::{
        ClientAuthentication, ClientSetupSecureConnection, Delete, Insertion, Message,
        Update,
    },
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
    ) -> Result<String, Error> {
        let message = Message::Insert(Insertion { acl, collection, data, usecases });
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;
        info!("message: {:?}", message);
        match message {
            Message::InsertResponse { inserted_id } => Ok(inserted_id),
            _ => Err(Error::MessageTypeError(MessageTypeError::default())),
        }
    }

    pub async fn insert_ope() -> Result<Message, Error> {
        todo!()
    }

    pub async fn query(&mut self, query: Query) -> Result<Message, Error> {
        let message = Message::Query(query);
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;
        info!("message: {:?}", message);
        match message {
            Message::QueryResponse { .. } => Ok(message),
            Message::SingleValueResponse { .. } => Ok(message),
            _ => Err(Error::MessageTypeError(MessageTypeError::default())),
        }
    }

    pub async fn modify(
        &mut self,
        id: String,
        collection: String,
        new_value: Vec<u8>,
    ) -> Result<Message, Error> {
        let update = Update { collection, id, new_value };
        let message = Message::Update(update);
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;

        info!("message: {:?}", message);
        match message {
            Message::UpdateResponse { .. } => Ok(message),
            _ => Err(Error::MessageTypeError(MessageTypeError::default())),
        }
    }
    pub async fn delete(
        &mut self,
        id: String,
        collection: String,
    ) -> Result<Message, Error> {
        let delete = Delete { collection, id };
        let message = Message::Delete(delete);
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;

        info!("message: {:?}", message);
        match message {
            Message::DeleteResult(_) => Ok(message),
            _ => Err(Error::MessageTypeError(MessageTypeError::default())),
        }
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
