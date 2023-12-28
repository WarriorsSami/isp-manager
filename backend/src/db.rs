use crate::config::CONFIG;
use crate::error::application::Error;
use crate::{DBCon, DBPool};
use r2d2_oracle::{r2d2, OracleConnectionManager};
use std::fs;

pub type Result<T> = std::result::Result<T, Error>;

const DB_POOL_MAX_OPEN: u32 = 32;
const CREATE_TABLES_SQL: &str = "./db-scripts/tables.sql";
const SEEDING_DATA_SQL: &str = "./db-scripts/seeding.sql";
const CREATE_PROCEDURES_SQL: &str = "./db-scripts/procedures.sql";
const CREATE_TRIGGERS_SQL: &str = "./db-scripts/triggers.sql";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let con = get_db_con(db_pool).await?;

    let init_file = fs::read_to_string(CREATE_TABLES_SQL)?;
    con.execute(init_file.as_str(), &[])
        .map_err(Error::DBInit)?;

    let init_file = fs::read_to_string(SEEDING_DATA_SQL)?;
    con.execute(init_file.as_str(), &[])
        .map_err(Error::DBInit)?;

    let init_file = fs::read_to_string(CREATE_PROCEDURES_SQL)?;
    con.execute(init_file.as_str(), &[])
        .map_err(Error::DBInit)?;

    let init_file = fs::read_to_string(CREATE_TRIGGERS_SQL)?;
    con.execute(init_file.as_str(), &[])
        .map_err(Error::DBInit)?;

    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().map_err(Error::DBPool)
}

pub fn create_pool() -> std::result::Result<DBPool, oracle::Error> {
    let config = CONFIG.clone();
    let manager = OracleConnectionManager::new(
        config.db_user.as_str(),
        config.db_pass.as_str(),
        config.db_dsn.as_str(),
    );

    Ok(r2d2::Pool::builder()
        .max_size(DB_POOL_MAX_OPEN)
        .build(manager)
        .expect("database pool can be created"))
}

pub mod customer {
    use common::customer::Customer;
    use oracle::Row;

    pub const TABLE: &str = "customer";
    pub const SELECT_FIELDS: &str = "id, name, fullname, address, phone, cnp";

    pub fn row_to_customer(row: &Row) -> Customer {
        let id: u32 = row.get(0).unwrap();
        let name: String = row.get(1).unwrap();
        let fullname: String = row.get(2).unwrap();
        let address: String = row.get(3).unwrap();
        let phone: String = row.get(4).unwrap();
        let cnp: String = row.get(5).unwrap();

        Customer {
            id,
            name,
            fullname,
            address,
            phone,
            cnp,
        }
    }
}

pub mod subscription {
    use common::subscription::Subscription;
    use oracle::Row;

    pub const TABLE: &str = "subscription";
    pub const SELECT_FIELDS: &str = "id, description, type, traffic, price, extra_traffic_price";

    pub fn row_to_subscription(row: &Row) -> Subscription {
        let id: u32 = row.get(0).unwrap();
        let description: String = row.get(1).unwrap();
        let subscription_type: String = row.get(2).unwrap();
        let traffic: i32 = row.get(3).unwrap();
        let price: f64 = row.get(4).unwrap();
        let extra_traffic_price: f64 = row.get(5).unwrap();

        Subscription {
            id,
            description,
            subscription_type: subscription_type.into(),
            traffic,
            price,
            extra_traffic_price,
        }
    }
}

pub mod contract {
    use chrono::{DateTime, Utc};
    use common::contract::Contract;
    use oracle::Row;

    pub const TABLE: &str = "contract";
    pub const SELECT_FIELDS: &str = "id, customer_id, subscription_id, start_date, end_date";

    pub fn row_to_contract(row: &Row) -> Contract {
        let id: u32 = row.get(0).unwrap();
        let customer_id: u32 = row.get(1).unwrap();
        let subscription_id: u32 = row.get(2).unwrap();
        let start_date: DateTime<Utc> = row.get(3).unwrap();
        let end_date: DateTime<Utc> = row.get(4).unwrap();

        Contract {
            id,
            customer_id,
            subscription_id,
            start_date,
            end_date,
        }
    }
}

pub mod invoice {
    use chrono::{DateTime, Utc};
    use common::invoice::Invoice;
    use oracle::Row;

    pub const TABLE: &str = "invoice";
    pub const SELECT_FIELDS: &str = "id, contract_id, issue_date, due_date, amount, status";

    pub fn row_to_invoice(row: &Row) -> Invoice {
        let id: u32 = row.get(0).unwrap();
        let contract_id: u32 = row.get(1).unwrap();
        let issue_date: DateTime<Utc> = row.get(2).unwrap();
        let due_date: DateTime<Utc> = row.get(3).unwrap();
        let amount: f64 = row.get(4).unwrap();
        let status: String = row.get(5).unwrap();

        Invoice {
            id,
            contract_id,
            issue_date,
            due_date,
            amount,
            status: status.into(),
        }
    }
}

pub mod payment {
    use chrono::{DateTime, Utc};
    use common::payment::Payment;
    use oracle::Row;

    pub const TABLE: &str = "payment";
    pub const SELECT_FIELDS: &str = "id, invoice_id, payment_date, amount";

    pub fn row_to_payment(row: &Row) -> Payment {
        let id: u32 = row.get(0).unwrap();
        let invoice_id: u32 = row.get(1).unwrap();
        let payment_date: DateTime<Utc> = row.get(2).unwrap();
        let amount: f64 = row.get(3).unwrap();

        Payment {
            id,
            invoice_id,
            payment_date,
            amount,
        }
    }
}
