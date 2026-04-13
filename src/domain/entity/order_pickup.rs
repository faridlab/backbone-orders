use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::PickupType;
use super::PickupStatus;
use super::AuditMetadata;

/// Strongly-typed ID for OrderPickup
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderPickupId(pub Uuid);

impl OrderPickupId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for OrderPickupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OrderPickupId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for OrderPickupId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<OrderPickupId> for Uuid {
    fn from(id: OrderPickupId) -> Self { id.0 }
}

impl AsRef<Uuid> for OrderPickupId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for OrderPickupId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderPickup {
    pub id: Uuid,
    pub order_id: Uuid,
    pub pickup_type: PickupType,
    pub scheduled_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_slot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_requested_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_requested_time_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_requested_time_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<Uuid>,
    pub address_line: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    pub contact_name: String,
    pub contact_phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone_alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_at: Option<DateTime<Utc>>,
    pub status: PickupStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_route_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrived_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_weight_kg: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_weight_kg: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_items: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_items: Option<i32>,
    pub bag_count: i32,
    pub bag_numbers: serde_json::Value,
    pub photos: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_from_address_meters: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    pub reschedule_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_scheduled_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reschedule_reason: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl OrderPickup {
    /// Create a builder for OrderPickup
    pub fn builder() -> OrderPickupBuilder {
        OrderPickupBuilder::default()
    }

    /// Create a new OrderPickup with required fields
    pub fn new(order_id: Uuid, pickup_type: PickupType, scheduled_date: NaiveDate, address_line: String, contact_name: String, contact_phone: String, status: PickupStatus, bag_count: i32, bag_numbers: serde_json::Value, photos: serde_json::Value, reschedule_count: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            pickup_type,
            scheduled_date,
            scheduled_time_start: None,
            scheduled_time_end: None,
            time_slot: None,
            customer_requested_date: None,
            customer_requested_time_start: None,
            customer_requested_time_end: None,
            address_id: None,
            address_line,
            address_detail: None,
            landmark: None,
            latitude: None,
            longitude: None,
            contact_name,
            contact_phone,
            contact_phone_alt: None,
            agent_id: None,
            agent_name: None,
            agent_phone: None,
            assigned_at: None,
            status,
            en_route_at: None,
            arrived_at: None,
            started_at: None,
            completed_at: None,
            estimated_weight_kg: None,
            actual_weight_kg: None,
            estimated_items: None,
            actual_items: None,
            bag_count,
            bag_numbers,
            photos,
            signature_url: None,
            signed_by: None,
            pickup_latitude: None,
            pickup_longitude: None,
            distance_from_address_meters: None,
            customer_notes: None,
            agent_notes: None,
            internal_notes: None,
            failure_reason: None,
            failure_code: None,
            reschedule_count,
            original_scheduled_date: None,
            reschedule_reason: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> OrderPickupId {
        OrderPickupId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }

    /// Get the current status
    pub fn status(&self) -> &PickupStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the scheduled_time_start field (chainable)
    pub fn with_scheduled_time_start(mut self, value: String) -> Self {
        self.scheduled_time_start = Some(value);
        self
    }

    /// Set the scheduled_time_end field (chainable)
    pub fn with_scheduled_time_end(mut self, value: String) -> Self {
        self.scheduled_time_end = Some(value);
        self
    }

    /// Set the time_slot field (chainable)
    pub fn with_time_slot(mut self, value: String) -> Self {
        self.time_slot = Some(value);
        self
    }

    /// Set the customer_requested_date field (chainable)
    pub fn with_customer_requested_date(mut self, value: NaiveDate) -> Self {
        self.customer_requested_date = Some(value);
        self
    }

    /// Set the customer_requested_time_start field (chainable)
    pub fn with_customer_requested_time_start(mut self, value: String) -> Self {
        self.customer_requested_time_start = Some(value);
        self
    }

    /// Set the customer_requested_time_end field (chainable)
    pub fn with_customer_requested_time_end(mut self, value: String) -> Self {
        self.customer_requested_time_end = Some(value);
        self
    }

    /// Set the address_id field (chainable)
    pub fn with_address_id(mut self, value: Uuid) -> Self {
        self.address_id = Some(value);
        self
    }

    /// Set the address_detail field (chainable)
    pub fn with_address_detail(mut self, value: String) -> Self {
        self.address_detail = Some(value);
        self
    }

    /// Set the landmark field (chainable)
    pub fn with_landmark(mut self, value: String) -> Self {
        self.landmark = Some(value);
        self
    }

    /// Set the latitude field (chainable)
    pub fn with_latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    /// Set the longitude field (chainable)
    pub fn with_longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Set the contact_phone_alt field (chainable)
    pub fn with_contact_phone_alt(mut self, value: String) -> Self {
        self.contact_phone_alt = Some(value);
        self
    }

    /// Set the agent_id field (chainable)
    pub fn with_agent_id(mut self, value: Uuid) -> Self {
        self.agent_id = Some(value);
        self
    }

    /// Set the agent_name field (chainable)
    pub fn with_agent_name(mut self, value: String) -> Self {
        self.agent_name = Some(value);
        self
    }

    /// Set the agent_phone field (chainable)
    pub fn with_agent_phone(mut self, value: String) -> Self {
        self.agent_phone = Some(value);
        self
    }

    /// Set the assigned_at field (chainable)
    pub fn with_assigned_at(mut self, value: DateTime<Utc>) -> Self {
        self.assigned_at = Some(value);
        self
    }

    /// Set the en_route_at field (chainable)
    pub fn with_en_route_at(mut self, value: DateTime<Utc>) -> Self {
        self.en_route_at = Some(value);
        self
    }

    /// Set the arrived_at field (chainable)
    pub fn with_arrived_at(mut self, value: DateTime<Utc>) -> Self {
        self.arrived_at = Some(value);
        self
    }

    /// Set the started_at field (chainable)
    pub fn with_started_at(mut self, value: DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Set the completed_at field (chainable)
    pub fn with_completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    /// Set the estimated_weight_kg field (chainable)
    pub fn with_estimated_weight_kg(mut self, value: Decimal) -> Self {
        self.estimated_weight_kg = Some(value);
        self
    }

    /// Set the actual_weight_kg field (chainable)
    pub fn with_actual_weight_kg(mut self, value: Decimal) -> Self {
        self.actual_weight_kg = Some(value);
        self
    }

    /// Set the estimated_items field (chainable)
    pub fn with_estimated_items(mut self, value: i32) -> Self {
        self.estimated_items = Some(value);
        self
    }

    /// Set the actual_items field (chainable)
    pub fn with_actual_items(mut self, value: i32) -> Self {
        self.actual_items = Some(value);
        self
    }

    /// Set the signature_url field (chainable)
    pub fn with_signature_url(mut self, value: String) -> Self {
        self.signature_url = Some(value);
        self
    }

    /// Set the signed_by field (chainable)
    pub fn with_signed_by(mut self, value: String) -> Self {
        self.signed_by = Some(value);
        self
    }

    /// Set the pickup_latitude field (chainable)
    pub fn with_pickup_latitude(mut self, value: f64) -> Self {
        self.pickup_latitude = Some(value);
        self
    }

    /// Set the pickup_longitude field (chainable)
    pub fn with_pickup_longitude(mut self, value: f64) -> Self {
        self.pickup_longitude = Some(value);
        self
    }

    /// Set the distance_from_address_meters field (chainable)
    pub fn with_distance_from_address_meters(mut self, value: i32) -> Self {
        self.distance_from_address_meters = Some(value);
        self
    }

    /// Set the customer_notes field (chainable)
    pub fn with_customer_notes(mut self, value: String) -> Self {
        self.customer_notes = Some(value);
        self
    }

    /// Set the agent_notes field (chainable)
    pub fn with_agent_notes(mut self, value: String) -> Self {
        self.agent_notes = Some(value);
        self
    }

    /// Set the internal_notes field (chainable)
    pub fn with_internal_notes(mut self, value: String) -> Self {
        self.internal_notes = Some(value);
        self
    }

    /// Set the failure_reason field (chainable)
    pub fn with_failure_reason(mut self, value: String) -> Self {
        self.failure_reason = Some(value);
        self
    }

    /// Set the failure_code field (chainable)
    pub fn with_failure_code(mut self, value: String) -> Self {
        self.failure_code = Some(value);
        self
    }

    /// Set the original_scheduled_date field (chainable)
    pub fn with_original_scheduled_date(mut self, value: NaiveDate) -> Self {
        self.original_scheduled_date = Some(value);
        self
    }

    /// Set the reschedule_reason field (chainable)
    pub fn with_reschedule_reason(mut self, value: String) -> Self {
        self.reschedule_reason = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "order_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.order_id = v; }
                }
                "pickup_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.pickup_type = v; }
                }
                "scheduled_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scheduled_date = v; }
                }
                "scheduled_time_start" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scheduled_time_start = v; }
                }
                "scheduled_time_end" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scheduled_time_end = v; }
                }
                "time_slot" => {
                    if let Ok(v) = serde_json::from_value(value) { self.time_slot = v; }
                }
                "customer_requested_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.customer_requested_date = v; }
                }
                "customer_requested_time_start" => {
                    if let Ok(v) = serde_json::from_value(value) { self.customer_requested_time_start = v; }
                }
                "customer_requested_time_end" => {
                    if let Ok(v) = serde_json::from_value(value) { self.customer_requested_time_end = v; }
                }
                "address_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.address_id = v; }
                }
                "address_line" => {
                    if let Ok(v) = serde_json::from_value(value) { self.address_line = v; }
                }
                "address_detail" => {
                    if let Ok(v) = serde_json::from_value(value) { self.address_detail = v; }
                }
                "landmark" => {
                    if let Ok(v) = serde_json::from_value(value) { self.landmark = v; }
                }
                "latitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.latitude = v; }
                }
                "longitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.longitude = v; }
                }
                "contact_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.contact_name = v; }
                }
                "contact_phone" => {
                    if let Ok(v) = serde_json::from_value(value) { self.contact_phone = v; }
                }
                "contact_phone_alt" => {
                    if let Ok(v) = serde_json::from_value(value) { self.contact_phone_alt = v; }
                }
                "agent_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.agent_id = v; }
                }
                "agent_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.agent_name = v; }
                }
                "agent_phone" => {
                    if let Ok(v) = serde_json::from_value(value) { self.agent_phone = v; }
                }
                "assigned_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.assigned_at = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "en_route_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.en_route_at = v; }
                }
                "arrived_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.arrived_at = v; }
                }
                "started_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.started_at = v; }
                }
                "completed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.completed_at = v; }
                }
                "estimated_weight_kg" => {
                    if let Ok(v) = serde_json::from_value(value) { self.estimated_weight_kg = v; }
                }
                "actual_weight_kg" => {
                    if let Ok(v) = serde_json::from_value(value) { self.actual_weight_kg = v; }
                }
                "estimated_items" => {
                    if let Ok(v) = serde_json::from_value(value) { self.estimated_items = v; }
                }
                "actual_items" => {
                    if let Ok(v) = serde_json::from_value(value) { self.actual_items = v; }
                }
                "bag_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bag_count = v; }
                }
                "bag_numbers" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bag_numbers = v; }
                }
                "photos" => {
                    if let Ok(v) = serde_json::from_value(value) { self.photos = v; }
                }
                "signature_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.signature_url = v; }
                }
                "signed_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.signed_by = v; }
                }
                "pickup_latitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.pickup_latitude = v; }
                }
                "pickup_longitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.pickup_longitude = v; }
                }
                "distance_from_address_meters" => {
                    if let Ok(v) = serde_json::from_value(value) { self.distance_from_address_meters = v; }
                }
                "customer_notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.customer_notes = v; }
                }
                "agent_notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.agent_notes = v; }
                }
                "internal_notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.internal_notes = v; }
                }
                "failure_reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.failure_reason = v; }
                }
                "failure_code" => {
                    if let Ok(v) = serde_json::from_value(value) { self.failure_code = v; }
                }
                "reschedule_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reschedule_count = v; }
                }
                "original_scheduled_date" => {
                    if let Ok(v) = serde_json::from_value(value) { self.original_scheduled_date = v; }
                }
                "reschedule_reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reschedule_reason = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for OrderPickup {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "OrderPickup"
    }
}

