use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "order_type", rename_all = "snake_case")]
pub enum OrderType {
    Kiloan,
    Satuan,
    DryClean,
    Express,
    SameDay,
    Specialty,
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kiloan => write!(f, "kiloan"),
            Self::Satuan => write!(f, "satuan"),
            Self::DryClean => write!(f, "dry_clean"),
            Self::Express => write!(f, "express"),
            Self::SameDay => write!(f, "same_day"),
            Self::Specialty => write!(f, "specialty"),
        }
    }
}

impl FromStr for OrderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kiloan" => Ok(Self::Kiloan),
            "satuan" => Ok(Self::Satuan),
            "dry_clean" => Ok(Self::DryClean),
            "express" => Ok(Self::Express),
            "same_day" => Ok(Self::SameDay),
            "specialty" => Ok(Self::Specialty),
            _ => Err(format!("Unknown OrderType variant: {}", s)),
        }
    }
}

impl Default for OrderType {
    fn default() -> Self {
        Self::Kiloan
    }
}
