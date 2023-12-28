use crate::contract::repository;
use crate::error::Error;
use crate::{customer, DBPool, Result};
use common::contract::{ContractResponse, CreateContractRequest, UpdateContractRequest};
use warp::reply::json;
use warp::{reject, Reply};

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
        .map_err(|_| reject::custom(Error::ContractNotFound(id)))?;
    Ok(json(&ContractResponse::from(contract)))
}

pub async fn create_contract_handler(
    body: CreateContractRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    println!("Creating a new contract");

    // check if customer exists
    if let Err(_) = customer::repository::fetch_one(&db_pool, body.customer_id).await {
        return Err(reject::custom(Error::CustomerNotFound(body.customer_id)));
    }

    // check if subscription exists
    if let Err(_) = customer::repository::fetch_one(&db_pool, body.subscription_id).await {
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
    body: UpdateContractRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    println!("Updating contract with id {}", id);

    Ok(json(&ContractResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(|_| reject::custom(Error::ContractNotFound(id)))?,
    )))
}

pub async fn delete_contract_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Deleting contract with id {}", id);

    repository::delete(&db_pool, id)
        .await
        .map_err(|_| reject::custom(Error::ContractNotFound(id)))?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