impl backbone_core::PersistentEntity for OrderPickup {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for OrderPickup {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("order_id".to_string(), "uuid".to_string());
        m.insert("address_id".to_string(), "uuid".to_string());
        m.insert("agent_id".to_string(), "uuid".to_string());
        m.insert("pickup_type".to_string(), "pickup_type".to_string());
        m.insert("status".to_string(), "pickup_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["address_line", "contact_name", "contact_phone"]
    }
}

/// Builder for OrderPickup entity
///
/// Provides a fluent API for constructing OrderPickup instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct OrderPickupBuilder {
    order_id: Option<Uuid>,
    pickup_type: Option<PickupType>,
    scheduled_date: Option<NaiveDate>,
    scheduled_time_start: Option<String>,
    scheduled_time_end: Option<String>,
    time_slot: Option<String>,
    customer_requested_date: Option<NaiveDate>,
    customer_requested_time_start: Option<String>,
    customer_requested_time_end: Option<String>,
    address_id: Option<Uuid>,
    address_line: Option<String>,
    address_detail: Option<String>,
    landmark: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    contact_name: Option<String>,
    contact_phone: Option<String>,
    contact_phone_alt: Option<String>,
    agent_id: Option<Uuid>,
    agent_name: Option<String>,
    agent_phone: Option<String>,
    assigned_at: Option<DateTime<Utc>>,
    status: Option<PickupStatus>,
    en_route_at: Option<DateTime<Utc>>,
    arrived_at: Option<DateTime<Utc>>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    estimated_weight_kg: Option<Decimal>,
    actual_weight_kg: Option<Decimal>,
    estimated_items: Option<i32>,
    actual_items: Option<i32>,
    bag_count: Option<i32>,
    bag_numbers: Option<serde_json::Value>,
    photos: Option<serde_json::Value>,
    signature_url: Option<String>,
    signed_by: Option<String>,
    pickup_latitude: Option<f64>,
    pickup_longitude: Option<f64>,
    distance_from_address_meters: Option<i32>,
    customer_notes: Option<String>,
    agent_notes: Option<String>,
    internal_notes: Option<String>,
    failure_reason: Option<String>,
    failure_code: Option<String>,
    reschedule_count: Option<i32>,
    original_scheduled_date: Option<NaiveDate>,
    reschedule_reason: Option<String>,
}

