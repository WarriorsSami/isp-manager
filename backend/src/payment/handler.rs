use crate::error::application::Error;
use crate::payment::repository;
use crate::{invoice, DBPool, Result};
use common::payment::{CreatePaymentRequest, PaymentResponse};
use validator::Validate;
use warp::reply::json;
use warp::{reject, Buf, Reply};

pub async fn list_payments_handler(db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Listing payments");

    let payments = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &payments.into_iter().map(PaymentResponse::from).collect(),
    ))
}

pub async fn fetch_payment_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Fetching payment with id {}", id);

    let payment = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&PaymentResponse::from(payment)))
}

pub async fn create_payment_handler(buf: impl Buf, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Creating a new payment");

    let deserialized = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: CreatePaymentRequest = serde_path_to_error::deserialize(deserialized)
        .map_err(|e| reject::custom(Error::JSONPath(e.to_string())))?;

    body.validate()
        .map_err(|e| reject::custom(Error::Validation(e)))?;

    // check if invoice exists
    let invoice = invoice::repository::fetch_one(&db_pool, body.invoice_id).await;

    if invoice.is_err() {
        return Err(reject::custom(Error::InvoiceNotFound(body.invoice_id)));
    }

    // check if payment date is later than or equal to invoice issue date
    let invoice = invoice.unwrap();

    if body.payment_date < invoice.issue_date {
        return Err(reject::custom(Error::PaymentBeforeInvoiceIssueDate(
            body.payment_date,
            invoice.id,
        )));
    }

    let created_payment = repository::create(&db_pool, body)
        .await
        .map_err(reject::custom)?;

    let response = json(&PaymentResponse::from(created_payment));

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))
}
