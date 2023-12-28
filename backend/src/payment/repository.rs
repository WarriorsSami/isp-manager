use crate::db::payment::{row_to_payment, SELECT_FIELDS, TABLE};
use crate::db::{get_db_con, Result};
use crate::error::Error;
use crate::DBPool;
use common::payment::{CreatePaymentRequest, Payment};
use oracle::sql_type::OracleType;

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

pub async fn create(db_pool: &DBPool, body: CreatePaymentRequest) -> Result<Payment> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (invoice_id, payment_date, amount) \
        VALUES (:invoice_id, :payment_date, :amount) RETURNING id INTO :id",
        TABLE
    );

    let stmt = con
        .execute_named(
            query.as_str(),
            &[
                ("invoice_id", &body.invoice_id),
                ("payment_date", &body.payment_date),
                ("amount", &body.amount),
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

    Ok(row_to_payment(&row))
}
