use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "item_condition", rename_all = "snake_case")]
pub enum ItemCondition {
    Good,
    Stained,
    Damaged,
    Worn,
}

impl std::fmt::Display for ItemCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Good => write!(f, "good"),
            Self::Stained => write!(f, "stained"),
            Self::Damaged => write!(f, "damaged"),
            Self::Worn => write!(f, "worn"),
        }
    }
}

impl FromStr for ItemCondition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "good" => Ok(Self::Good),
            "stained" => Ok(Self::Stained),
            "damaged" => Ok(Self::Damaged),
            "worn" => Ok(Self::Worn),
            _ => Err(format!("Unknown ItemCondition variant: {}", s)),
        }
    }
}

impl Default for ItemCondition {
    fn default() -> Self {
        Self::Good
    }
}
