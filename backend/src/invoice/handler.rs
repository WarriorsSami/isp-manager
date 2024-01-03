use crate::error::application::Error;
use crate::invoice::repository;
use crate::{contract, DBPool, Result};
use common::invoice::{CreateInvoiceRequest, InvoiceResponse};
use common::payment::PaymentResponse;
use validator::Validate;
use warp::reply::json;
use warp::{reject, Buf, Reply};

pub async fn list_invoices_handler(db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Listing invoices");

    let invoices = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &invoices.into_iter().map(InvoiceResponse::from).collect(),
    ))
}

pub async fn fetch_invoice_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Fetching invoice with id {}", id);

    let invoice = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&InvoiceResponse::from(invoice)))
}

pub async fn fetch_payments(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Fetching payments for invoice with id {}", id);

    let payments = repository::fetch_payments(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &payments.into_iter().map(PaymentResponse::from).collect(),
    ))
}

pub async fn create_invoice_handler(buf: impl Buf, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Creating a new invoice");

    let deserialized = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: CreateInvoiceRequest = serde_path_to_error::deserialize(deserialized)
        .map_err(|e| reject::custom(Error::JSONPath(e.to_string())))?;

    body.validate()
        .map_err(|e| reject::custom(Error::Validation(e)))?;

    // check if contract exists
    let contract = contract::repository::fetch_one(&db_pool, body.contract_id).await;

    if contract.is_err() {
        return Err(reject::custom(Error::ContractNotFound(body.contract_id)));
    }

    // check if invoice issue date is in contract availability period
    let contract = contract.unwrap();

    if body.issue_date < contract.start_date || body.due_date > contract.end_date {
        log::debug!(
            "Invoice (issue_date: {}, due_date: {}) not in contract (id: {}, start_date: {}, end_date: {}) availability period",
            body.issue_date,
            body.due_date,
            contract.id,
            contract.start_date,
            contract.end_date
        );

        return Err(reject::custom(
            Error::InvoiceNotInContractAvailabilityPeriod(
                contract.id,
                body.issue_date,
                body.due_date,
            ),
        ));
    }

    let created_invoice = repository::create(&db_pool, body)
        .await
        .map_err(reject::custom)?;

    let response = json(&InvoiceResponse::from(created_invoice));

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn delete_invoice_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Deleting invoice with id {}", id);

    repository::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
