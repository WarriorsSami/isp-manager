use warp::Filter;
use crate::{DBPool, with_db};

pub mod handler;
mod repository;

pub fn get_invoice_routes(
    db_pool: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let invoice = warp::path!("invoice");
    let invoice_param = warp::path!("invoice" / u32);

    invoice
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_invoices_handler)
        .or(invoice_param
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::fetch_invoice_handler))
        .or(invoice
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_invoice_handler))
        .or(invoice_param
            .and(warp::put())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_invoice_handler))
        .or(invoice_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_invoice_handler))
}