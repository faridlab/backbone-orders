use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "item_processing_status", rename_all = "snake_case")]
pub enum ItemProcessingStatus {
    Pending,
    Washing,
    Drying,
    Ironing,
    Folding,
    QualityCheck,
    Ready,
    Delivered,
    Problem,
}

impl std::fmt::Display for ItemProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Washing => write!(f, "washing"),
            Self::Drying => write!(f, "drying"),
            Self::Ironing => write!(f, "ironing"),
            Self::Folding => write!(f, "folding"),
            Self::QualityCheck => write!(f, "quality_check"),
            Self::Ready => write!(f, "ready"),
            Self::Delivered => write!(f, "delivered"),
            Self::Problem => write!(f, "problem"),
        }
    }
}

impl FromStr for ItemProcessingStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "washing" => Ok(Self::Washing),
            "drying" => Ok(Self::Drying),
            "ironing" => Ok(Self::Ironing),
            "folding" => Ok(Self::Folding),
            "quality_check" => Ok(Self::QualityCheck),
            "ready" => Ok(Self::Ready),
            "delivered" => Ok(Self::Delivered),
            "problem" => Ok(Self::Problem),
            _ => Err(format!("Unknown ItemProcessingStatus variant: {}", s)),
        }
    }
}

impl Default for ItemProcessingStatus {
    fn default() -> Self {
        Self::Pending
    }
}
