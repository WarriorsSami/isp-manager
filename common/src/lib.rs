pub mod contract;
pub mod customer;
pub mod invoice;
pub mod payment;
pub mod subscription;

pub(crate) mod validation_config {
    use chrono::Utc;
    use lazy_static::lazy_static;
    use regex::Regex;
    use validator::ValidationError;

    lazy_static! {
        pub static ref RE_CNP: Regex = Regex::new(r"^\d{13}$").unwrap();
    }

    pub fn validate_create_contract_request(
        contract: &crate::contract::CreateContractRequest,
    ) -> Result<(), ValidationError> {
        if contract.start_date < Utc::now() {
            return Err(ValidationError::new(
                "Start date should be later than or equal to today",
            ));
        }

        if contract.end_date < Utc::now() {
            return Err(ValidationError::new(
                "End date should be later than or equal to today",
            ));
        }

        if contract.start_date > contract.end_date {
            return Err(ValidationError::new(
                "Start date should be earlier than end date",
            ));
        }

        Ok(())
    }

    pub fn validate_update_contract_request(
        contract: &crate::contract::UpdateContractRequest,
    ) -> Result<(), ValidationError> {
        if contract.start_date < Utc::now() {
            return Err(ValidationError::new(
                "Start date should be later than or equal to today",
            ));
        }

        if contract.end_date < Utc::now() {
            return Err(ValidationError::new(
                "End date should be later than or equal to today",
            ));
        }

        if contract.start_date > contract.end_date {
            return Err(ValidationError::new(
                "Start date should be earlier than end date",
            ));
        }

        Ok(())
    }

    pub fn validate_create_invoice_request(
        invoice: &crate::invoice::CreateInvoiceRequest,
    ) -> Result<(), ValidationError> {
        if invoice.issue_date < Utc::now() {
            return Err(ValidationError::new(
                "Issue date should be later than or equal to today",
            ));
        }

        if invoice.due_date < Utc::now() {
            return Err(ValidationError::new(
                "Due date should be later than or equal to today",
            ));
        }

        if invoice.issue_date > invoice.due_date {
            return Err(ValidationError::new(
                "Issue date should be earlier than due date",
            ));
        }

        Ok(())
    }

    pub fn validate_payment_date(
        payment_date: &chrono::DateTime<Utc>,
    ) -> Result<(), ValidationError> {
        if payment_date < &Utc::now() {
            return Err(ValidationError::new(
                "Payment date should be later than or equal to today",
            ));
        }

        Ok(())
    }
}
