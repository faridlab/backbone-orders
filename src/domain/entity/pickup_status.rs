use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "pickup_status", rename_all = "snake_case")]
pub enum PickupStatus {
    Scheduled,
    Assigned,
    EnRoute,
    Arrived,
    InProgress,
    Completed,
    Failed,
    Rescheduled,
    Cancelled,
}

impl std::fmt::Display for PickupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scheduled => write!(f, "scheduled"),
            Self::Assigned => write!(f, "assigned"),
            Self::EnRoute => write!(f, "en_route"),
            Self::Arrived => write!(f, "arrived"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Rescheduled => write!(f, "rescheduled"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl FromStr for PickupStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "scheduled" => Ok(Self::Scheduled),
            "assigned" => Ok(Self::Assigned),
            "en_route" => Ok(Self::EnRoute),
            "arrived" => Ok(Self::Arrived),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "rescheduled" => Ok(Self::Rescheduled),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Unknown PickupStatus variant: {}", s)),
        }
    }
}

impl Default for PickupStatus {
    fn default() -> Self {
        Self::Scheduled
    }
}
