use crate::error::Error;
use crate::payment::repository;
use crate::{invoice, DBPool, Result};
use common::payment::{CreatePaymentRequest, PaymentResponse};
use warp::reply::json;
use warp::{reject, Reply};

pub async fn list_payments_handler(db_pool: DBPool) -> Result<impl Reply> {
    println!("Listing payments");

    let payments = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &payments.into_iter().map(PaymentResponse::from).collect(),
    ))
}

pub async fn fetch_payment_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Fetching payment with id {}", id);

    let payment = repository::fetch_one(&db_pool, id)
        .await
        .map_err(|_| reject::custom(Error::PaymentNotFound(id)))?;
    Ok(json(&PaymentResponse::from(payment)))
}

pub async fn create_payment_handler(
    body: CreatePaymentRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    println!("Creating a new payment");

    // check if invoice exists
    if let Err(_) = invoice::repository::fetch_one(&db_pool, body.invoice_id).await {
        return Err(reject::custom(Error::InvoiceNotFound(body.invoice_id)));
    }

    Ok(json(&PaymentResponse::from(
        repository::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}
