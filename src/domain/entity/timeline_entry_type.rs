use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "timeline_entry_type", rename_all = "snake_case")]
pub enum TimelineEntryType {
    StatusChange,
    Note,
    Assignment,
    Location,
    Communication,
    Action,
    System,
}

impl std::fmt::Display for TimelineEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StatusChange => write!(f, "status_change"),
            Self::Note => write!(f, "note"),
            Self::Assignment => write!(f, "assignment"),
            Self::Location => write!(f, "location"),
            Self::Communication => write!(f, "communication"),
            Self::Action => write!(f, "action"),
            Self::System => write!(f, "system"),
        }
    }
}

impl FromStr for TimelineEntryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "status_change" => Ok(Self::StatusChange),
            "note" => Ok(Self::Note),
            "assignment" => Ok(Self::Assignment),
            "location" => Ok(Self::Location),
            "communication" => Ok(Self::Communication),
            "action" => Ok(Self::Action),
            "system" => Ok(Self::System),
            _ => Err(format!("Unknown TimelineEntryType variant: {}", s)),
        }
    }
}

impl Default for TimelineEntryType {
    fn default() -> Self {
        Self::StatusChange
    }
}
