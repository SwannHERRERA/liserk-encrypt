use async_channel::Sender;
use liserk_shared::{
    message::{CountSubject, Message, QueryOutput},
    query::*,
};
use rug::Float;
use tikv_client::{KvPair, Transaction, TransactionClient};
use tracing::{debug, error, info};

use crate::{command::Command, config::TIKV_URL, Error};

/// Encrypted data used in Repsonse
pub type EncryptedData = Vec<KvPair>;

/// Nonces Nonce is used for complexifie AES encryption
pub type Nonces = Vec<KvPair>;

/// QueryResponse Represent a query
pub type QueryResponse = (EncryptedData, Option<Nonces>);

pub async fn handle_query(query: Query, tx: Sender<Message>) -> Result<Command, Error> {
    let client = TransactionClient::new(vec![TIKV_URL]).await;
    let client = client.expect("failed to connet to tikv");
    let mut transaction = client.begin_optimistic().await?;
    let message_converter = MessageConverter::default();

    let message = match query {
        Query::Single(single_query) => {
            let data = handle_single_query(&mut transaction, single_query).await?;
            message_converter.convert_to_message(data)
        }
        Query::Compound(compound_query) => {
            let data = handle_compound_query(&mut transaction, compound_query).await?;
            message_converter.convert_to_message(data)
        }
        Query::GetById { id, collection } => {
            let (data, nonce) = get_by_id(&mut transaction, id, collection).await?;
            Message::SingleValueResponse { data, nonce }
        }
        Query::GetByIds { ids, collection } => {
            let (data, nonce) = get_by_ids(&mut transaction, ids, collection).await?;
            let formated = (data, Some(nonce));
            message_converter.convert_to_message(formated)
        }
    };
    transaction.commit().await?;

    info!("data found {:?}", message);
    if let Err(err) = tx.send(message).await {
        error!("error while sending QueryResponse: {:?}", err);
    }
    // Todo send in a channel a message Ok(Command::Continue)
    Ok(Command::Continue)
}

trait TokioSender {
    fn serialize_kv_pairs(pairs: &Vec<KvPair>) -> Vec<Vec<u8>> {
        let mut serialized_pairs = Vec::new();
        for pair in pairs.iter() {
            serialized_pairs.push(pair.1.clone());
        }
        serialized_pairs
    }

    fn convert_to_output(&self, response: QueryResponse) -> QueryOutput {
        let (encrypted_data, nonces_option) = response;

        let serialized_encrypted_data = Self::serialize_kv_pairs(&encrypted_data);

        let serialized_nonces =
            nonces_option.as_ref().map(|nonces| Self::serialize_kv_pairs(&nonces));

        (serialized_encrypted_data, serialized_nonces)
    }

    fn convert_to_message(&self, response: QueryResponse) -> Message {
        let output = self.convert_to_output(response);
        Message::QueryResponse(output)
    }
}

#[derive(Debug, Default)]
struct MessageConverter {}

impl TokioSender for MessageConverter {}

async fn get_by_id(
    client: &mut Transaction,
    id: String,
    collection: String,
) -> Result<(Option<Vec<u8>>, Option<Vec<u8>>), Error> {
    let key = format!("{}:{}", collection, id);
    let key_nonce = format!("{}:{}:nonce", collection, id);
    let data = client.get(key).await?;
    let nonce = client.get(key_nonce).await?;
    Ok((data, nonce))
}

async fn get_by_ids(
    client: &mut Transaction,
    ids: Vec<String>,
    collection: String,
) -> Result<(Vec<KvPair>, Vec<KvPair>), Error> {
    let keys: Vec<String> =
        ids.iter().map(|id| format!("{}:{}", collection, id)).collect();
    let results = fetch_data_from_keys(client, keys.clone()).await?;
    let nonces = fetch_nonce_from_keys(client, keys).await?;

    Ok((results, nonces))
}

async fn handle_single_query(
    client: &mut Transaction,
    single_query: SingleQuery,
) -> Result<QueryResponse, Error> {
    let key = format!("{}:{}:usecase", single_query.collection, single_query.usecase);
    info!("key: {}", key);

    match client.get(key.clone()).await? {
        Some(value) => {
            println!("Got value for key {}: {:?}", key, value);
            let data_keys = extract_data_keys_from_value(value)?;
            let mut results = fetch_data_from_keys(client, data_keys.clone()).await?;
            if is_ope_query(&single_query) {
                results =
                    filter_results_by_upper_limit(results, single_query.upper_limit);
                results =
                    filter_results_by_lower_limit(results, single_query.lower_limit);
            } else {
                let nonce = fetch_nonce_from_keys(client, data_keys).await?;

                return Ok((results, Some(nonce)));
            }

            Ok((results, None))
        }
        None => {
            debug!("No value found for key {}", key);
            Ok((Vec::new(), None))
        }
    }
}

fn is_ope_query(query: &SingleQuery) -> bool {
    query.upper_limit.is_some() || query.lower_limit.is_some()
}

fn extract_data_keys_from_value(value: Vec<u8>) -> Result<Vec<String>, Error> {
    let data_keys: Vec<String> = serde_cbor::from_slice::<Vec<Vec<u8>>>(&value)?
        .iter()
        .map(|data_key| String::from_utf8_lossy(data_key).to_string())
        .collect();
    Ok(data_keys)
}

