use crate::{with_db, DBPool};
use warp::Filter;

pub mod handler;
mod repository;

pub fn get_customer_routes(
    db_pool: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let customer = warp::path!("customer");
    let customer_param = warp::path!("customer" / u32);

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
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_customer_handler))
        .or(customer_param
            .and(warp::put())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_customer_handler))
        .or(customer_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_customer_handler))
}