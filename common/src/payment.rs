use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Payment {
    pub id: u32,
    pub invoice_id: u32,
    pub payment_date: DateTime<Utc>,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Validate, Clone, PartialEq, Debug)]
pub struct CreatePaymentRequest {
    pub invoice_id: u32,
    #[validate(custom(
        function = "crate::validation_config::validate_payment_date",
        message = "Payment date must be later than or equal to today"
    ))]
    pub payment_date: DateTime<Utc>,
    #[validate(range(min = 0.0))]
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PaymentResponse {
    pub id: u32,
    pub invoice_id: u32,
    pub payment_date: DateTime<Utc>,
    pub amount: f64,
}

impl From<Payment> for PaymentResponse {
    fn from(payment: Payment) -> Self {
        PaymentResponse {
            id: payment.id,
            invoice_id: payment.invoice_id,
            payment_date: payment.payment_date,
            amount: payment.amount,
        }
    }
}
