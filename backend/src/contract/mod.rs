use crate::{with_db, DBPool};
use warp::Filter;

pub mod handler;
pub mod repository;

pub fn get_routes(
    db_pool: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let contract = warp::path!("contract");
    let contract_param = warp::path!("contract" / u32);

    contract
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_contracts_handler)
        .or(contract_param
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::fetch_contract_handler))
        .or(contract
            .and(warp::post())
            .and(warp::body::aggregate())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_contract_handler))
        .or(contract_param
            .and(warp::put())
            .and(warp::body::aggregate())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_contract_handler))
        .or(contract_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_contract_handler))
}
