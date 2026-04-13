use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "pickup_type", rename_all = "snake_case")]
pub enum PickupType {
    Scheduled,
    OnDemand,
    DropOff,
}

impl std::fmt::Display for PickupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scheduled => write!(f, "scheduled"),
            Self::OnDemand => write!(f, "on_demand"),
            Self::DropOff => write!(f, "drop_off"),
        }
    }
}

impl FromStr for PickupType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "scheduled" => Ok(Self::Scheduled),
            "on_demand" => Ok(Self::OnDemand),
            "drop_off" => Ok(Self::DropOff),
            _ => Err(format!("Unknown PickupType variant: {}", s)),
        }
    }
}

impl Default for PickupType {
    fn default() -> Self {
        Self::Scheduled
    }
}
