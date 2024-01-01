use crate::customer::repository;
use crate::error::application::Error;
use crate::{DBPool, Result};
use common::contract::ContractResponse;
use common::customer::{CustomerRequest, CustomerResponse};
use common::invoice::InvoiceResponse;
use validator::Validate;
use warp::reply::json;
use warp::{reject, Buf, Reply};

pub async fn list_customers_handler(db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Listing customers");

    let customers = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &customers.into_iter().map(CustomerResponse::from).collect(),
    ))
}

pub async fn fetch_customer_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Fetching customer with id {}", id);

    let customer = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&CustomerResponse::from(customer)))
}

pub async fn list_customer_unpaid_invoices_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Listing unpaid invoices for customer with id {}", id);

    let invoices = repository::fetch_unpaid_invoices(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &invoices.into_iter().map(InvoiceResponse::from).collect(),
    ))
}

pub async fn list_customer_contracts_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Listing contracts for customer with id {}", id);

    let contracts = repository::fetch_contracts(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &contracts.into_iter().map(ContractResponse::from).collect(),
    ))
}

pub async fn create_customer_handler(buf: impl Buf, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Creating a new customer");

    let deserialized = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: CustomerRequest = serde_path_to_error::deserialize(deserialized)
        .map_err(|e| reject::custom(Error::JSONPath(e.to_string())))?;

    body.validate()
        .map_err(|e| reject::custom(Error::Validation(e)))?;

    let created_customer = repository::create(&db_pool, body)
        .await
        .map_err(reject::custom)?;

    let response = json(&CustomerResponse::from(created_customer));

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn update_customer_handler(
    id: u32,
    buf: impl Buf,
    db_pool: DBPool,
) -> Result<impl Reply> {
    log::info!("Updating customer with id {}", id);

    let deserialized = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: CustomerRequest = serde_path_to_error::deserialize(deserialized)
        .map_err(|e| reject::custom(Error::JSONPath(e.to_string())))?;

    body.validate()
        .map_err(|e| reject::custom(Error::Validation(e)))?;

    Ok(json(&CustomerResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_customer_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Deleting customer with id {}", id);

    repository::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
