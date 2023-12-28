use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Contract {
    pub id: u32,
    pub customer_id: u32,
    pub subscription_id: u32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Validate, Clone, PartialEq, Debug)]
#[validate(schema(function = "crate::validation_config::validate_create_contract_request"))]
pub struct CreateContractRequest {
    pub customer_id: u32,
    pub subscription_id: u32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Validate, Clone, PartialEq, Debug)]
#[validate(schema(function = "crate::validation_config::validate_update_contract_request"))]
pub struct UpdateContractRequest {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ContractResponse {
    pub id: u32,
    pub customer_id: u32,
    pub subscription_id: u32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl From<Contract> for ContractResponse {
    fn from(contract: Contract) -> Self {
        ContractResponse {
            id: contract.id,
            customer_id: contract.customer_id,
            subscription_id: contract.subscription_id,
            start_date: contract.start_date,
            end_date: contract.end_date,
        }
    }
}
