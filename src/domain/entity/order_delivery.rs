use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::DeliveryType;
use super::DeliveryStatus;
use super::AuditMetadata;

/// Strongly-typed ID for OrderDelivery
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderDeliveryId(pub Uuid);

impl OrderDeliveryId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for OrderDeliveryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OrderDeliveryId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for OrderDeliveryId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<OrderDeliveryId> for Uuid {
    fn from(id: OrderDeliveryId) -> Self { id.0 }
}

impl AsRef<Uuid> for OrderDeliveryId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for OrderDeliveryId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderDelivery {
    pub id: Uuid,
    pub order_id: Uuid,
    pub delivery_type: DeliveryType,
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
    pub status: DeliveryStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatched_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_route_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrived_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<DateTime<Utc>>,
    pub item_count: i32,
    pub bag_count: i32,
    pub bag_numbers: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    pub photos: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_to_customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_from_address_meters: Option<i32>,
    pub delivery_fee: Decimal,
    pub express_fee: Decimal,
    pub total_fee: Decimal,
    pub currency: String,
    pub is_cod: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cod_amount: Option<Decimal>,
    pub cod_collected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cod_collected_at: Option<DateTime<Utc>>,
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
    pub attempt_count: i32,
    pub reschedule_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_scheduled_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reschedule_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_rating: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_comment: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl OrderDelivery {
    /// Create a builder for OrderDelivery
    pub fn builder() -> OrderDeliveryBuilder {
        OrderDeliveryBuilder::default()
    }

    /// Create a new OrderDelivery with required fields
    pub fn new(order_id: Uuid, delivery_type: DeliveryType, scheduled_date: NaiveDate, address_line: String, contact_name: String, contact_phone: String, status: DeliveryStatus, item_count: i32, bag_count: i32, bag_numbers: serde_json::Value, photos: serde_json::Value, delivery_fee: Decimal, express_fee: Decimal, total_fee: Decimal, currency: String, is_cod: bool, cod_collected: bool, attempt_count: i32, reschedule_count: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            delivery_type,
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
            dispatched_at: None,
            en_route_at: None,
            arrived_at: None,
            delivered_at: None,
            item_count,
            bag_count,
            bag_numbers,
            verification_method: None,
            verification_code: None,
            verified_at: None,
            photos,
            proof_photo_url: None,
            signature_url: None,
            signed_by: None,
            relation_to_customer: None,
            delivery_latitude: None,
            delivery_longitude: None,
            distance_from_address_meters: None,
            delivery_fee,
            express_fee,
            total_fee,
            currency,
            is_cod,
            cod_amount: None,
            cod_collected,
            cod_collected_at: None,
            customer_notes: None,
            agent_notes: None,
            internal_notes: None,
            failure_reason: None,
            failure_code: None,
            attempt_count,
            reschedule_count,
            original_scheduled_date: None,
            reschedule_reason: None,
            delivery_rating: None,
            rating_comment: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> OrderDeliveryId {
        OrderDeliveryId(self.id)
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
    pub fn status(&self) -> &DeliveryStatus {
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

    /// Set the dispatched_at field (chainable)
    pub fn with_dispatched_at(mut self, value: DateTime<Utc>) -> Self {
        self.dispatched_at = Some(value);
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

    /// Set the delivered_at field (chainable)
    pub fn with_delivered_at(mut self, value: DateTime<Utc>) -> Self {
        self.delivered_at = Some(value);
        self
    }

    /// Set the verification_method field (chainable)
    pub fn with_verification_method(mut self, value: String) -> Self {
        self.verification_method = Some(value);
        self
    }

    /// Set the verification_code field (chainable)
    pub fn with_verification_code(mut self, value: String) -> Self {
        self.verification_code = Some(value);
        self
    }

    /// Set the verified_at field (chainable)
    pub fn with_verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the proof_photo_url field (chainable)
    pub fn with_proof_photo_url(mut self, value: String) -> Self {
        self.proof_photo_url = Some(value);
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

    /// Set the relation_to_customer field (chainable)
    pub fn with_relation_to_customer(mut self, value: String) -> Self {
        self.relation_to_customer = Some(value);
        self
    }

    /// Set the delivery_latitude field (chainable)
    pub fn with_delivery_latitude(mut self, value: f64) -> Self {
        self.delivery_latitude = Some(value);
        self
    }

    /// Set the delivery_longitude field (chainable)
    pub fn with_delivery_longitude(mut self, value: f64) -> Self {
        self.delivery_longitude = Some(value);
        self
    }

    /// Set the distance_from_address_meters field (chainable)
    pub fn with_distance_from_address_meters(mut self, value: i32) -> Self {
        self.distance_from_address_meters = Some(value);
        self
    }

    /// Set the cod_amount field (chainable)
    pub fn with_cod_amount(mut self, value: Decimal) -> Self {
        self.cod_amount = Some(value);
        self
    }

    /// Set the cod_collected_at field (chainable)
    pub fn with_cod_collected_at(mut self, value: DateTime<Utc>) -> Self {
        self.cod_collected_at = Some(value);
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

    /// Set the delivery_rating field (chainable)
    pub fn with_delivery_rating(mut self, value: i32) -> Self {
        self.delivery_rating = Some(value);
        self
    }

    /// Set the rating_comment field (chainable)
    pub fn with_rating_comment(mut self, value: String) -> Self {
        self.rating_comment = Some(value);
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
                "delivery_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivery_type = v; }
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
                "dispatched_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.dispatched_at = v; }
                }
                "en_route_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.en_route_at = v; }
                }
                "arrived_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.arrived_at = v; }
                }
                "delivered_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivered_at = v; }
                }
                "item_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.item_count = v; }
                }
                "bag_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bag_count = v; }
                }
                "bag_numbers" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bag_numbers = v; }
                }
                "verification_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_method = v; }
                }
                "verification_code" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_code = v; }
                }
                "verified_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verified_at = v; }
                }
                "photos" => {
                    if let Ok(v) = serde_json::from_value(value) { self.photos = v; }
                }
                "proof_photo_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.proof_photo_url = v; }
                }
                "signature_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.signature_url = v; }
                }
                "signed_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.signed_by = v; }
                }
                "relation_to_customer" => {
                    if let Ok(v) = serde_json::from_value(value) { self.relation_to_customer = v; }
                }
                "delivery_latitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivery_latitude = v; }
                }
                "delivery_longitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivery_longitude = v; }
                }
                "distance_from_address_meters" => {
                    if let Ok(v) = serde_json::from_value(value) { self.distance_from_address_meters = v; }
                }
                "delivery_fee" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivery_fee = v; }
                }
                "express_fee" => {
                    if let Ok(v) = serde_json::from_value(value) { self.express_fee = v; }
                }
                "total_fee" => {
                    if let Ok(v) = serde_json::from_value(value) { self.total_fee = v; }
                }
                "currency" => {
                    if let Ok(v) = serde_json::from_value(value) { self.currency = v; }
                }
                "is_cod" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_cod = v; }
                }
                "cod_amount" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cod_amount = v; }
                }
                "cod_collected" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cod_collected = v; }
                }
                "cod_collected_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cod_collected_at = v; }
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
                "attempt_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.attempt_count = v; }
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
                "delivery_rating" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivery_rating = v; }
                }
                "rating_comment" => {
                    if let Ok(v) = serde_json::from_value(value) { self.rating_comment = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for OrderDelivery {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "OrderDelivery"
    }
}

impl backbone_core::PersistentEntity for OrderDelivery {
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

impl backbone_orm::EntityRepoMeta for OrderDelivery {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("order_id".to_string(), "uuid".to_string());
        m.insert("address_id".to_string(), "uuid".to_string());
        m.insert("agent_id".to_string(), "uuid".to_string());
        m.insert("delivery_type".to_string(), "delivery_type".to_string());
        m.insert("status".to_string(), "delivery_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["address_line", "contact_name", "contact_phone", "currency"]
    }
}

/// Builder for OrderDelivery entity
///
/// Provides a fluent API for constructing OrderDelivery instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct OrderDeliveryBuilder {
    order_id: Option<Uuid>,
    delivery_type: Option<DeliveryType>,
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
    status: Option<DeliveryStatus>,
    dispatched_at: Option<DateTime<Utc>>,
    en_route_at: Option<DateTime<Utc>>,
    arrived_at: Option<DateTime<Utc>>,
    delivered_at: Option<DateTime<Utc>>,
    item_count: Option<i32>,
    bag_count: Option<i32>,
    bag_numbers: Option<serde_json::Value>,
    verification_method: Option<String>,
    verification_code: Option<String>,
    verified_at: Option<DateTime<Utc>>,
    photos: Option<serde_json::Value>,
    proof_photo_url: Option<String>,
    signature_url: Option<String>,
    signed_by: Option<String>,
    relation_to_customer: Option<String>,
    delivery_latitude: Option<f64>,
    delivery_longitude: Option<f64>,
    distance_from_address_meters: Option<i32>,
    delivery_fee: Option<Decimal>,
    express_fee: Option<Decimal>,
    total_fee: Option<Decimal>,
    currency: Option<String>,
    is_cod: Option<bool>,
    cod_amount: Option<Decimal>,
    cod_collected: Option<bool>,
    cod_collected_at: Option<DateTime<Utc>>,
    customer_notes: Option<String>,
    agent_notes: Option<String>,
    internal_notes: Option<String>,
    failure_reason: Option<String>,
    failure_code: Option<String>,
    attempt_count: Option<i32>,
    reschedule_count: Option<i32>,
    original_scheduled_date: Option<NaiveDate>,
    reschedule_reason: Option<String>,
    delivery_rating: Option<i32>,
    rating_comment: Option<String>,
}

impl OrderDeliveryBuilder {
    /// Set the order_id field (required)
    pub fn order_id(mut self, value: Uuid) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Set the delivery_type field (default: `DeliveryType::default()`)
    pub fn delivery_type(mut self, value: DeliveryType) -> Self {
        self.delivery_type = Some(value);
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

    /// Set the status field (default: `DeliveryStatus::default()`)
    pub fn status(mut self, value: DeliveryStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the dispatched_at field (optional)
    pub fn dispatched_at(mut self, value: DateTime<Utc>) -> Self {
        self.dispatched_at = Some(value);
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

    /// Set the delivered_at field (optional)
    pub fn delivered_at(mut self, value: DateTime<Utc>) -> Self {
        self.delivered_at = Some(value);
        self
    }

    /// Set the item_count field (default: `0`)
    pub fn item_count(mut self, value: i32) -> Self {
        self.item_count = Some(value);
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

    /// Set the verification_method field (optional)
    pub fn verification_method(mut self, value: String) -> Self {
        self.verification_method = Some(value);
        self
    }

    /// Set the verification_code field (optional)
    pub fn verification_code(mut self, value: String) -> Self {
        self.verification_code = Some(value);
        self
    }

    /// Set the verified_at field (optional)
    pub fn verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the photos field (default: `serde_json::json!([])`)
    pub fn photos(mut self, value: serde_json::Value) -> Self {
        self.photos = Some(value);
        self
    }

    /// Set the proof_photo_url field (optional)
    pub fn proof_photo_url(mut self, value: String) -> Self {
        self.proof_photo_url = Some(value);
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

    /// Set the relation_to_customer field (optional)
    pub fn relation_to_customer(mut self, value: String) -> Self {
        self.relation_to_customer = Some(value);
        self
    }

    /// Set the delivery_latitude field (optional)
    pub fn delivery_latitude(mut self, value: f64) -> Self {
        self.delivery_latitude = Some(value);
        self
    }

    /// Set the delivery_longitude field (optional)
    pub fn delivery_longitude(mut self, value: f64) -> Self {
        self.delivery_longitude = Some(value);
        self
    }

    /// Set the distance_from_address_meters field (optional)
    pub fn distance_from_address_meters(mut self, value: i32) -> Self {
        self.distance_from_address_meters = Some(value);
        self
    }

    /// Set the delivery_fee field (default: `Decimal::from(0)`)
    pub fn delivery_fee(mut self, value: Decimal) -> Self {
        self.delivery_fee = Some(value);
        self
    }

    /// Set the express_fee field (default: `Decimal::from(0)`)
    pub fn express_fee(mut self, value: Decimal) -> Self {
        self.express_fee = Some(value);
        self
    }

    /// Set the total_fee field (default: `Decimal::from(0)`)
    pub fn total_fee(mut self, value: Decimal) -> Self {
        self.total_fee = Some(value);
        self
    }

    /// Set the currency field (default: `"IDR".to_string()`)
    pub fn currency(mut self, value: String) -> Self {
        self.currency = Some(value);
        self
    }

    /// Set the is_cod field (default: `false`)
    pub fn is_cod(mut self, value: bool) -> Self {
        self.is_cod = Some(value);
        self
    }

    /// Set the cod_amount field (optional)
    pub fn cod_amount(mut self, value: Decimal) -> Self {
        self.cod_amount = Some(value);
        self
    }

    /// Set the cod_collected field (default: `false`)
    pub fn cod_collected(mut self, value: bool) -> Self {
        self.cod_collected = Some(value);
        self
    }

    /// Set the cod_collected_at field (optional)
    pub fn cod_collected_at(mut self, value: DateTime<Utc>) -> Self {
        self.cod_collected_at = Some(value);
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

    /// Set the attempt_count field (default: `1`)
    pub fn attempt_count(mut self, value: i32) -> Self {
        self.attempt_count = Some(value);
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

    /// Set the delivery_rating field (optional)
    pub fn delivery_rating(mut self, value: i32) -> Self {
        self.delivery_rating = Some(value);
        self
    }

    /// Set the rating_comment field (optional)
    pub fn rating_comment(mut self, value: String) -> Self {
        self.rating_comment = Some(value);
        self
    }

    /// Build the OrderDelivery entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<OrderDelivery, String> {
        let order_id = self.order_id.ok_or_else(|| "order_id is required".to_string())?;
        let scheduled_date = self.scheduled_date.ok_or_else(|| "scheduled_date is required".to_string())?;
        let address_line = self.address_line.ok_or_else(|| "address_line is required".to_string())?;
        let contact_name = self.contact_name.ok_or_else(|| "contact_name is required".to_string())?;
        let contact_phone = self.contact_phone.ok_or_else(|| "contact_phone is required".to_string())?;

        Ok(OrderDelivery {
            id: Uuid::new_v4(),
            order_id,
            delivery_type: self.delivery_type.unwrap_or(DeliveryType::default()),
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
            status: self.status.unwrap_or(DeliveryStatus::default()),
            dispatched_at: self.dispatched_at,
            en_route_at: self.en_route_at,
            arrived_at: self.arrived_at,
            delivered_at: self.delivered_at,
            item_count: self.item_count.unwrap_or(0),
            bag_count: self.bag_count.unwrap_or(0),
            bag_numbers: self.bag_numbers.unwrap_or(serde_json::json!([])),
            verification_method: self.verification_method,
            verification_code: self.verification_code,
            verified_at: self.verified_at,
            photos: self.photos.unwrap_or(serde_json::json!([])),
            proof_photo_url: self.proof_photo_url,
            signature_url: self.signature_url,
            signed_by: self.signed_by,
            relation_to_customer: self.relation_to_customer,
            delivery_latitude: self.delivery_latitude,
            delivery_longitude: self.delivery_longitude,
            distance_from_address_meters: self.distance_from_address_meters,
            delivery_fee: self.delivery_fee.unwrap_or(Decimal::from(0)),
            express_fee: self.express_fee.unwrap_or(Decimal::from(0)),
            total_fee: self.total_fee.unwrap_or(Decimal::from(0)),
            currency: self.currency.unwrap_or("IDR".to_string()),
            is_cod: self.is_cod.unwrap_or(false),
            cod_amount: self.cod_amount,
            cod_collected: self.cod_collected.unwrap_or(false),
            cod_collected_at: self.cod_collected_at,
            customer_notes: self.customer_notes,
            agent_notes: self.agent_notes,
            internal_notes: self.internal_notes,
            failure_reason: self.failure_reason,
            failure_code: self.failure_code,
            attempt_count: self.attempt_count.unwrap_or(1),
            reschedule_count: self.reschedule_count.unwrap_or(0),
            original_scheduled_date: self.original_scheduled_date,
            reschedule_reason: self.reschedule_reason,
            delivery_rating: self.delivery_rating,
            rating_comment: self.rating_comment,
            metadata: AuditMetadata::default(),
        })
    }
}
