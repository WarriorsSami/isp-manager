use crate::customer::repository;
use crate::{DBPool, Result};
use common::customer::{CustomerRequest, CustomerResponse};
use warp::reply::json;
use warp::{reject, Reply};

pub async fn list_customers_handler(db_pool: DBPool) -> Result<impl Reply> {
    let customers = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &customers.into_iter().map(CustomerResponse::from).collect(),
    ))
}

pub async fn fetch_customer_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    let customer = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&CustomerResponse::from(customer)))
}

pub async fn create_customer_handler(body: CustomerRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&CustomerResponse::from(
        repository::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_customer_handler(
    id: u32,
    body: CustomerRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&CustomerResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_customer_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    repository::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
