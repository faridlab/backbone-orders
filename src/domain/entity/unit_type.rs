use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "unit_type", rename_all = "snake_case")]
pub enum UnitType {
    PerKg,
    PerItem,
    PerPiece,
    FlatRate,
    Tiered,
    PerSet,
    PerPair,
    PerSqm,
    Other,
}

impl std::fmt::Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PerKg => write!(f, "per_kg"),
            Self::PerItem => write!(f, "per_item"),
            Self::PerPiece => write!(f, "per_piece"),
            Self::FlatRate => write!(f, "flat_rate"),
            Self::Tiered => write!(f, "tiered"),
            Self::PerSet => write!(f, "per_set"),
            Self::PerPair => write!(f, "per_pair"),
            Self::PerSqm => write!(f, "per_sqm"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl FromStr for UnitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "per_kg" => Ok(Self::PerKg),
            "per_item" => Ok(Self::PerItem),
            "per_piece" => Ok(Self::PerPiece),
            "flat_rate" => Ok(Self::FlatRate),
            "tiered" => Ok(Self::Tiered),
            "per_set" => Ok(Self::PerSet),
            "per_pair" => Ok(Self::PerPair),
            "per_sqm" => Ok(Self::PerSqm),
            "other" => Ok(Self::Other),
            _ => Err(format!("Unknown UnitType variant: {}", s)),
        }
    }
}

impl Default for UnitType {
    fn default() -> Self {
        Self::PerKg
    }
}
