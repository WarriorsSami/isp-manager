use crate::db::invoice::{row_to_invoice, SELECT_FIELDS, TABLE};
use crate::db::{get_db_con, Result};
use crate::error::Error;
use crate::DBPool;
use common::invoice::{CreateInvoiceRequest, Invoice, UpdateInvoiceRequest};
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
        "INSERT INTO {} (contract_id, issue_date, due_date, amount, status) \
        VALUES (:contract_id, :issue_date, :due_date, :amount, :status) RETURNING id into :id",
        TABLE
    );

    let status: String = body.status.into();

    let stmt = con
        .execute_named(
            query.as_str(),
            &[
                ("contract_id", &body.contract_id),
                ("issue_date", &body.issue_date),
                ("due_date", &body.due_date),
                ("amount", &body.amount),
                ("status", &status),
                ("id", &OracleType::Number(0, 0)),
            ],
        )
        .map_err(Error::DBQuery)?;

    if let Err(e) = con.commit() {
        con.rollback().map_err(Error::DBQuery)?;
        return Err(Error::DBQuery(e));
    }

    let row_id: u32 = stmt.returned_values("id")?[0];
    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &row_id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_invoice(&row))
}

pub async fn update(db_pool: &DBPool, id: u32, body: UpdateInvoiceRequest) -> Result<Invoice> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET issue_date = :issue_date, due_date = :due_date, amount = :amount, status = :status WHERE id = :id", TABLE);

    let status: String = body.status.into();

    con.execute_named(
        query.as_str(),
        &[
            ("issue_date", &body.issue_date),
            ("due_date", &body.due_date),
            ("amount", &body.amount),
            ("status", &status),
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