impl OrderPickupBuilder {
    /// Set the order_id field (required)
    pub fn order_id(mut self, value: Uuid) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Set the pickup_type field (default: `PickupType::default()`)
    pub fn pickup_type(mut self, value: PickupType) -> Self {
        self.pickup_type = Some(value);
        self
    }

    /// Set the scheduled_date field (required)
    pub fn scheduled_date(mut self, value: NaiveDate) -> Self {
        self.scheduled_date = Some(value);
        self
    }

    /// Set the scheduled_time_start field (optional)
    pub fn scheduled_time_start(mut self, value: String) -> Self {
        self.scheduled_time_start = Some(value);
        self
    }

    /// Set the scheduled_time_end field (optional)
    pub fn scheduled_time_end(mut self, value: String) -> Self {
        self.scheduled_time_end = Some(value);
        self
    }

    /// Set the time_slot field (optional)
    pub fn time_slot(mut self, value: String) -> Self {
        self.time_slot = Some(value);
        self
    }

    /// Set the customer_requested_date field (optional)
    pub fn customer_requested_date(mut self, value: NaiveDate) -> Self {
        self.customer_requested_date = Some(value);
        self
    }

    /// Set the customer_requested_time_start field (optional)
    pub fn customer_requested_time_start(mut self, value: String) -> Self {
        self.customer_requested_time_start = Some(value);
        self
    }

