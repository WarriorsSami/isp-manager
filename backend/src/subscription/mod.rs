use crate::{with_db, DBPool};
use warp::Filter;

pub mod handler;
pub mod repository;

pub fn get_routes(
    db_pool: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let subscription = warp::path!("subscription");
    let subscription_param = warp::path!("subscription" / u32);

    subscription
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_subscriptions_handler)
        .or(subscription_param
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::fetch_subscription_handler))
        .or(subscription
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_subscription_handler))
        .or(subscription_param
            .and(warp::put())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_subscription_handler))
        .or(subscription_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_subscription_handler))
}
