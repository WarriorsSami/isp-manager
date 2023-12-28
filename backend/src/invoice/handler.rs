use crate::error::Error;
use crate::invoice::repository;
use crate::{contract, DBPool, Result};
use common::invoice::{CreateInvoiceRequest, InvoiceResponse, UpdateInvoiceRequest};
use warp::reply::json;
use warp::{reject, Reply};

pub async fn list_invoices_handler(db_pool: DBPool) -> Result<impl Reply> {
    println!("Listing invoices");

    let invoices = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &invoices.into_iter().map(InvoiceResponse::from).collect(),
    ))
}

pub async fn fetch_invoice_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Fetching invoice with id {}", id);

    let invoice = repository::fetch_one(&db_pool, id)
        .await
        .map_err(|_| reject::custom(Error::InvoiceNotFound(id)))?;
    Ok(json(&InvoiceResponse::from(invoice)))
}

pub async fn create_invoice_handler(
    body: CreateInvoiceRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    println!("Creating a new invoice");

    // check if contract exists
    if let Err(_) = contract::repository::fetch_one(&db_pool, body.contract_id).await {
        return Err(reject::custom(Error::ContractNotFound(body.contract_id)));
    }

    Ok(json(&InvoiceResponse::from(
        repository::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_invoice_handler(
    id: u32,
    body: UpdateInvoiceRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    println!("Updating invoice with id {}", id);

    Ok(json(&InvoiceResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(|_| reject::custom(Error::InvoiceNotFound(id)))?,
    )))
}

pub async fn delete_invoice_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Deleting invoice with id {}", id);

    repository::delete(&db_pool, id)
        .await
        .map_err(|_| reject::custom(Error::InvoiceNotFound(id)))?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
