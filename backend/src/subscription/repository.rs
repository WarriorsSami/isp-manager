use crate::db::{get_db_con, Result};
use crate::error::Error;
use crate::DBPool;
use common::subscription::{Subscription, SubscriptionRequest};
use oracle::Row;

const TABLE: &str = "subscription";
const SELECT_FIELDS: &str = "id, description, type, traffic, price, extra_traffic_price";

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<Subscription>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);

    let rows = con.query(query.as_str(), &[]).map_err(Error::DBQuery)?;

    Ok(rows
        .filter(|r| r.is_ok())
        .map(|r| row_to_subscription(&r.unwrap()))
        .collect())
}

pub async fn fetch_one(db_pool: &DBPool, id: u32) -> Result<Subscription> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = :id", SELECT_FIELDS, TABLE);

    let row = con
        .query_row_named(query.as_str(), &[("id", &id)])
        .map_err(Error::DBQuery)?;

    Ok(row_to_subscription(&row))
}

pub async fn create(db_pool: &DBPool, body: SubscriptionRequest) -> Result<Subscription> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (description, type, traffic, price, extra_traffic_price) VALUES (:description, :type, :traffic, :price, :extra_traffic_price)", TABLE);

    let subscription_type: String = body.subscription_type.into();

    con.execute_named(
        query.as_str(),
        &[
            ("description", &body.description),
            ("type", &subscription_type),
            ("traffic", &body.traffic),
            ("price", &body.price),
            ("extra_traffic_price", &body.extra_traffic_price),
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

    Ok(row_to_subscription(&row))
}

pub async fn update(db_pool: &DBPool, id: u32, body: SubscriptionRequest) -> Result<Subscription> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET description = :description, type = :type, traffic = :traffic, price = :price, extra_traffic_price = :extra_traffic_price WHERE id = :id", TABLE);

    let subscription_type: String = body.subscription_type.into();

    con.execute_named(
        query.as_str(),
        &[
            ("id", &id),
            ("description", &body.description),
            ("type", &subscription_type),
            ("traffic", &body.traffic),
            ("price", &body.price),
            ("extra_traffic_price", &body.extra_traffic_price),
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

    Ok(row_to_subscription(&row))
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

fn row_to_subscription(row: &Row) -> Subscription {
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
