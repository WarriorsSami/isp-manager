use serde::{Deserialize, Serialize};
use std::fmt;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum SubscriptionType {
    #[serde(rename = "MOBILE")]
    Mobile,
    #[serde(rename = "FIXED")]
    Fixed,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "MOBILE_INTERNET")]
    MobileInternet,
    #[serde(rename = "FIXED_INTERNET")]
    FixedInternet,
}

impl From<String> for SubscriptionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "MOBILE" => SubscriptionType::Mobile,
            "FIXED" => SubscriptionType::Fixed,
            "TV" => SubscriptionType::Tv,
            "MOBILE_INTERNET" => SubscriptionType::MobileInternet,
            "FIXED_INTERNET" => SubscriptionType::FixedInternet,
            _ => SubscriptionType::Mobile,
        }
    }
}

impl From<SubscriptionType> for String {
    fn from(s: SubscriptionType) -> Self {
        match s {
            SubscriptionType::Mobile => "MOBILE".to_string(),
            SubscriptionType::Fixed => "FIXED".to_string(),
            SubscriptionType::Tv => "TV".to_string(),
            SubscriptionType::MobileInternet => "MOBILE_INTERNET".to_string(),
            SubscriptionType::FixedInternet => "FIXED_INTERNET".to_string(),
        }
    }
}

impl fmt::Display for SubscriptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubscriptionType::Mobile => write!(f, "MOBILE"),
            SubscriptionType::Fixed => write!(f, "FIXED"),
            SubscriptionType::Tv => write!(f, "TV"),
            SubscriptionType::MobileInternet => write!(f, "MOBILE INTERNET"),
            SubscriptionType::FixedInternet => write!(f, "FIXED INTERNET"),
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Subscription {
    pub id: u32,
    pub description: String,
    #[serde(rename = "type")]
    pub subscription_type: SubscriptionType,
    pub traffic: i32,
    pub price: f64,
    pub extra_traffic_price: f64,
}

#[derive(Serialize, Deserialize, Validate, Clone, PartialEq, Debug)]
pub struct SubscriptionRequest {
    #[validate(length(min = 3, max = 100))]
    pub description: String,
    #[serde(rename = "type")]
    pub subscription_type: SubscriptionType,
    #[validate(range(min = 0))]
    pub traffic: i32,
    #[validate(range(min = 0.0))]
    pub price: f64,
    #[validate(range(min = 0.0))]
    pub extra_traffic_price: f64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SubscriptionResponse {
    pub id: u32,
    pub description: String,
    #[serde(rename = "type")]
    pub subscription_type: SubscriptionType,
    pub traffic: i32,
    pub price: f64,
    pub extra_traffic_price: f64,
}

impl From<Subscription> for SubscriptionResponse {
    fn from(subscription: Subscription) -> SubscriptionResponse {
        SubscriptionResponse {
            id: subscription.id,
            description: subscription.description,
            subscription_type: subscription.subscription_type,
            traffic: subscription.traffic,
            price: subscription.price,
            extra_traffic_price: subscription.extra_traffic_price,
        }
    }
}
