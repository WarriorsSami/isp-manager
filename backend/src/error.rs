use serde::Serialize;
use std::convert::Infallible;
use std::error::Error;
use validator::{ValidationErrors, ValidationErrorsKind};
use warp::{http::StatusCode, Rejection, Reply};

pub mod application {
    use chrono::{DateTime, Utc};
    use r2d2_oracle::r2d2;
    use thiserror::Error;
    use validator::ValidationErrors;

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
        #[error("error executing DB statement")]
        DBStatement,
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
        #[error(
            "invoice (issue_date: {1}, due_date: {2}) not in contract (id: {0}) availability period"
        )]
        InvoiceNotInContractAvailabilityPeriod(u32, DateTime<Utc>, DateTime<Utc>),
        #[error("payment date ({0}) must be later than invoice (id: {1}) issue date")]
        PaymentBeforeInvoiceIssueDate(DateTime<Utc>, u32),
        #[error("JSON path error: {0}")]
        JSONPath(String),
        #[error("validation error: {0}")]
        Validation(ValidationErrors),
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    errors: Option<Vec<FieldError>>,
}

#[derive(Serialize)]
struct FieldError {
    field: String,
    field_errors: Vec<String>,
}

impl warp::reject::Reject for crate::error::application::Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message, errors) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string(), None)
    } else if let Some(body_err) = err.find::<warp::filters::body::BodyDeserializeError>() {
        eprintln!("invalid body: {}", body_err);

        (
            StatusCode::BAD_REQUEST,
            body_err
                .source()
                .map(|cause| cause.to_string())
                .unwrap_or_else(|| "Bad Request".to_string()),
            None,
        )
    } else if let Some(e) = err.find::<application::Error>() {
        match e {
            application::Error::JSONPath(e) => {
                eprintln!("error parsing JSON: {}", e);
                (StatusCode::BAD_REQUEST, e.to_string(), None)
            }
            application::Error::Validation(val_errs) => {
                let errors: Vec<FieldError> = val_errs
                    .errors()
                    .iter()
                    .map(|error_kind| FieldError {
                        field: error_kind.0.to_string(),
                        field_errors: match error_kind.1 {
                            ValidationErrorsKind::Struct(struct_err) => {
                                validation_errs_to_str_vec(struct_err)
                            }
                            ValidationErrorsKind::Field(field_errs) => field_errs
                                .iter()
                                .map(|fe| format!("{}: {:?}", fe.code, fe.params))
                                .collect(),
                            ValidationErrorsKind::List(vec_errs) => vec_errs
                                .iter()
                                .map(|ve| {
                                    format!(
                                        "{}: {:?}",
                                        ve.0,
                                        validation_errs_to_str_vec(ve.1).join(" | "),
                                    )
                                })
                                .collect(),
                        },
                    })
                    .collect();

                (
                    StatusCode::BAD_REQUEST,
                    "field errors".to_string(),
                    Some(errors),
                )
            }
            application::Error::DBQuery(e) => {
                eprintln!("error executing query: {:?}", e);

                match e {
                    oracle::Error::NoDataFound => {
                        (StatusCode::NOT_FOUND, "Not Found".to_string(), None)
                    }
                    oracle::Error::OciError(e) => match e.code() {
                        20001 => (
                            StatusCode::BAD_REQUEST,
                            "The invoice is already paid!".to_string(),
                            None,
                        ),
                        20000 => (
                            StatusCode::BAD_REQUEST,
                            "You cannot pay more than the total amount of the invoice!".to_string(),
                            None,
                        ),
                        _ => (
                            StatusCode::BAD_REQUEST,
                            "Could not execute request".to_string(),
                            None,
                        ),
                    },
                    _ => (
                        StatusCode::BAD_REQUEST,
                        "Could not execute request".to_string(),
                        None,
                    ),
                }
            }
            application::Error::CustomerNotFound(id) => {
                eprintln!("customer not found: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    format!("Customer {} not found", id),
                    None,
                )
            }
            application::Error::ContractNotFound(id) => {
                eprintln!("contract not found: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    format!("Contract {} not found", id),
                    None,
                )
            }
            application::Error::InvoiceNotFound(id) => {
                eprintln!("invoice not found: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    format!("Invoice {} not found", id),
                    None,
                )
            }
            application::Error::PaymentNotFound(id) => {
                eprintln!("payment not found: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    format!("Payment {} not found", id),
                    None,
                )
            }
            application::Error::SubscriptionNotFound(id) => {
                eprintln!("subscription not found: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    format!("Subscription {} not found", id),
                    None,
                )
            }
            application::Error::InvoiceNotInContractAvailabilityPeriod(
                id,
                issue_date,
                due_date,
            ) => {
                eprintln!(
                    "invoice (issue_date: {}, due_date: {}) not in contract (id: {}) availability period",
                    issue_date, due_date, id
                );
                (
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Invoice (issue_date: {}, due_date: {}) not in contract (id: {}) availability period",
                        issue_date, due_date, id
                    ),
                    None,
                )
            }
            application::Error::PaymentBeforeInvoiceIssueDate(payment_date, invoice_id) => {
                eprintln!(
                    "payment date ({}) must be later than invoice (id: {}) issue date",
                    payment_date, invoice_id
                );
                (
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Payment date ({}) must be later than invoice (id: {}) issue date",
                        payment_date, invoice_id
                    ),
                    None,
                )
            }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                    None,
                )
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
            None,
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
            None,
        )
    };

    let json = warp::reply::json(&ErrorResponse { message, errors });

    Ok(warp::reply::with_status(json, code))
}

fn validation_errs_to_str_vec(ve: &ValidationErrors) -> Vec<String> {
    ve.field_errors()
        .iter()
        .map(|fe| {
            format!(
                "{}: errors: {}",
                fe.0,
                fe.1.iter()
                    .map(|ve| format!("{}: {:?}", ve.code, ve.params))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        })
        .collect()
}
