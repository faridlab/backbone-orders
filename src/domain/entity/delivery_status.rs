use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "delivery_status", rename_all = "snake_case")]
pub enum DeliveryStatus {
    Pending,
    Scheduled,
    Assigned,
    Preparing,
    Dispatched,
    EnRoute,
    Arrived,
    Delivering,
    Delivered,
    Failed,
    Returned,
    Rescheduled,
    Cancelled,
}

impl std::fmt::Display for DeliveryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Assigned => write!(f, "assigned"),
            Self::Preparing => write!(f, "preparing"),
            Self::Dispatched => write!(f, "dispatched"),
            Self::EnRoute => write!(f, "en_route"),
            Self::Arrived => write!(f, "arrived"),
            Self::Delivering => write!(f, "delivering"),
            Self::Delivered => write!(f, "delivered"),
            Self::Failed => write!(f, "failed"),
            Self::Returned => write!(f, "returned"),
            Self::Rescheduled => write!(f, "rescheduled"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl FromStr for DeliveryStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "scheduled" => Ok(Self::Scheduled),
            "assigned" => Ok(Self::Assigned),
            "preparing" => Ok(Self::Preparing),
            "dispatched" => Ok(Self::Dispatched),
            "en_route" => Ok(Self::EnRoute),
            "arrived" => Ok(Self::Arrived),
            "delivering" => Ok(Self::Delivering),
            "delivered" => Ok(Self::Delivered),
            "failed" => Ok(Self::Failed),
            "returned" => Ok(Self::Returned),
            "rescheduled" => Ok(Self::Rescheduled),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Unknown DeliveryStatus variant: {}", s)),
        }
    }
}

impl Default for DeliveryStatus {
    fn default() -> Self {
        Self::Pending
    }
}
