use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "cancellation_reason", rename_all = "snake_case")]
pub enum CancellationReason {
    CustomerRequest,
    ProviderUnavailable,
    NoPickupResponse,
    NoDeliveryResponse,
    PaymentFailed,
    ItemIssue,
    ForceMajeure,
    Other,
}

impl std::fmt::Display for CancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustomerRequest => write!(f, "customer_request"),
            Self::ProviderUnavailable => write!(f, "provider_unavailable"),
            Self::NoPickupResponse => write!(f, "no_pickup_response"),
            Self::NoDeliveryResponse => write!(f, "no_delivery_response"),
            Self::PaymentFailed => write!(f, "payment_failed"),
            Self::ItemIssue => write!(f, "item_issue"),
            Self::ForceMajeure => write!(f, "force_majeure"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl FromStr for CancellationReason {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "customer_request" => Ok(Self::CustomerRequest),
            "provider_unavailable" => Ok(Self::ProviderUnavailable),
            "no_pickup_response" => Ok(Self::NoPickupResponse),
            "no_delivery_response" => Ok(Self::NoDeliveryResponse),
            "payment_failed" => Ok(Self::PaymentFailed),
            "item_issue" => Ok(Self::ItemIssue),
            "force_majeure" => Ok(Self::ForceMajeure),
            "other" => Ok(Self::Other),
            _ => Err(format!("Unknown CancellationReason variant: {}", s)),
        }
    }
}

impl Default for CancellationReason {
    fn default() -> Self {
        Self::CustomerRequest
    }
}