    /// Set the customer_requested_time_end field (optional)
    pub fn customer_requested_time_end(mut self, value: String) -> Self {
        self.customer_requested_time_end = Some(value);
        self
    }

    /// Set the address_id field (optional)
    pub fn address_id(mut self, value: Uuid) -> Self {
        self.address_id = Some(value);
        self
    }

    /// Set the address_line field (required)
    pub fn address_line(mut self, value: String) -> Self {
        self.address_line = Some(value);
        self
    }

    /// Set the address_detail field (optional)
    pub fn address_detail(mut self, value: String) -> Self {
        self.address_detail = Some(value);
        self
    }

    /// Set the landmark field (optional)
    pub fn landmark(mut self, value: String) -> Self {
        self.landmark = Some(value);
        self
    }

    /// Set the latitude field (optional)
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    /// Set the longitude field (optional)
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Set the contact_name field (required)
    pub fn contact_name(mut self, value: String) -> Self {
        self.contact_name = Some(value);
        self
    }

    /// Set the contact_phone field (required)
    pub fn contact_phone(mut self, value: String) -> Self {
        self.contact_phone = Some(value);
        self
    }

    /// Set the contact_phone_alt field (optional)
    pub fn contact_phone_alt(mut self, value: String) -> Self {
        self.contact_phone_alt = Some(value);
        self
    }

