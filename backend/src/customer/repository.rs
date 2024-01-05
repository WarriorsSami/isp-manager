use crate::db::contract::row_to_contract;
use crate::db::customer::{row_to_customer, SELECT_FIELDS, TABLE};
use crate::db::invoice::row_to_invoice;
use crate::db::{get_db_con, Result};
use crate::error::application::Error;
use crate::DBPool;
use common::contract::Contract;
use common::customer::{Customer, CustomerRequest};
use common::invoice::Invoice;
use oracle::sql_type::OracleType;

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<Customer>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);

    let rows = con.query(query.as_str(), &[]).map_err(Error::DBQuery)?;

    Ok(rows
        .filter(|r| r.is_ok())
        .map(|r| row_to_customer(&r.unwrap()))
        .collect())
}

pub async fn fetch_one(db_pool: &DBPool, id: u32) -> Result<Customer> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_customer(&row))
}

pub async fn create(db_pool: &DBPool, body: CustomerRequest) -> Result<Customer> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (name, fullname, address, phone, cnp) \
        VALUES (:name, :fullname, :address, :phone, :cnp) RETURNING id into :id",
        TABLE
    );

    let stmt = con
        .execute_named(
            query.as_str(),
            &[
                ("name", &body.name),
                ("fullname", &body.fullname),
                ("address", &body.address),
                ("phone", &body.phone),
                ("cnp", &body.cnp),
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

    Ok(row_to_customer(&row))
}

pub async fn update(db_pool: &DBPool, id: u32, body: CustomerRequest) -> Result<Customer> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET name = :name, fullname = :fullname, address = :address, phone = :phone, cnp = :cnp WHERE id = :id", TABLE);

    con.execute_named(
        query.as_str(),
        &[
            ("name", &body.name),
            ("fullname", &body.fullname),
            ("address", &body.address),
            ("phone", &body.phone),
            ("cnp", &body.cnp),
            ("id", &id),
        ],
    )
    .map_err(|e| match e {
        oracle::Error::NoDataFound => Error::CustomerNotFound(id),
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

    Ok(row_to_customer(&row))
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

pub async fn fetch_unpaid_invoices(db_pool: &DBPool, id: u32) -> Result<Vec<Invoice>> {
    use crate::db::invoice::SELECT_FIELDS;

    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM GET_UNPAID_INVOICES(:id)", SELECT_FIELDS);

    let rows = con
        .query_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(rows
        .filter(|r| r.is_ok())
        .map(|r| row_to_invoice(&r.unwrap()))
        .collect())
}

pub async fn fetch_unpaid_invoices_proc(db_pool: &DBPool, id: u32) -> Result<Vec<Invoice>> {
    let con = get_db_con(db_pool).await?;
    let query = r#"
        DECLARE
            invoices_cursor SYS_REFCURSOR;
        BEGIN
            GET_UNPAID_INVOICES_PROC(:id, invoices_cursor);
            DBMS_SQL.RETURN_RESULT(invoices_cursor);
        END;
    "#;

    let mut stmt = con.statement(query).build().map_err(Error::DBQuery)?;
    stmt.execute_named(&[("id", &id)]).map_err(Error::DBQuery)?;

    let opt_cursor = stmt.implicit_result().map_err(Error::DBQuery)?;
    let mut invoices = Vec::new();

    if let Some(mut cursor) = opt_cursor {
        let rows = cursor.query().map_err(Error::DBQuery)?;
        invoices = rows
            .filter(|r| r.is_ok())
            .map(|r| row_to_invoice(&r.unwrap()))
            .collect();
    }

    Ok(invoices)
}

pub async fn fetch_contracts(db_pool: &DBPool, id: u32) -> Result<Vec<Contract>> {
    use crate::db::contract::SELECT_FIELDS;

    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM GET_CONTRACTS(:id)", SELECT_FIELDS);

    let rows = con
        .query_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(rows
        .filter(|r| r.is_ok())
        .map(|r| row_to_contract(&r.unwrap()))
        .collect())
}
