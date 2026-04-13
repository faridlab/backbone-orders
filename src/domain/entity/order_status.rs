use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    PickupScheduled,
    PickupInProgress,
    PickedUp,
    Received,
    QueueWashing,
    Washing,
    Drying,
    QueueIroning,
    Ironing,
    QualityCheck,
    Packing,
    Ready,
    DeliveryScheduled,
    DeliveryInProgress,
    Delivered,
    Completed,
    Cancelled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::PickupScheduled => write!(f, "pickup_scheduled"),
            Self::PickupInProgress => write!(f, "pickup_in_progress"),
            Self::PickedUp => write!(f, "picked_up"),
            Self::Received => write!(f, "received"),
            Self::QueueWashing => write!(f, "queue_washing"),
            Self::Washing => write!(f, "washing"),
            Self::Drying => write!(f, "drying"),
            Self::QueueIroning => write!(f, "queue_ironing"),
            Self::Ironing => write!(f, "ironing"),
            Self::QualityCheck => write!(f, "quality_check"),
            Self::Packing => write!(f, "packing"),
            Self::Ready => write!(f, "ready"),
            Self::DeliveryScheduled => write!(f, "delivery_scheduled"),
            Self::DeliveryInProgress => write!(f, "delivery_in_progress"),
            Self::Delivered => write!(f, "delivered"),
            Self::Completed => write!(f, "completed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "confirmed" => Ok(Self::Confirmed),
            "pickup_scheduled" => Ok(Self::PickupScheduled),
            "pickup_in_progress" => Ok(Self::PickupInProgress),
            "picked_up" => Ok(Self::PickedUp),
            "received" => Ok(Self::Received),
            "queue_washing" => Ok(Self::QueueWashing),
            "washing" => Ok(Self::Washing),
            "drying" => Ok(Self::Drying),
            "queue_ironing" => Ok(Self::QueueIroning),
            "ironing" => Ok(Self::Ironing),
            "quality_check" => Ok(Self::QualityCheck),
            "packing" => Ok(Self::Packing),
            "ready" => Ok(Self::Ready),
            "delivery_scheduled" => Ok(Self::DeliveryScheduled),
            "delivery_in_progress" => Ok(Self::DeliveryInProgress),
            "delivered" => Ok(Self::Delivered),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Unknown OrderStatus variant: {}", s)),
        }
    }
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Pending
    }
}
