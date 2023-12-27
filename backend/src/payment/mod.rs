use warp::Filter;
use crate::{DBPool, with_db};

mod repository;
pub mod handler;

pub fn get_payment_routes(
    db_pool: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let payment = warp::path!("payment");
    let payment_param = warp::path!("payment" / u32);

    payment
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_payments_handler)
        .or(payment_param
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::fetch_payment_handler))
        .or(payment
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_payment_handler))
        .or(payment_param
            .and(warp::put())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_payment_handler))
        .or(payment_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_payment_handler))
}