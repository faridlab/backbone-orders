use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "delivery_type", rename_all = "snake_case")]
pub enum DeliveryType {
    Scheduled,
    Express,
    SameDay,
    SelfPickup,
}

impl std::fmt::Display for DeliveryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scheduled => write!(f, "scheduled"),
            Self::Express => write!(f, "express"),
            Self::SameDay => write!(f, "same_day"),
            Self::SelfPickup => write!(f, "self_pickup"),
        }
    }
}

impl FromStr for DeliveryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "scheduled" => Ok(Self::Scheduled),
            "express" => Ok(Self::Express),
            "same_day" => Ok(Self::SameDay),
            "self_pickup" => Ok(Self::SelfPickup),
            _ => Err(format!("Unknown DeliveryType variant: {}", s)),
        }
    }
}

impl Default for DeliveryType {
    fn default() -> Self {
        Self::Scheduled
    }
}
