use crate::customer::repository;
use crate::error::Error;
use crate::{DBPool, Result};
use common::customer::{CustomerRequest, CustomerResponse};
use common::invoice::InvoiceResponse;
use warp::reply::json;
use warp::{reject, Reply};

pub async fn list_customers_handler(db_pool: DBPool) -> Result<impl Reply> {
    println!("Listing customers");

    let customers = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &customers.into_iter().map(CustomerResponse::from).collect(),
    ))
}

pub async fn fetch_customer_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Fetching customer with id {}", id);

    let customer = repository::fetch_one(&db_pool, id)
        .await
        .map_err(|_| reject::custom(Error::CustomerNotFound(id)))?;
    Ok(json(&CustomerResponse::from(customer)))
}

pub async fn list_customer_unpaid_invoices_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Listing unpaid invoices for customer with id {}", id);

    let invoices = repository::fetch_unpaid_invoices(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &invoices.into_iter().map(InvoiceResponse::from).collect(),
    ))
}

pub async fn create_customer_handler(body: CustomerRequest, db_pool: DBPool) -> Result<impl Reply> {
    println!("Creating a new customer");

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
    println!("Updating customer with id {}", id);

    Ok(json(&CustomerResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(|_| reject::custom(Error::CustomerNotFound(id)))?,
    )))
}

pub async fn delete_customer_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Deleting customer with id {}", id);

    repository::delete(&db_pool, id)
        .await
        .map_err(|_| reject::custom(Error::CustomerNotFound(id)))?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
