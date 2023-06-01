use async_channel::Sender;
use shared::message::{
    ClientAuthentication, ClientSetupSecureConnection, CountSubject, Delete, Insertion,
    Message, Update,
};
use shared::query::Query;
use tracing::debug;
use tracing::{error, info};

use crate::command::Command;
use crate::mutation;
use crate::query_engine;

pub async fn parse_message(message: Message, tx: Sender<Message>) -> Command {
    match message {
        Message::ClientSetup(param) => parse_client_setup(param),
        Message::ClientAuthentification(param) => parse_authentification(param),
        Message::Insert(param) => insert(param, tx).await,
        Message::Query(param) => handle_query(param, tx).await,
        Message::Count(param) => count(param, tx).await,
        Message::Update(param) => update(param, tx).await,
        Message::Delete(param) => delete(param, tx).await,
        Message::DeleteForUsecase { collection, id } => todo!(),
        Message::Drop(_) => todo!(),
        Message::EndOfCommunication => end_communication(tx).await,
        Message::DeleteResult(_) => unreachable!(),
        Message::InsertResponse { .. } => unreachable!(),
        Message::QueryResponse { .. } => unreachable!(),
        Message::SingleValueResponse { .. } => unreachable!(),
        Message::CloseCommunication => unreachable!(),
        Message::UpdateResponse { .. } => unreachable!(),
        Message::DropResult(_) => unreachable!(),
        Message::CountResponse(_) => todo!(),
    }
}

async fn count(param: CountSubject, tx: Sender<Message>) -> Command {
    let command = query_engine::count(param, tx).await;
    if command.is_err() {
        error!("error in count: {:?}", command.unwrap_err());
        return Command::Continue;
    }
    command.expect("error checked before")
}

async fn update(query: Update, tx: Sender<Message>) -> Command {
    let status = match mutation::update(query).await {
        Ok(status) => status,
        Err(_) => shared::message::UpdateStatus::Failure,
    };
    if let Err(err) = tx.send(Message::UpdateResponse { status }).await {
        error!("err while sending update response: {:?}", err);
    }
    Command::Continue
}

async fn delete(delete: Delete, tx: Sender<Message>) -> Command {
    let result = mutation::delete(delete).await.unwrap_or(false);
    if let Err(err) = tx.send(Message::DeleteResult(result)).await {
        error!("delete message: {:?}", err);
    }
    Command::Continue
}

fn parse_authentification(authentification: ClientAuthentication) -> Command {
    info!("authentification: {:?}", authentification);
    Command::Continue
}

fn parse_client_setup(secure_connection_message: ClientSetupSecureConnection) -> Command {
    info!("secure message: {:?}", secure_connection_message);
    Command::Continue
}

async fn end_communication(tx: Sender<Message>) -> Command {
    if let Err(err) = tx.send(Message::CloseCommunication).await {
        error!("err while shutdown communication: {:?}", err);
    }
    Command::Exit
}

async fn insert(insertion: Insertion, tx: Sender<Message>) -> Command {
    match mutation::insert(insertion).await {
        Ok(inserted_id) => {
            debug!("inserted uuid: {}", inserted_id);
            if let Err(err) = tx.send(Message::InsertResponse { inserted_id }).await {
                error!("err: {:?}", err);
            }
        }
        Err(err) => debug!("{:?}", err),
    }
    Command::Continue
}

async fn handle_query(query: Query, tx: Sender<Message>) -> Command {
    match query_engine::handle_query(query, tx).await {
        Ok(command) => command,
        Err(err) => {
            error!("{:?}", err);
            Command::Exit
        }
    }
}
