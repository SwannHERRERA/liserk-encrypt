use futures::{stream, StreamExt};
use shared::query::*;
use tikv_client::RawClient;
use tracing::debug;

use crate::{command::Command, config::TIKV_URL, Error};

pub async fn handle_query(query: Query) -> Result<Command, Error> {
    let client = RawClient::new(vec![TIKV_URL]).await;
    let client = client.expect("failed to connet to tikv");

    match query {
        Query::Single(single_query) => {
            handle_single_query(&client, single_query).await?;
        }
        Query::Compound(compound_query) => {
            handle_compound_query(&client, compound_query).await?;
        }
    }
    Ok(Command::Continue)
}

async fn handle_single_query(
    client: &RawClient,
    single_query: SingleQuery,
) -> Result<(), Error> {
    let key = format!("{}:{}:usecase", single_query.collection, single_query.usecase);

    match client.get(key.clone()).await? {
        Some(value) => {
            println!("Got value for key {}: {:?}", key, value);
        }
        None => {
            println!("No value found for key {}", key);
        }
    }
    Ok(())
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
    client: &RawClient,
    compound_query: CompoundQuery,
) -> Result<(), Error> {
    let keys = retrieve_keys_from_query(&compound_query);
    debug!("keys {:?}", keys);

    match compound_query.query_type {
        QueryType::And => {
            let values = get_kvpair_from_keys(keys, client).await;

            for (key, value) in values {
                match value {
                    Some(value) => {
                        println!("Got value for key {}: {:?}", key, value);
                    }
                    None => {
                        println!("No value found for key {}", key);
                        return Ok(());
                    }
                }
            }
        }
        QueryType::Or => {
            let values = get_kvpair_from_keys(keys, client).await;

            for (key, value) in values {
                if let Some(value) = value {
                    println!("Got value for key {}: {:?}", key, value);
                    return Ok(());
                }
            }
            println!("No values found for keys");
        }
    }
    Ok(())
}

async fn get_kvpair_from_keys(
    keys: Vec<String>,
    client: &RawClient,
) -> Vec<(String, Option<Vec<u8>>)> {
    let values: Vec<(String, Option<Vec<u8>>)> = stream::iter(keys)
        .map(|key| {
            let client = client.clone();
            async move {
                let value = client.get(key.clone()).await.unwrap();
                (key, value)
            }
        })
        .buffer_unordered(num_cpus::get())
        .collect()
        .await;

    values
}
