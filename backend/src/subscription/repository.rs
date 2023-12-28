use crate::db::subscription::{row_to_subscription, SELECT_FIELDS, TABLE};
use crate::db::{get_db_con, Result};
use crate::error::application::Error;
use crate::DBPool;
use common::subscription::{Subscription, SubscriptionRequest};
use oracle::sql_type::OracleType;

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
    let query = format!(
        "INSERT INTO {} (description, type, traffic, price, extra_traffic_price) \
        VALUES (:description, :type, :traffic, :price, :extra_traffic_price) RETURNING id INTO :id",
        TABLE
    );

    let subscription_type: String = body.subscription_type.into();

    let stmt = con
        .execute_named(
            query.as_str(),
            &[
                ("description", &body.description),
                ("type", &subscription_type),
                ("traffic", &body.traffic),
                ("price", &body.price),
                ("extra_traffic_price", &body.extra_traffic_price),
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
    .map_err(|e| match e {
        oracle::Error::NoDataFound => Error::SubscriptionNotFound(id),
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
