use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    AwaitingPayment,
    Paid,
    PartiallyPaid,
    Refunded,
    Failed,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::AwaitingPayment => write!(f, "awaiting_payment"),
            Self::Paid => write!(f, "paid"),
            Self::PartiallyPaid => write!(f, "partially_paid"),
            Self::Refunded => write!(f, "refunded"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

impl FromStr for PaymentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "awaiting_payment" => Ok(Self::AwaitingPayment),
            "paid" => Ok(Self::Paid),
            "partially_paid" => Ok(Self::PartiallyPaid),
            "refunded" => Ok(Self::Refunded),
            "failed" => Ok(Self::Failed),
            _ => Err(format!("Unknown PaymentStatus variant: {}", s)),
        }
    }
}

impl Default for PaymentStatus {
    fn default() -> Self {
        Self::Pending
    }
}
