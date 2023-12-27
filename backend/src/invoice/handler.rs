use crate::invoice::repository;
use crate::{DBPool, Result};
use common::invoice::{InvoiceRequest, InvoiceResponse};
use warp::reply::json;
use warp::{reject, Reply};

pub async fn list_invoices_handler(db_pool: DBPool) -> Result<impl Reply> {
    let invoices = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &invoices.into_iter().map(InvoiceResponse::from).collect(),
    ))
}

pub async fn fetch_invoice_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    let invoice = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&InvoiceResponse::from(invoice)))
}

pub async fn create_invoice_handler(body: InvoiceRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&InvoiceResponse::from(
        repository::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_invoice_handler(
    id: u32,
    body: InvoiceRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&InvoiceResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_invoice_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    repository::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
