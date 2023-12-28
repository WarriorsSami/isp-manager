use crate::validation_config::RE_CNP;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub fullname: String,
    pub address: String,
    pub phone: String,
    pub cnp: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, PartialEq, Debug)]
pub struct CustomerRequest {
    #[validate(length(min = 3, max = 20))]
    pub name: String,
    #[validate(length(min = 3, max = 50))]
    pub fullname: String,
    #[validate(length(min = 3, max = 100))]
    pub address: String,
    #[validate(phone)]
    pub phone: String,
    #[validate(regex = "RE_CNP")]
    pub cnp: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CustomerResponse {
    pub id: u32,
    pub name: String,
    pub fullname: String,
    pub address: String,
    pub phone: String,
    pub cnp: String,
}

impl From<Customer> for CustomerResponse {
    fn from(customer: Customer) -> Self {
        CustomerResponse {
            id: customer.id,
            name: customer.name,
            fullname: customer.fullname,
            address: customer.address,
            phone: customer.phone,
            cnp: customer.cnp,
        }
    }
}