async fn fetch_data_from_keys(
    client: &mut Transaction,
    data_keys: Vec<String>,
) -> Result<Vec<KvPair>, Error> {
    Ok(client.batch_get(data_keys).await?.collect())
}

async fn fetch_nonce_from_keys(
    client: &mut Transaction,
    data_keys: Vec<String>,
) -> Result<Vec<KvPair>, Error> {
    let nonce_key: Vec<String> =
        data_keys.iter().map(|key| key.to_owned() + ":nonce").collect();
    Ok(client.batch_get(nonce_key).await?.collect())
}

fn filter_results_by_upper_limit(
    mut results: Vec<KvPair>,
    upper_limit: Option<f64>,
) -> Vec<KvPair> {
    if let Some(upper_limit) = upper_limit {
        let upper_limit = Float::with_val(53, upper_limit); // 53 bits precision (similar to f64)
        results = results
            .into_iter()
            .filter(|kv_pair| {
                let deserialized_value: f64 =
                    serde_cbor::from_slice(&kv_pair.1).unwrap_or(0.0);
                let value_as_float = Float::with_val(53, deserialized_value);
                value_as_float <= upper_limit
            })
            .collect();
    }
    results
}

fn filter_results_by_lower_limit(
    mut results: Vec<KvPair>,
    lower_limit: Option<f64>,
) -> Vec<KvPair> {
    if let Some(lower_limit) = lower_limit {
        let lower_limit = Float::with_val(53, lower_limit); // 53 bits precision (similar to f64)
        results = results
            .into_iter()
            .filter(|kv_pair| {
                let deserialized_value: f64 =
                    serde_cbor::from_slice(&kv_pair.1).unwrap_or(0.0);
                let value_as_float = Float::with_val(53, deserialized_value);
                value_as_float >= lower_limit
            })
            .collect();
    }
    results
}

fn retrieve_keys_from_query(compound_query: &CompoundQuery) -> Vec<String> {
    compound_query
        .queries
        .iter()
        .filter_map(|query| match query {
            Query::Single(single_query) => {
                Some(format!("{}:{}", single_query.collection, single_query.usecase))
            }
            _ => None,
        })
        .collect()
}

async fn handle_compound_query(
    client: &mut Transaction,
    compound_query: CompoundQuery,
) -> Result<(Vec<KvPair>, Option<Vec<KvPair>>), Error> {
    let keys = retrieve_keys_from_query(&compound_query);
    debug!("keys {:?}", keys);

    match compound_query.query_type {
        QueryType::And => {
            let values = get_kvpair_from_keys(keys, client).await?;

            for (key, value) in values {
                println!("Got value for key {}: {:?}", key, value);
                let data_keys: Vec<String> =
                    serde_cbor::from_slice::<Vec<Vec<u8>>>(&value)?
                        .iter()
                        .map(|data_key| String::from_utf8_lossy(data_key).to_string())
                        .collect();
                let data = fetch_data_from_keys(client, data_keys.clone()).await?;
                let nonce = fetch_nonce_from_keys(client, data_keys).await?;
                return Ok((data, Some(nonce)));
            }
            debug!("No values found for keys");
            Ok((Vec::new(), None))
        }
        QueryType::Or => {
            let values = get_kvpair_from_keys(keys, client).await?;

            for (key, value) in values {
                debug!("Got value for key {}: {:?}", key, value);
                let data_keys: Vec<String> =
                    serde_cbor::from_slice::<Vec<Vec<u8>>>(&value)?
                        .iter()
                        .map(|data_key| String::from_utf8_lossy(data_key).to_string())
                        .collect();
                let data = fetch_data_from_keys(client, data_keys.clone()).await?;
                let nonce = fetch_nonce_from_keys(client, data_keys).await?;
                return Ok((data, Some(nonce)));
            }
            debug!("No values found for keys");
            Ok((Vec::new(), None))
        }
    }
}

async fn get_kvpair_from_keys(
    keys: Vec<String>,
    transaction: &mut Transaction,
) -> Result<Vec<(String, Vec<u8>)>, Error> {
    let kv_pairs = transaction.batch_get(keys).await?;
    let kv_pairs = kv_pairs
        .map(|pair| {
            let key = pair.0;
            let key_string: String = String::from_utf8_lossy((&key).into()).to_string();
            let value = pair.1;
            (key_string, value)
        })
        .collect();
    Ok(kv_pairs)
}

pub async fn count(count: CountSubject, tx: Sender<Message>) -> Result<Command, Error> {
    let key = match count {
        CountSubject::Collection(collection) => {
            format!("{}:keys", collection)
        }
        CountSubject::Usecase { collection, usecase } => {
            format!("{}:{}:usecase", collection, usecase)
        }
    };
    let client = TransactionClient::new(vec![TIKV_URL]).await?;
    let mut transaction = client.begin_optimistic().await?;
    let values = transaction.get(key).await?;
    transaction.commit().await?;
    let length = compute_length_of_cell(values)?;
    tx.send(Message::CountResponse(length)).await?;
    Ok(Command::Continue)
}

fn compute_length_of_cell(values: Option<Vec<u8>>) -> Result<u32, Error> {
    if values.is_none() {
        return Ok(0);
    }
    Ok(serde_cbor::from_slice::<Vec<Vec<u8>>>(
        &values.expect("values is Some check is none before"),
    )?
    .iter()
    .count() as u32)
}
