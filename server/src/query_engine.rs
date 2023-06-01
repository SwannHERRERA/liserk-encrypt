use async_channel::Sender;
use rayon::prelude::*;
use shared::{message::Message, query::*};
use tikv_client::{KvPair, Transaction, TransactionClient};
use tracing::{debug, error, info};

use crate::{command::Command, config::TIKV_URL, Error};

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
            let data = get_by_id(&mut transaction, id, collection).await?;
            Message::SingleValueResponse { data }
        }
        Query::GetByIds { ids, collection } => {
            let data = get_by_ids(&mut transaction, ids, collection).await?;
            message_converter.convert_to_message(data)
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
    fn convert_to_bytes(&self, pairs: Vec<KvPair>) -> Vec<Vec<u8>> {
        pairs.into_par_iter().map(|pair| pair.1).collect()
    }

    fn convert_to_message(&self, pairs: Vec<KvPair>) -> Message {
        Message::QueryResponse { data: self.convert_to_bytes(pairs) }
    }
}

#[derive(Debug, Default)]
struct MessageConverter {}

impl TokioSender for MessageConverter {}

async fn get_by_id(
    client: &mut Transaction,
    id: String,
    collection: String,
) -> Result<Option<Vec<u8>>, Error> {
    let key = format!("{}:{}", collection, id);
    Ok(client.get(key).await?)
}

async fn get_by_ids(
    client: &mut Transaction,
    ids: Vec<String>,
    collection: String,
) -> Result<Vec<KvPair>, Error> {
    let keys: Vec<String> =
        ids.iter().map(|id| format!("{}:{}", collection, id)).collect();
    Ok(client.batch_get(keys).await?.collect())
}

async fn handle_single_query(
    client: &mut Transaction,
    single_query: SingleQuery,
) -> Result<Vec<KvPair>, Error> {
    let key = format!("{}:{}:usecase", single_query.collection, single_query.usecase);
    info!("key: {}", key);

    match client.get(key.clone()).await? {
        Some(value) => {
            println!("Got value for key {}: {:?}", key, value);
            let data_keys: Vec<String> = serde_cbor::from_slice::<Vec<Vec<u8>>>(&value)?
                .iter()
                .map(|data_key| String::from_utf8_lossy(data_key).to_string())
                .collect();
            Ok(client.batch_get(data_keys).await?.collect())
        }
        None => {
            debug!("No value found for key {}", key);
            Ok(Vec::new())
        }
    }
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
) -> Result<Vec<KvPair>, Error> {
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
                return Ok(client.batch_get(data_keys).await?.collect());
            }
            debug!("No values found for keys");
            Ok(Vec::new())
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
                return Ok(client.batch_get(data_keys).await?.collect());
            }
            debug!("No values found for keys");
            Ok(Vec::new())
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
