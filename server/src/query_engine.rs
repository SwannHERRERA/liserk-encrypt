use async_channel::Sender;
use shared::{message::Message, query::*};
use tikv_client::{KvPair, Transaction, TransactionClient};
use tracing::{debug, info};

use crate::{command::Command, config::TIKV_URL, Error};

pub async fn handle_query(query: Query, tx: Sender<Message>) -> Result<Command, Error> {
    let client = TransactionClient::new(vec![TIKV_URL]).await;
    let client = client.expect("failed to connet to tikv");
    let mut transaction = client.begin_optimistic().await?;

    let x = match query {
        Query::Single(single_query) => {
            handle_single_query(&mut transaction, single_query).await?
        }
        Query::Compound(compound_query) => {
            handle_compound_query(&mut transaction, compound_query).await?
        }
    };
    transaction.commit().await?;

    info!("data found {:?}", x);
    // Todo send in a channel a message
    Ok(Command::Continue)
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
