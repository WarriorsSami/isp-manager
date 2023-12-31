use crate::{with_db, DBPool};
use warp::Filter;

pub mod handler;
pub mod repository;

pub fn get_routes(
    db_pool: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let payment = warp::path!("api" / "payment");
    let payment_param = warp::path!("api" / "payment" / u32);

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
            .and(warp::body::aggregate())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_payment_handler))
}
