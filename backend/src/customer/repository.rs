use crate::db::{get_db_con, Result};
use crate::error::Error;
use crate::DBPool;
use common::customer::{Customer, CustomerRequest};
use oracle::Row;

const TABLE: &str = "customer";
const SELECT_FIELDS: &str = "id, name, fullname, address, phone, cnp";

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
    let query = format!("INSERT INTO {} (name, fullname, address, phone, cnp) VALUES (:name, :fullname, :address, :phone, :cnp)", TABLE);

    con.execute_named(
        query.as_str(),
        &[
            ("name", &body.name),
            ("fullname", &body.fullname),
            ("address", &body.address),
            ("phone", &body.phone),
            ("cnp", &body.cnp),
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

    Ok(row_to_customer(&row))
}

pub async fn update(db_pool: &DBPool, id: u32, body: CustomerRequest) -> Result<Customer> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET name = :name, fullname = :fullname, address = :address, phone = :phone, cnp = :cnp WHERE id = :id", TABLE);

    con.execute_named(
        query.as_str(),
        &[
            ("id", &id),
            ("name", &body.name),
            ("fullname", &body.fullname),
            ("address", &body.address),
            ("phone", &body.phone),
            ("cnp", &body.cnp),
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

fn row_to_customer(row: &Row) -> Customer {
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
