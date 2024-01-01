use crate::error::application::Error;
use crate::subscription::repository;
use crate::{DBPool, Result};
use common::subscription::{SubscriptionRequest, SubscriptionResponse};
use validator::Validate;
use warp::reply::json;
use warp::{reject, Buf, Reply};

pub async fn list_subscriptions_handler(db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Listing subscriptions");

    let subscriptions = repository::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &subscriptions
            .into_iter()
            .map(SubscriptionResponse::from)
            .collect(),
    ))
}

pub async fn fetch_subscription_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Fetching subscription with id {}", id);

    let subscription = repository::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&SubscriptionResponse::from(subscription)))
}

pub async fn create_subscription_handler(buf: impl Buf, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Creating a new subscription");

    let deserialized = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: SubscriptionRequest = serde_path_to_error::deserialize(deserialized)
        .map_err(|e| reject::custom(Error::JSONPath(e.to_string())))?;

    body.validate()
        .map_err(|e| reject::custom(Error::Validation(e)))?;

    let created_subscription = repository::create(&db_pool, body)
        .await
        .map_err(reject::custom)?;

    let response = json(&SubscriptionResponse::from(created_subscription));

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn update_subscription_handler(
    id: u32,
    buf: impl Buf,
    db_pool: DBPool,
) -> Result<impl Reply> {
    log::info!("Updating subscription with id {}", id);

    let deserialized = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: SubscriptionRequest = serde_path_to_error::deserialize(deserialized)
        .map_err(|e| reject::custom(Error::JSONPath(e.to_string())))?;

    body.validate()
        .map_err(|e| reject::custom(Error::Validation(e)))?;

    Ok(json(&SubscriptionResponse::from(
        repository::update(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_subscription_handler(id: u32, db_pool: DBPool) -> Result<impl Reply> {
    log::info!("Deleting subscription with id {}", id);

    repository::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
