use crate::db::invoice::{row_to_invoice, SELECT_FIELDS, TABLE};
use crate::db::payment::row_to_payment;
use crate::db::{get_db_con, Result};
use crate::error::application::Error;
use crate::DBPool;
use common::invoice::{CreateInvoiceRequest, Invoice};
use common::payment::Payment;
use oracle::sql_type::OracleType;

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<Invoice>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);

    let rows = con.query(query.as_str(), &[]).map_err(Error::DBQuery)?;

    Ok(rows
        .filter(|r| r.is_ok())
        .map(|r| row_to_invoice(&r.unwrap()))
        .collect())
}

pub async fn fetch_one(db_pool: &DBPool, id: u32) -> Result<Invoice> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_invoice(&row))
}

pub async fn create(db_pool: &DBPool, body: CreateInvoiceRequest) -> Result<Invoice> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (contract_id, issue_date, due_date, amount) \
        VALUES (:contract_id, :issue_date, :due_date, :amount) RETURNING id into :id",
        TABLE
    );

    let stmt = con
        .execute_named(
            query.as_str(),
            &[
                ("contract_id", &body.contract_id),
                ("issue_date", &body.issue_date),
                ("due_date", &body.due_date),
                ("amount", &body.amount),
                ("id", &OracleType::Number(0, 0)),
            ],
        )
        .map_err(Error::DBQuery)?;

    if let Err(e) = con.commit() {
        con.rollback().map_err(Error::DBQuery)?;
        return Err(Error::DBQuery(e));
    }

    let row_id: u32 = stmt.returned_values("id").map_err(Error::DBQuery)?[0];
    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &row_id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_invoice(&row))
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

pub async fn fetch_payments(db_pool: &DBPool, id: u32) -> Result<Vec<Payment>> {
    use crate::db::payment::SELECT_FIELDS;

    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM GET_PAYMENTS(:id)", SELECT_FIELDS);

    let rows = con
        .query_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(rows
        .filter(|r| r.is_ok())
        .map(|r| row_to_payment(&r.unwrap()))
        .collect())
}
