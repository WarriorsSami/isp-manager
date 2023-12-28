use crate::contract::repository;
use crate::error::application::Error;
use crate::{customer, DBPool, Result};
use common::contract::{ContractResponse, CreateContractRequest, UpdateContractRequest};
use validator::Validate;
use warp::reply::json;
use warp::{reject, Buf, Reply};

pub async fn list_contracts_handler(db_pool: DBPool) -> Result<impl Reply> {
    println!("Listing contracts");

    let contracts = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &contracts.into_iter().map(ContractResponse::from).collect(),
    ))
}

pub async fn fetch_contract_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Fetching contract with id {}", id);

    let contract = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&ContractResponse::from(contract)))
}

pub async fn create_contract_handler(buf: impl Buf, db_pool: DBPool) -> Result<impl Reply> {
    println!("Creating a new contract");

    let deserialized = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: CreateContractRequest = serde_path_to_error::deserialize(deserialized)
        .map_err(|e| reject::custom(Error::JSONPath(e.to_string())))?;

    body.validate()
        .map_err(|e| reject::custom(Error::Validation(e)))?;

    // check if customer exists
    if customer::repository::fetch_one(&db_pool, body.customer_id)
        .await
        .is_err()
    {
        return Err(reject::custom(Error::CustomerNotFound(body.customer_id)));
    }

    // check if subscription exists
    if customer::repository::fetch_one(&db_pool, body.subscription_id)
        .await
        .is_err()
    {
        return Err(reject::custom(Error::SubscriptionNotFound(
            body.subscription_id,
        )));
    }

    Ok(json(&ContractResponse::from(
        repository::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_contract_handler(
    id: u32,
    buf: impl Buf,
    db_pool: DBPool,
) -> Result<impl Reply> {
    println!("Updating contract with id {}", id);

    let deserialized = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: UpdateContractRequest = serde_path_to_error::deserialize(deserialized)
        .map_err(|e| reject::custom(Error::JSONPath(e.to_string())))?;

    body.validate()
        .map_err(|e| reject::custom(Error::Validation(e)))?;

    Ok(json(&ContractResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_contract_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Deleting contract with id {}", id);

    repository::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
