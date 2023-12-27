use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum InvoiceStatus {
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "UNPAID")]
    Unpaid,
}

impl From<InvoiceStatus> for String {
    fn from(status: InvoiceStatus) -> Self {
        match status {
            InvoiceStatus::Paid => "PAID".to_string(),
            InvoiceStatus::Unpaid => "UNPAID".to_string(),
        }
    }
}

impl From<String> for InvoiceStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "PAID" => InvoiceStatus::Paid,
            "UNPAID" => InvoiceStatus::Unpaid,
            _ => InvoiceStatus::Unpaid,
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Invoice {
    pub id: u32,
    pub contract_id: u32,
    pub issue_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub amount: f64,
    pub status: InvoiceStatus,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InvoiceRequest {
    pub contract_id: u32,
    pub issue_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub amount: f64,
    pub status: InvoiceStatus,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InvoiceResponse {
    pub id: u32,
    pub contract_id: u32,
    pub issue_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub amount: f64,
    pub status: InvoiceStatus,
}

impl From<Invoice> for InvoiceResponse {
    fn from(invoice: Invoice) -> Self {
        InvoiceResponse {
            id: invoice.id,
            contract_id: invoice.contract_id,
            issue_date: invoice.issue_date,
            due_date: invoice.due_date,
            amount: invoice.amount,
            status: invoice.status,
        }
    }
}
