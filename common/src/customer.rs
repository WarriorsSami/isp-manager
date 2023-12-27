use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub fullname: String,
    pub address: String,
    pub phone: String,
    pub cnp: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CustomerRequest {
    pub name: String,
    pub fullname: String,
    pub address: String,
    pub phone: String,
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
