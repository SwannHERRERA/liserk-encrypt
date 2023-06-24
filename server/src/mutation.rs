use liserk_shared::message::{Delete, Insertion, Update, UpdateStatus};
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
    transaction.insert(data_key.clone(), insertion.data).await?;

    let acl_key = format!("{}:{}:acl", insertion.collection, unique_id);
    let acl_json = serde_cbor::to_vec(&insertion.acl)?;
    transaction.insert(acl_key, acl_json).await?;

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
    info!("insert commit: {:?}", commit);
    Ok(unique_id)
}

pub async fn update(query: Update) -> Result<UpdateStatus, Error> {
    let client = TransactionClient::new(vec![TIKV_URL]).await?;

    let data_key = format!("{}:{}", query.collection, query.id);
    info!("data_key: {}", data_key);

    let mut transaction = client.begin_optimistic().await?;
    let Some(_) = transaction.get_for_update(data_key.clone()).await? else {
        let _ = transaction.commit().await?;
        return Ok(UpdateStatus::KeyNotFound);
    };
    transaction.put(data_key, query.new_value).await?;
    let commit = transaction.commit().await?;
    info!("update commit: {:?}", commit);
    Ok(UpdateStatus::Success)
}

pub async fn delete(query: Delete) -> Result<bool, Error> {
    let client = TransactionClient::new(vec![TIKV_URL]).await?;
    let key = format!("{}:{}", query.collection, query.id);
    let mut transaction = client.begin_optimistic().await?;
    let is_deleted = match transaction.delete(key).await {
        Ok(_) => true,
        Err(_) => false,
    };
    let commit = transaction.commit().await?;
    info!("delet commit: {:?}", commit);
    Ok(is_deleted)
}
