use chrono::{DateTime, Utc};
use crate::db::{get_db_con, Result};
use crate::error::Error;
use crate::DBPool;
use common::payment::{Payment, PaymentRequest};
use oracle::Row;

const TABLE: &str = "payment";
const SELECT_FIELDS: &str = "id, invoice_id, payment_date, amount";

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<Payment>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);

    let rows = con.query(query.as_str(), &[]).map_err(Error::DBQuery)?;

    Ok(rows
        .filter(|r| r.is_ok())
        .map(|r| row_to_payment(&r.unwrap()))
        .collect())
}

pub async fn fetch_one(db_pool: &DBPool, id: u32) -> Result<Payment> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_payment(&row))
}

pub async fn create(db_pool: &DBPool, body: PaymentRequest) -> Result<Payment> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (invoice_id, payment_date, amount) VALUES (:invoice_id, :payment_date, :amount)", TABLE);

    con.execute_named(
        query.as_str(),
        &[
            ("invoice_id", &body.invoice_id),
            ("payment_date", &body.payment_date),
            ("amount", &body.amount),
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

    Ok(row_to_payment(&row))
}

pub async fn update(db_pool: &DBPool, id: u32, body: PaymentRequest) -> Result<Payment> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET invoice_id = :invoice_id, payment_date = :payment_date, amount = :amount WHERE id = :id", TABLE);

    con.execute_named(
        query.as_str(),
        &[
            ("invoice_id", &body.invoice_id),
            ("payment_date", &body.payment_date),
            ("amount", &body.amount),
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

    Ok(row_to_payment(&row))
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

fn row_to_payment(row: &Row) -> Payment {
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
