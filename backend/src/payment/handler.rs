use crate::payment::repository;
use crate::{DBPool, Result};
use common::payment::{PaymentRequest, PaymentResponse};
use warp::reply::json;
use warp::{reject, Reply};

pub async fn list_payments_handler(db_pool: DBPool) -> Result<impl Reply> {
    let payments = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &payments.into_iter().map(PaymentResponse::from).collect(),
    ))
}

pub async fn fetch_payment_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    let payment = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&PaymentResponse::from(payment)))
}

pub async fn create_payment_handler(body: PaymentRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&PaymentResponse::from(
        repository::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_payment_handler(
    id: u32,
    body: PaymentRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&PaymentResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_payment_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    repository::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
