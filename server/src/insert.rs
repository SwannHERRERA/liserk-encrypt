use shared::message::Insertion;
use tikv_client::TransactionClient;
use tracing::info;
use uuid::Uuid;

use crate::{config::TIKV_URL, Error};

pub async fn insert(insertion: Insertion) -> Result<String, Error> {
    let client = TransactionClient::new(vec![TIKV_URL]).await?;

    let unique_id = Uuid::new_v4().to_string();

    let data_key = format!("{}:{}", insertion.collection, unique_id);
    info!("data_key: {}", data_key);

    let mut transaction = client.begin_optimistic().await?;
    transaction.put(data_key.clone(), insertion.data).await?;

    let acl_key = format!("{}:{}:acl", insertion.collection, unique_id);
    let acl_json = serde_cbor::to_vec(&insertion.acl)?;
    transaction.put(acl_key, acl_json).await?;

    for usecase in insertion.usecases {
        let usecase_key = format!("{}:{}:usecase", insertion.collection, usecase);
        info!("usecase_key: {}", usecase_key);
        let values = match transaction.get(usecase_key.clone()).await? {
            Some(value) => {
                let mut values: Vec<Vec<u8>> = serde_cbor::from_slice(&value)?;
                values.push(data_key.clone().into_bytes());
                values
            }
            None => {
                vec![data_key.clone().into_bytes()]
            }
        };
        let bytes = serde_cbor::to_vec(&values)?;
        transaction.put(usecase_key, bytes).await?;
    }
    let commit = transaction.commit().await?;
    info!("{:?}", commit);
    Ok(unique_id)
}
