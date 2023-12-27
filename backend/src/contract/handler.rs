use crate::contract::repository;
use crate::{DBPool, Result};
use common::contract::{ContractRequest, ContractResponse};
use warp::reply::json;
use warp::{reject, Reply};

pub async fn list_contracts_handler(db_pool: DBPool) -> Result<impl Reply> {
    let contracts = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &contracts.into_iter().map(ContractResponse::from).collect(),
    ))
}

pub async fn fetch_contract_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    let contract = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&ContractResponse::from(contract)))
}

pub async fn create_contract_handler(body: ContractRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&ContractResponse::from(
        repository::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_contract_handler(
    id: u32,
    body: ContractRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&ContractResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_contract_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    repository::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
