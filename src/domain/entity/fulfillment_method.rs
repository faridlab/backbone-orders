use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "fulfillment_method", rename_all = "snake_case")]
pub enum FulfillmentMethod {
    PickupDelivery,
    PickupOnly,
    DeliveryOnly,
    DropOff,
}

impl std::fmt::Display for FulfillmentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PickupDelivery => write!(f, "pickup_delivery"),
            Self::PickupOnly => write!(f, "pickup_only"),
            Self::DeliveryOnly => write!(f, "delivery_only"),
            Self::DropOff => write!(f, "drop_off"),
        }
    }
}

impl FromStr for FulfillmentMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pickup_delivery" => Ok(Self::PickupDelivery),
            "pickup_only" => Ok(Self::PickupOnly),
            "delivery_only" => Ok(Self::DeliveryOnly),
            "drop_off" => Ok(Self::DropOff),
            _ => Err(format!("Unknown FulfillmentMethod variant: {}", s)),
        }
    }
}

impl Default for FulfillmentMethod {
    fn default() -> Self {
        Self::PickupDelivery
    }
}
