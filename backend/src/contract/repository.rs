use crate::db::contract::{row_to_contract, SELECT_FIELDS, TABLE};
use crate::db::{get_db_con, Result};
use crate::error::application::Error;
use crate::DBPool;
use common::contract::{Contract, CreateContractRequest, UpdateContractRequest};
use oracle::sql_type::OracleType;

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

pub async fn create(db_pool: &DBPool, body: CreateContractRequest) -> Result<Contract> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (customer_id, subscription_id, start_date, end_date) \
        VALUES (:customer_id, :subscription_id, :start_date, :end_date) RETURNING id INTO :id",
        TABLE
    );

    let stmt = con
        .execute_named(
            query.as_str(),
            &[
                ("customer_id", &body.customer_id),
                ("subscription_id", &body.subscription_id),
                ("start_date", &body.start_date),
                ("end_date", &body.end_date),
                ("id", &OracleType::Number(0, 0)),
            ],
        )
        .map_err(Error::DBQuery)?;

    if let Err(e) = con.commit() {
        con.rollback().map_err(Error::DBQuery)?;
        return Err(Error::DBQuery(e));
    }

    let row_id: u32 = stmt.returned_values("id").map_err(|_| Error::DBStatement)?[0];
    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &row_id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_contract(&row))
}

pub async fn update(db_pool: &DBPool, id: u32, body: UpdateContractRequest) -> Result<Contract> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET start_date = :start_date, end_date = :end_date WHERE id = :id",
        TABLE
    );

    con.execute_named(
        query.as_str(),
        &[
            ("start_date", &body.start_date),
            ("end_date", &body.end_date),
            ("id", &id),
        ],
    )
    .map_err(|e| match e {
        oracle::Error::NoDataFound => Error::ContractNotFound(id),
        _ => Error::DBQuery(e),
    })?;

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