    /// Set the agent_id field (optional)
    pub fn agent_id(mut self, value: Uuid) -> Self {
        self.agent_id = Some(value);
        self
    }

    /// Set the agent_name field (optional)
    pub fn agent_name(mut self, value: String) -> Self {
        self.agent_name = Some(value);
        self
    }

    /// Set the agent_phone field (optional)
    pub fn agent_phone(mut self, value: String) -> Self {
        self.agent_phone = Some(value);
        self
    }

    /// Set the assigned_at field (optional)
    pub fn assigned_at(mut self, value: DateTime<Utc>) -> Self {
        self.assigned_at = Some(value);
        self
    }

    /// Set the status field (default: `PickupStatus::default()`)
    pub fn status(mut self, value: PickupStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the en_route_at field (optional)
    pub fn en_route_at(mut self, value: DateTime<Utc>) -> Self {
        self.en_route_at = Some(value);
        self
    }

    /// Set the arrived_at field (optional)
    pub fn arrived_at(mut self, value: DateTime<Utc>) -> Self {
        self.arrived_at = Some(value);
        self
    }

    /// Set the started_at field (optional)
    pub fn started_at(mut self, value: DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Set the completed_at field (optional)
    pub fn completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    /// Set the estimated_weight_kg field (optional)
    pub fn estimated_weight_kg(mut self, value: Decimal) -> Self {
        self.estimated_weight_kg = Some(value);
        self
    }

    /// Set the actual_weight_kg field (optional)
    pub fn actual_weight_kg(mut self, value: Decimal) -> Self {
        self.actual_weight_kg = Some(value);
        self
    }

    /// Set the estimated_items field (optional)
    pub fn estimated_items(mut self, value: i32) -> Self {
        self.estimated_items = Some(value);
        self
    }

    /// Set the actual_items field (optional)
    pub fn actual_items(mut self, value: i32) -> Self {
        self.actual_items = Some(value);
        self
    }

    /// Set the bag_count field (default: `0`)
    pub fn bag_count(mut self, value: i32) -> Self {
        self.bag_count = Some(value);
        self
    }

    /// Set the bag_numbers field (default: `serde_json::json!([])`)
    pub fn bag_numbers(mut self, value: serde_json::Value) -> Self {
        self.bag_numbers = Some(value);
        self
    }

    /// Set the photos field (default: `serde_json::json!([])`)
    pub fn photos(mut self, value: serde_json::Value) -> Self {
        self.photos = Some(value);
        self
    }

    /// Set the signature_url field (optional)
    pub fn signature_url(mut self, value: String) -> Self {
        self.signature_url = Some(value);
        self
    }

    /// Set the signed_by field (optional)
    pub fn signed_by(mut self, value: String) -> Self {
        self.signed_by = Some(value);
        self
    }

    /// Set the pickup_latitude field (optional)
    pub fn pickup_latitude(mut self, value: f64) -> Self {
        self.pickup_latitude = Some(value);
        self
    }

    /// Set the pickup_longitude field (optional)
    pub fn pickup_longitude(mut self, value: f64) -> Self {
        self.pickup_longitude = Some(value);
        self
    }

    /// Set the distance_from_address_meters field (optional)
    pub fn distance_from_address_meters(mut self, value: i32) -> Self {
        self.distance_from_address_meters = Some(value);
        self
    }

    /// Set the customer_notes field (optional)
    pub fn customer_notes(mut self, value: String) -> Self {
        self.customer_notes = Some(value);
        self
    }

    /// Set the agent_notes field (optional)
    pub fn agent_notes(mut self, value: String) -> Self {
        self.agent_notes = Some(value);
        self
    }

    /// Set the internal_notes field (optional)
    pub fn internal_notes(mut self, value: String) -> Self {
        self.internal_notes = Some(value);
        self
    }

    /// Set the failure_reason field (optional)
    pub fn failure_reason(mut self, value: String) -> Self {
        self.failure_reason = Some(value);
        self
    }

    /// Set the failure_code field (optional)
    pub fn failure_code(mut self, value: String) -> Self {
        self.failure_code = Some(value);
        self
    }

    /// Set the reschedule_count field (default: `0`)
    pub fn reschedule_count(mut self, value: i32) -> Self {
        self.reschedule_count = Some(value);
        self
    }

    /// Set the original_scheduled_date field (optional)
    pub fn original_scheduled_date(mut self, value: NaiveDate) -> Self {
        self.original_scheduled_date = Some(value);
        self
    }

    /// Set the reschedule_reason field (optional)
    pub fn reschedule_reason(mut self, value: String) -> Self {
        self.reschedule_reason = Some(value);
        self
    }

    /// Build the OrderPickup entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<OrderPickup, String> {
        let order_id = self.order_id.ok_or_else(|| "order_id is required".to_string())?;
        let scheduled_date = self.scheduled_date.ok_or_else(|| "scheduled_date is required".to_string())?;
        let address_line = self.address_line.ok_or_else(|| "address_line is required".to_string())?;
        let contact_name = self.contact_name.ok_or_else(|| "contact_name is required".to_string())?;
        let contact_phone = self.contact_phone.ok_or_else(|| "contact_phone is required".to_string())?;

        Ok(OrderPickup {
            id: Uuid::new_v4(),
            order_id,
            pickup_type: self.pickup_type.unwrap_or(PickupType::default()),
            scheduled_date,
            scheduled_time_start: self.scheduled_time_start,
            scheduled_time_end: self.scheduled_time_end,
            time_slot: self.time_slot,
            customer_requested_date: self.customer_requested_date,
            customer_requested_time_start: self.customer_requested_time_start,
            customer_requested_time_end: self.customer_requested_time_end,
            address_id: self.address_id,
            address_line,
            address_detail: self.address_detail,
            landmark: self.landmark,
            latitude: self.latitude,
            longitude: self.longitude,
            contact_name,
            contact_phone,
            contact_phone_alt: self.contact_phone_alt,
            agent_id: self.agent_id,
            agent_name: self.agent_name,
            agent_phone: self.agent_phone,
            assigned_at: self.assigned_at,
            status: self.status.unwrap_or(PickupStatus::default()),
            en_route_at: self.en_route_at,
            arrived_at: self.arrived_at,
            started_at: self.started_at,
            completed_at: self.completed_at,
            estimated_weight_kg: self.estimated_weight_kg,
            actual_weight_kg: self.actual_weight_kg,
            estimated_items: self.estimated_items,
            actual_items: self.actual_items,
            bag_count: self.bag_count.unwrap_or(0),
            bag_numbers: self.bag_numbers.unwrap_or(serde_json::json!([])),
            photos: self.photos.unwrap_or(serde_json::json!([])),
            signature_url: self.signature_url,
            signed_by: self.signed_by,
            pickup_latitude: self.pickup_latitude,
            pickup_longitude: self.pickup_longitude,
            distance_from_address_meters: self.distance_from_address_meters,
            customer_notes: self.customer_notes,
            agent_notes: self.agent_notes,
            internal_notes: self.internal_notes,
            failure_reason: self.failure_reason,
            failure_code: self.failure_code,
            reschedule_count: self.reschedule_count.unwrap_or(0),
            original_scheduled_date: self.original_scheduled_date,
            reschedule_reason: self.reschedule_reason,
            metadata: AuditMetadata::default(),
        })
    }
}
