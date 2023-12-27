use crate::db::{get_db_con, Result};
use crate::error::Error;
use crate::DBPool;
use chrono::{DateTime, Utc};
use common::contract::{Contract, ContractRequest};
use oracle::Row;

const TABLE: &str = "contract";
const SELECT_FIELDS: &str = "id, customer_id, subscription_id, start_date, end_date";

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<Contract>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);

    let rows = con.query(query.as_str(), &[]).map_err(Error::DBQuery)?;

    Ok(rows
        .filter(|r| r.is_ok())
        .map(|r| row_to_contract(&r.unwrap()))
        .collect())
}

pub async fn fetch_one(db_pool: &DBPool, id: u32) -> Result<Contract> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_contract(&row))
}

pub async fn create(db_pool: &DBPool, body: ContractRequest) -> Result<Contract> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (customer_id, subscription_id, start_date, end_date) VALUES (:customer_id, :subscription_id, :start_date, :end_date)", TABLE);

    con.execute_named(
        query.as_str(),
        &[
            ("customer_id", &body.customer_id),
            ("subscription_id", &body.subscription_id),
            ("start_date", &body.start_date),
            ("end_date", &body.end_date),
        ],
    )
    .map_err(Error::DBQuery)?;

    if let Err(e) = con.commit() {
        con.rollback().map_err(Error::DBQuery)?;
        return Err(Error::DBQuery(e));
    }

    let query = format!(
        "SELECT {} FROM {} ORDER BY id DESC FETCH FIRST ROW ONLY",
        SELECT_FIELDS, TABLE
    );

    let row = con
        .query_row_named(query.as_str(), &[])
        .map_err(Error::DBQuery)?;

    Ok(row_to_contract(&row))
}

pub async fn update(db_pool: &DBPool, id: u32, body: ContractRequest) -> Result<Contract> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET customer_id = :customer_id, subscription_id = :subscription_id, start_date = :start_date, end_date = :end_date WHERE id = :id", TABLE);

    con.execute_named(
        query.as_str(),
        &[
            ("customer_id", &body.customer_id),
            ("subscription_id", &body.subscription_id),
            ("start_date", &body.start_date),
            ("end_date", &body.end_date),
            ("id", &id),
        ],
    )
    .map_err(Error::DBQuery)?;

    if let Err(e) = con.commit() {
        con.rollback().map_err(Error::DBQuery)?;
        return Err(Error::DBQuery(e));
    }

    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_contract(&row))
}

pub async fn delete(db_pool: &DBPool, id: u32) -> Result<()> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = :id", TABLE);

    con.execute_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    if let Err(e) = con.commit() {
        con.rollback().map_err(Error::DBQuery)?;
        return Err(Error::DBQuery(e));
    }

    Ok(())
}

fn row_to_contract(row: &Row) -> Contract {
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
