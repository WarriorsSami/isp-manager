use r2d2_oracle::r2d2;
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("error getting connection from DB pool: {0}")]
    DBPool(r2d2::Error),
    #[error("error executing DB query: {0}")]
    DBQuery(#[from] oracle::Error),
    #[error("error creating table: {0}")]
    DBInit(oracle::Error),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("customer {0} not found")]
    CustomerNotFound(u32),
    #[error("contract {0} not found")]
    ContractNotFound(u32),
    #[error("invoice {0} not found")]
    InvoiceNotFound(u32),
    #[error("payment {0} not found")]
    PaymentNotFound(u32),
    #[error("subscription {0} not found")]
    SubscriptionNotFound(u32),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found".to_string();
    } else if let Some(body_err) = err.find::<warp::filters::body::BodyDeserializeError>() {
        eprintln!("invalid body: {}", body_err);
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body".to_string();
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::DBQuery(e) => {
                eprintln!("error executing query: {:?}", e);

                match e {
                    oracle::Error::NoDataFound => {
                        code = StatusCode::NOT_FOUND;
                        message = "Not Found".to_string();
                    }
                    oracle::Error::OciError(e) => match e.code() {
                        20001 => {
                            code = StatusCode::BAD_REQUEST;
                            message = "The invoice is already paid!".to_string();
                        }
                        20000 => {
                            code = StatusCode::BAD_REQUEST;
                            message = "You cannot pay more than the total amount of the invoice!".to_string();
                        }
                        _ => {
                            code = StatusCode::BAD_REQUEST;
                            message = "Could not execute request".to_string();
                        }
                    },
                    _ => {
                        code = StatusCode::BAD_REQUEST;
                        message = "Could not execute request".to_string();
                    }
                }
            }
            Error::CustomerNotFound(id) => {
                eprintln!("customer not found: {}", id);
                code = StatusCode::NOT_FOUND;
                message = format!("Customer {} not found", id);
            }
            Error::ContractNotFound(id) => {
                eprintln!("contract not found: {}", id);
                code = StatusCode::NOT_FOUND;
                message = format!("Contract {} not found", id);
            }
            Error::InvoiceNotFound(id) => {
                eprintln!("invoice not found: {}", id);
                code = StatusCode::NOT_FOUND;
                message = format!("Invoice {} not found", id);
            }
            Error::PaymentNotFound(id) => {
                eprintln!("payment not found: {}", id);
                code = StatusCode::NOT_FOUND;
                message = format!("Payment {} not found", id);
            }
            Error::SubscriptionNotFound(id) => {
                eprintln!("subscription not found: {}", id);
                code = StatusCode::NOT_FOUND;
                message = format!("Subscription {} not found", id);
            }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error".to_string();
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed".to_string();
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error".to_string();
    }

    let json = warp::reply::json(&ErrorResponse {
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
