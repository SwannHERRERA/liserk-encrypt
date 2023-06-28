use liserk_ope::simplified_version::encrypt_ope;
use liserk_shared::{
    message::{
        ClientAuthentication, ClientSetupSecureConnection, Delete, Insertion, Message,
        Update,
    },
    message_type::{MessageType, MessageTypeError},
    query::Query,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};
use tracing::{debug, info, trace};

const KEY: [u8; 32] = [0; 32];
const NONCE: [u8; 12] = [0; 12];

use crate::{basic_decrypt, basic_encrypt, error::Error};

#[derive(Debug)]
pub enum QueryResult {
    EmptyResult,
    SingleValue(Vec<u8>),
    MultipleValues(Vec<Vec<u8>>),
}

/// Represents a client that has not yet established a connection to the server.
#[derive(Debug, Default)]
pub struct UnconnectedClient;

/// Represents a client that has established a connection to the server but is not yet authenticated.
#[derive(Debug)]
pub struct ConnectedClient {
    /// The TCP stream representing the connection to the server.
    pub stream: TcpStream,
}

/// Represents a client that has been authenticated.
#[derive(Debug)]
pub struct AuthenticatedClient {
    /// The read half of the TCP stream.
    pub read: OwnedReadHalf,

    /// The write half of the TCP stream.
    pub write: OwnedWriteHalf,
}

impl UnconnectedClient {
    /// Connects to the server at the given URL and returns a `ConnectedClient`.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the server to connect to.
    pub async fn connect(self, url: &str) -> Result<ConnectedClient, Error> {
        let mut rng = rand::thread_rng();
        let kyber_key = pqc_kyber::keypair(&mut rng);
        let mut stream = TcpStream::connect(url).await?;
        let setup_security = Message::ClientSetup(ClientSetupSecureConnection::new(
            kyber_key.public.to_vec(),
        ));
        let message = setup_security.setup_for_network()?;

        stream.write_all(&message).await?;
        Ok(ConnectedClient { stream })
    }
}

impl ConnectedClient {
    /// Authenticates the connected client with a username and password.
    ///
    /// # Arguments
    ///
    /// * `username` - The username as a String.
    /// * `password` - The password as a String.
    ///
    /// # Returns
    ///
    /// * `Result<AuthenticatedClient, Error>` - If successful, returns an instance of AuthenticatedClient.
    ///                                          Otherwise, returns an Error indicating what went wrong.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn run_example() -> Result<(), Error> {
    /// let unconnected_client = UnconnectedClient;
    /// let connected_client = unconnected_client.connect("127.0.0.1:12345").await?;
    /// let authenticated_client = connected_client.authenticate("username".to_string(), "password".to_string()).await?;
    /// # Ok(()) }
    /// ```
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
    /// Checks if the client connection is alive.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the connection is alive, `false` otherwise.
    pub fn is_alive(&self) -> bool {
        true
    }

    /// Terminates the connection of the client.
    pub async fn terminate_connection(&mut self) -> Result<(), Error> {
        let message = Message::EndOfCommunication;
        let message = message.setup_for_network()?;
        // debug!("terminate Connection {:?}", message);
        self.write.write_all(&message).await?;
        Ok(())
    }

    /// Inserts data into a specified collection.
    ///
    /// # Arguments
    ///
    /// * `collection` - The name of the collection to insert the data into.
    /// * `data` - The data to be inserted.
    /// * `acl` - The access control list.
    /// * `usecases` - The use cases associated with the data.
    pub async fn insert(
        &mut self,
        collection: String,
        data: Vec<u8>,
        acl: Vec<String>,
        usecases: Vec<String>,
    ) -> Result<String, Error> {
        let encrypt_data = basic_encrypt(&KEY, &NONCE, &data)?;
        let message =
            Message::Insert(Insertion { acl, collection, data: encrypt_data, usecases });
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;
        info!("message: {:?}", message);
        match message {
            Message::InsertResponse { inserted_id } => Ok(inserted_id),
            _ => Err(Error::MessageTypeError(MessageTypeError::default())),
        }
    }

    /// Inserts a number into the database with Order Preserving Encryption (OPE).
    ///
    /// # Arguments
    ///
    /// * `number_to_encrypt` - The number to be encrypted and inserted.
    /// * `acl` - The access control list.
    /// * `usecases` - The use cases associated with the data.
    /// * `collection` - The name of the collection to insert the data into.
    pub async fn insert_ope(
        &mut self,
        number_to_encrypt: f64,
        acl: Vec<String>,
        usecases: Vec<String>,
        collection: String,
    ) -> Result<String, Error> {
        let encrypted_number = encrypt_ope(number_to_encrypt);
        let data = encrypted_number.to_string().as_bytes().to_vec();

        let message = Message::InsertOpe(Insertion { acl, collection, data, usecases });
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;
        info!("message: {:?}", message);
        match message {
            Message::InsertResponse { inserted_id } => Ok(inserted_id),
            _ => Err(Error::MessageTypeError(MessageTypeError::default())),
        }
    }

    /// Queries the database and returns the results.
    ///
    /// # Arguments
    ///
    /// * `query` - The query object representing the database query.
    pub async fn query(&mut self, query: Query) -> Result<QueryResult, Error> {
        let message = Message::Query(query);
        let message = message.setup_for_network()?;
        self.write.write_all(&message).await?;
        let message = parse_message_from_tcp_stream(&mut self.read).await?;
        info!("message: {:?}", message);
        match message {
            Message::QueryResponse { data } => {
                let mut values = Vec::with_capacity(data.len());
                for cipher in data {
                    let value = basic_decrypt(&KEY, &NONCE, &cipher)?;
                    values.push(value);
                }
                Ok(QueryResult::MultipleValues(values))
            }
            Message::SingleValueResponse { data } => {
                if data.is_none() {
                    return Ok(QueryResult::EmptyResult);
                }
                let value = basic_decrypt(
                    &KEY,
                    &NONCE,
                    &data.expect("if is none reutrn empty result"),
                )?;
                Ok(QueryResult::SingleValue(value))
            }
            _ => Err(Error::MessageTypeError(MessageTypeError::default())),
        }
    }

    /// Modifies an existing document in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the document to be modified.
    /// * `collection` - The name of the collection containing the document.
    /// * `new_value` - The new value to be set in the document.
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

    /// Deletes a document from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the document to be deleted.
    /// * `collection` - The name of the collection containing the document.
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

/// Parses a message from a TCP stream.
///
/// # Arguments
///
/// * `stream` - A mutable reference to the read half of a TCP stream.
///
/// # Returns
///
/// * `Result<Message, Error>` - The parsed message, or an error if parsing fails.
pub async fn parse_message_from_tcp_stream(
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
