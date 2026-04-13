use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::TimelineEntryType;
use super::AuditMetadata;

/// Strongly-typed ID for OrderStatusTimeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderStatusTimelineId(pub Uuid);

impl OrderStatusTimelineId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for OrderStatusTimelineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OrderStatusTimelineId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for OrderStatusTimelineId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<OrderStatusTimelineId> for Uuid {
    fn from(id: OrderStatusTimelineId) -> Self { id.0 }
}

impl AsRef<Uuid> for OrderStatusTimelineId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for OrderStatusTimelineId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderStatusTimeline {
    pub id: Uuid,
    pub order_id: Uuid,
    pub entry_type: TimelineEntryType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_status: Option<String>,
    pub occurred_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_role: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    pub is_customer_visible: bool,
    pub is_provider_visible: bool,
    pub is_agent_visible: bool,
    pub attachments: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    pub device_info: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl OrderStatusTimeline {
    /// Create a builder for OrderStatusTimeline
    pub fn builder() -> OrderStatusTimelineBuilder {
        OrderStatusTimelineBuilder::default()
    }

    /// Create a new OrderStatusTimeline with required fields
    pub fn new(order_id: Uuid, entry_type: TimelineEntryType, occurred_at: DateTime<Utc>, title: String, is_customer_visible: bool, is_provider_visible: bool, is_agent_visible: bool, attachments: serde_json::Value, device_info: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            entry_type,
            from_status: None,
            to_status: None,
            occurred_at,
            actor_type: None,
            actor_id: None,
            actor_name: None,
            actor_role: None,
            title,
            description: None,
            latitude: None,
            longitude: None,
            location_name: None,
            is_customer_visible,
            is_provider_visible,
            is_agent_visible,
            attachments,
            duration_minutes: None,
            channel: None,
            device_info,
            ip_address: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> OrderStatusTimelineId {
        OrderStatusTimelineId(self.id)
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


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the from_status field (chainable)
    pub fn with_from_status(mut self, value: String) -> Self {
        self.from_status = Some(value);
        self
    }

    /// Set the to_status field (chainable)
    pub fn with_to_status(mut self, value: String) -> Self {
        self.to_status = Some(value);
        self
    }

    /// Set the actor_type field (chainable)
    pub fn with_actor_type(mut self, value: String) -> Self {
        self.actor_type = Some(value);
        self
    }

    /// Set the actor_id field (chainable)
    pub fn with_actor_id(mut self, value: Uuid) -> Self {
        self.actor_id = Some(value);
        self
    }

    /// Set the actor_name field (chainable)
    pub fn with_actor_name(mut self, value: String) -> Self {
        self.actor_name = Some(value);
        self
    }

    /// Set the actor_role field (chainable)
    pub fn with_actor_role(mut self, value: String) -> Self {
        self.actor_role = Some(value);
        self
    }

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
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

    /// Set the location_name field (chainable)
    pub fn with_location_name(mut self, value: String) -> Self {
        self.location_name = Some(value);
        self
    }

    /// Set the duration_minutes field (chainable)
    pub fn with_duration_minutes(mut self, value: i32) -> Self {
        self.duration_minutes = Some(value);
        self
    }

    /// Set the channel field (chainable)
    pub fn with_channel(mut self, value: String) -> Self {
        self.channel = Some(value);
        self
    }

    /// Set the ip_address field (chainable)
    pub fn with_ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
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
                "entry_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.entry_type = v; }
                }
                "from_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.from_status = v; }
                }
                "to_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.to_status = v; }
                }
                "occurred_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.occurred_at = v; }
                }
                "actor_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.actor_type = v; }
                }
                "actor_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.actor_id = v; }
                }
                "actor_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.actor_name = v; }
                }
                "actor_role" => {
                    if let Ok(v) = serde_json::from_value(value) { self.actor_role = v; }
                }
                "title" => {
                    if let Ok(v) = serde_json::from_value(value) { self.title = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "latitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.latitude = v; }
                }
                "longitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.longitude = v; }
                }
                "location_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.location_name = v; }
                }
                "is_customer_visible" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_customer_visible = v; }
                }
                "is_provider_visible" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_provider_visible = v; }
                }
                "is_agent_visible" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_agent_visible = v; }
                }
                "attachments" => {
                    if let Ok(v) = serde_json::from_value(value) { self.attachments = v; }
                }
                "duration_minutes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.duration_minutes = v; }
                }
                "channel" => {
                    if let Ok(v) = serde_json::from_value(value) { self.channel = v; }
                }
                "device_info" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_info = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for OrderStatusTimeline {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "OrderStatusTimeline"
    }
}

