use crate::error::Error;
use crate::subscription::repository;
use crate::{DBPool, Result};
use common::subscription::{SubscriptionRequest, SubscriptionResponse};
use warp::reply::json;
use warp::{reject, Reply};

pub async fn list_subscriptions_handler(db_pool: DBPool) -> Result<impl Reply> {
    println!("Listing subscriptions");

    let subscriptions = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &subscriptions
            .into_iter()
            .map(SubscriptionResponse::from)
            .collect(),
    ))
}

pub async fn fetch_subscription_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Fetching subscription with id {}", id);

    let subscription = repository::fetch_one(&db_pool, id)
        .await
        .map_err(|_| reject::custom(Error::SubscriptionNotFound(id)))?;
    Ok(json(&SubscriptionResponse::from(subscription)))
}

pub async fn create_subscription_handler(
    body: SubscriptionRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    println!("Creating a new subscription");

    Ok(json(&SubscriptionResponse::from(
        repository::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_subscription_handler(
    id: u32,
    body: SubscriptionRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    println!("Updating subscription with id {}", id);

    Ok(json(&SubscriptionResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(|_| reject::custom(Error::SubscriptionNotFound(id)))?,
    )))
}

pub async fn delete_subscription_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    println!("Deleting subscription with id {}", id);

    repository::delete(&db_pool, id)
        .await
        .map_err(|_| reject::custom(Error::SubscriptionNotFound(id)))?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
