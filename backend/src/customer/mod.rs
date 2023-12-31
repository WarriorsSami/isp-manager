use crate::{with_db, DBPool};
use warp::Filter;

pub mod handler;
pub mod repository;

pub fn get_routes(
    db_pool: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let customer = warp::path!("api" / "customer");
    let customer_param = warp::path!("api" / "customer" / u32);
    let customer_unpaid_invoices = warp::path!("api" / "customer" / u32 / "invoice");
    let customer_contracts = warp::path!("api" / "customer" / u32 / "contract");

    customer
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_customers_handler)
        .or(customer_param
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::fetch_customer_handler))
        .or(customer
            .and(warp::post())
            .and(warp::body::aggregate())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_customer_handler))
        .or(customer_param
            .and(warp::put())
            .and(warp::body::aggregate())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_customer_handler))
        .or(customer_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_customer_handler))
        .or(customer_unpaid_invoices
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::list_customer_unpaid_invoices_handler))
        .or(customer_contracts
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::list_customer_contracts_handler))
}