impl backbone_core::PersistentEntity for OrderStatusTimeline {
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

impl backbone_orm::EntityRepoMeta for OrderStatusTimeline {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("order_id".to_string(), "uuid".to_string());
        m.insert("actor_id".to_string(), "uuid".to_string());
        m.insert("entry_type".to_string(), "timeline_entry_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["title"]
    }
}

/// Builder for OrderStatusTimeline entity
///
/// Provides a fluent API for constructing OrderStatusTimeline instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct OrderStatusTimelineBuilder {
    order_id: Option<Uuid>,
    entry_type: Option<TimelineEntryType>,
    from_status: Option<String>,
    to_status: Option<String>,
    occurred_at: Option<DateTime<Utc>>,
    actor_type: Option<String>,
    actor_id: Option<Uuid>,
    actor_name: Option<String>,
    actor_role: Option<String>,
    title: Option<String>,
    description: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    location_name: Option<String>,
    is_customer_visible: Option<bool>,
    is_provider_visible: Option<bool>,
    is_agent_visible: Option<bool>,
    attachments: Option<serde_json::Value>,
    duration_minutes: Option<i32>,
    channel: Option<String>,
    device_info: Option<serde_json::Value>,
    ip_address: Option<String>,
}

impl OrderStatusTimelineBuilder {
    /// Set the order_id field (required)
    pub fn order_id(mut self, value: Uuid) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Set the entry_type field (default: `TimelineEntryType::default()`)
    pub fn entry_type(mut self, value: TimelineEntryType) -> Self {
        self.entry_type = Some(value);
        self
    }

    /// Set the from_status field (optional)
    pub fn from_status(mut self, value: String) -> Self {
        self.from_status = Some(value);
        self
    }

    /// Set the to_status field (optional)
    pub fn to_status(mut self, value: String) -> Self {
        self.to_status = Some(value);
        self
    }

    /// Set the occurred_at field (required)
    pub fn occurred_at(mut self, value: DateTime<Utc>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    /// Set the actor_type field (optional)
    pub fn actor_type(mut self, value: String) -> Self {
        self.actor_type = Some(value);
        self
    }

    /// Set the actor_id field (optional)
    pub fn actor_id(mut self, value: Uuid) -> Self {
        self.actor_id = Some(value);
        self
    }

    /// Set the actor_name field (optional)
    pub fn actor_name(mut self, value: String) -> Self {
        self.actor_name = Some(value);
        self
    }

    /// Set the actor_role field (optional)
    pub fn actor_role(mut self, value: String) -> Self {
        self.actor_role = Some(value);
        self
    }

    /// Set the title field (required)
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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

    /// Set the location_name field (optional)
    pub fn location_name(mut self, value: String) -> Self {
        self.location_name = Some(value);
        self
    }

    /// Set the is_customer_visible field (default: `true`)
    pub fn is_customer_visible(mut self, value: bool) -> Self {
        self.is_customer_visible = Some(value);
        self
    }

    /// Set the is_provider_visible field (default: `true`)
    pub fn is_provider_visible(mut self, value: bool) -> Self {
        self.is_provider_visible = Some(value);
        self
    }

    /// Set the is_agent_visible field (default: `true`)
    pub fn is_agent_visible(mut self, value: bool) -> Self {
        self.is_agent_visible = Some(value);
        self
    }

    /// Set the attachments field (default: `serde_json::json!([])`)
    pub fn attachments(mut self, value: serde_json::Value) -> Self {
        self.attachments = Some(value);
        self
    }

    /// Set the duration_minutes field (optional)
    pub fn duration_minutes(mut self, value: i32) -> Self {
        self.duration_minutes = Some(value);
        self
    }

    /// Set the channel field (optional)
    pub fn channel(mut self, value: String) -> Self {
        self.channel = Some(value);
        self
    }

    /// Set the device_info field (default: `serde_json::json!({})`)
    pub fn device_info(mut self, value: serde_json::Value) -> Self {
        self.device_info = Some(value);
        self
    }

    /// Set the ip_address field (optional)
    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Build the OrderStatusTimeline entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<OrderStatusTimeline, String> {
        let order_id = self.order_id.ok_or_else(|| "order_id is required".to_string())?;
        let occurred_at = self.occurred_at.ok_or_else(|| "occurred_at is required".to_string())?;
        let title = self.title.ok_or_else(|| "title is required".to_string())?;

        Ok(OrderStatusTimeline {
            id: Uuid::new_v4(),
            order_id,
            entry_type: self.entry_type.unwrap_or(TimelineEntryType::default()),
            from_status: self.from_status,
            to_status: self.to_status,
            occurred_at,
            actor_type: self.actor_type,
            actor_id: self.actor_id,
            actor_name: self.actor_name,
            actor_role: self.actor_role,
            title,
            description: self.description,
            latitude: self.latitude,
            longitude: self.longitude,
            location_name: self.location_name,
            is_customer_visible: self.is_customer_visible.unwrap_or(true),
            is_provider_visible: self.is_provider_visible.unwrap_or(true),
            is_agent_visible: self.is_agent_visible.unwrap_or(true),
            attachments: self.attachments.unwrap_or(serde_json::json!([])),
            duration_minutes: self.duration_minutes,
            channel: self.channel,
            device_info: self.device_info.unwrap_or(serde_json::json!({})),
            ip_address: self.ip_address,
            metadata: AuditMetadata::default(),
        })
    }
}
