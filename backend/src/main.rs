use r2d2_oracle::{r2d2, OracleConnectionManager};
use std::convert::Infallible;
use warp::{
    http::{header, Method},
    Filter, Rejection,
};

mod config;
mod contract;
mod customer;
mod db;
mod error;
mod invoice;
mod payment;
mod subscription;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = r2d2::PooledConnection<OracleConnectionManager>;
type DBPool = r2d2::Pool<OracleConnectionManager>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_pool = db::create_pool().expect("database pool can be created");

    // db::init_db(&db_pool)
    //     .await
    //     .expect("database can be initialized");

    let customer_routes = customer::get_routes(db_pool.clone());
    let subscription_routes = subscription::get_routes(db_pool.clone());
    let contract_routes = contract::get_routes(db_pool.clone());
    let invoice_routes = invoice::get_routes(db_pool.clone());
    let payment_routes = payment::get_routes(db_pool.clone());

    let routes = customer_routes
        .or(subscription_routes)
        .or(contract_routes)
        .or(invoice_routes)
        .or(payment_routes)
        .recover(error::handle_rejection)
        .with(
            warp::cors()
                .allow_credentials(true)
                .allow_methods(&[
                    Method::OPTIONS,
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                ])
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .expose_headers(vec![header::LINK])
                .max_age(300)
                .allow_any_origin(),
        );

    log::info!("Listening on port :{}", 8000);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
