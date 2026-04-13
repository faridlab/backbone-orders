use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::UnitType;
use super::ItemCondition;
use super::ItemProcessingStatus;
use super::AuditMetadata;

/// Strongly-typed ID for OrderItem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderItemId(pub Uuid);

impl OrderItemId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for OrderItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OrderItemId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for OrderItemId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<OrderItemId> for Uuid {
    fn from(id: OrderItemId) -> Self { id.0 }
}

impl AsRef<Uuid> for OrderItemId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for OrderItemId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    pub quantity: i32,
    pub unit_type: UnitType,
    pub unit_price: Decimal,
    pub subtotal: Decimal,
    pub discount: Decimal,
    pub addon_total: Decimal,
    pub total: Decimal,
    pub currency: String,
    pub addons: serde_json::Value,
    pub condition_on_receipt: ItemCondition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_notes: Option<String>,
    pub processing_status: ItemProcessingStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_tier_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl OrderItem {
    /// Create a builder for OrderItem
    pub fn builder() -> OrderItemBuilder {
        OrderItemBuilder::default()
    }

    /// Create a new OrderItem with required fields
    pub fn new(order_id: Uuid, quantity: i32, unit_type: UnitType, unit_price: Decimal, subtotal: Decimal, discount: Decimal, addon_total: Decimal, total: Decimal, currency: String, addons: serde_json::Value, condition_on_receipt: ItemCondition, processing_status: ItemProcessingStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            service_id: None,
            item_name: None,
            quantity,
            unit_type,
            unit_price,
            subtotal,
            discount,
            addon_total,
            total,
            currency,
            addons,
            condition_on_receipt,
            condition_notes: None,
            processing_status,
            special_instructions: None,
            processing_notes: None,
            delivery_tier_id: None,
            started_at: None,
            completed_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> OrderItemId {
        OrderItemId(self.id)
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

    /// Set the service_id field (chainable)
    pub fn with_service_id(mut self, value: Uuid) -> Self {
        self.service_id = Some(value);
        self
    }

    /// Set the item_name field (chainable)
    pub fn with_item_name(mut self, value: String) -> Self {
        self.item_name = Some(value);
        self
    }

    /// Set the condition_notes field (chainable)
    pub fn with_condition_notes(mut self, value: String) -> Self {
        self.condition_notes = Some(value);
        self
    }

    /// Set the special_instructions field (chainable)
    pub fn with_special_instructions(mut self, value: String) -> Self {
        self.special_instructions = Some(value);
        self
    }

    /// Set the processing_notes field (chainable)
    pub fn with_processing_notes(mut self, value: String) -> Self {
        self.processing_notes = Some(value);
        self
    }

    /// Set the delivery_tier_id field (chainable)
    pub fn with_delivery_tier_id(mut self, value: Uuid) -> Self {
        self.delivery_tier_id = Some(value);
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
                "service_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.service_id = v; }
                }
                "item_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.item_name = v; }
                }
                "quantity" => {
                    if let Ok(v) = serde_json::from_value(value) { self.quantity = v; }
                }
                "unit_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.unit_type = v; }
                }
                "unit_price" => {
                    if let Ok(v) = serde_json::from_value(value) { self.unit_price = v; }
                }
                "subtotal" => {
                    if let Ok(v) = serde_json::from_value(value) { self.subtotal = v; }
                }
                "discount" => {
                    if let Ok(v) = serde_json::from_value(value) { self.discount = v; }
                }
                "addon_total" => {
                    if let Ok(v) = serde_json::from_value(value) { self.addon_total = v; }
                }
                "total" => {
                    if let Ok(v) = serde_json::from_value(value) { self.total = v; }
                }
                "currency" => {
                    if let Ok(v) = serde_json::from_value(value) { self.currency = v; }
                }
                "addons" => {
                    if let Ok(v) = serde_json::from_value(value) { self.addons = v; }
                }
                "condition_on_receipt" => {
                    if let Ok(v) = serde_json::from_value(value) { self.condition_on_receipt = v; }
                }
                "condition_notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.condition_notes = v; }
                }
                "processing_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.processing_status = v; }
                }
                "special_instructions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.special_instructions = v; }
                }
                "processing_notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.processing_notes = v; }
                }
                "delivery_tier_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivery_tier_id = v; }
                }
                "started_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.started_at = v; }
                }
                "completed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.completed_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for OrderItem {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "OrderItem"
    }
}

impl backbone_core::PersistentEntity for OrderItem {
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

impl backbone_orm::EntityRepoMeta for OrderItem {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("order_id".to_string(), "uuid".to_string());
        m.insert("service_id".to_string(), "uuid".to_string());
        m.insert("delivery_tier_id".to_string(), "uuid".to_string());
        m.insert("unit_type".to_string(), "unit_type".to_string());
        m.insert("condition_on_receipt".to_string(), "item_condition".to_string());
        m.insert("processing_status".to_string(), "item_processing_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["currency"]
    }
}

/// Builder for OrderItem entity
///
/// Provides a fluent API for constructing OrderItem instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct OrderItemBuilder {
    order_id: Option<Uuid>,
    service_id: Option<Uuid>,
    item_name: Option<String>,
    quantity: Option<i32>,
    unit_type: Option<UnitType>,
    unit_price: Option<Decimal>,
    subtotal: Option<Decimal>,
    discount: Option<Decimal>,
    addon_total: Option<Decimal>,
    total: Option<Decimal>,
    currency: Option<String>,
    addons: Option<serde_json::Value>,
    condition_on_receipt: Option<ItemCondition>,
    condition_notes: Option<String>,
    processing_status: Option<ItemProcessingStatus>,
    special_instructions: Option<String>,
    processing_notes: Option<String>,
    delivery_tier_id: Option<Uuid>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
}

impl OrderItemBuilder {
    /// Set the order_id field (required)
    pub fn order_id(mut self, value: Uuid) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Set the service_id field (optional)
    pub fn service_id(mut self, value: Uuid) -> Self {
        self.service_id = Some(value);
        self
    }

    /// Set the item_name field (optional)
    pub fn item_name(mut self, value: String) -> Self {
        self.item_name = Some(value);
        self
    }

    /// Set the quantity field (default: `1`)
    pub fn quantity(mut self, value: i32) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Set the unit_type field (default: `UnitType::default()`)
    pub fn unit_type(mut self, value: UnitType) -> Self {
        self.unit_type = Some(value);
        self
    }

    /// Set the unit_price field (required)
    pub fn unit_price(mut self, value: Decimal) -> Self {
        self.unit_price = Some(value);
        self
    }

    /// Set the subtotal field (required)
    pub fn subtotal(mut self, value: Decimal) -> Self {
        self.subtotal = Some(value);
        self
    }

    /// Set the discount field (default: `Decimal::from(0)`)
    pub fn discount(mut self, value: Decimal) -> Self {
        self.discount = Some(value);
        self
    }

    /// Set the addon_total field (default: `Decimal::from(0)`)
    pub fn addon_total(mut self, value: Decimal) -> Self {
        self.addon_total = Some(value);
        self
    }

    /// Set the total field (required)
    pub fn total(mut self, value: Decimal) -> Self {
        self.total = Some(value);
        self
    }

    /// Set the currency field (default: `"IDR".to_string()`)
    pub fn currency(mut self, value: String) -> Self {
        self.currency = Some(value);
        self
    }

    /// Set the addons field (default: `serde_json::json!([])`)
    pub fn addons(mut self, value: serde_json::Value) -> Self {
        self.addons = Some(value);
        self
    }

    /// Set the condition_on_receipt field (default: `ItemCondition::default()`)
    pub fn condition_on_receipt(mut self, value: ItemCondition) -> Self {
        self.condition_on_receipt = Some(value);
        self
    }

    /// Set the condition_notes field (optional)
    pub fn condition_notes(mut self, value: String) -> Self {
        self.condition_notes = Some(value);
        self
    }

    /// Set the processing_status field (default: `ItemProcessingStatus::default()`)
    pub fn processing_status(mut self, value: ItemProcessingStatus) -> Self {
        self.processing_status = Some(value);
        self
    }

    /// Set the special_instructions field (optional)
    pub fn special_instructions(mut self, value: String) -> Self {
        self.special_instructions = Some(value);
        self
    }

    /// Set the processing_notes field (optional)
    pub fn processing_notes(mut self, value: String) -> Self {
        self.processing_notes = Some(value);
        self
    }

    /// Set the delivery_tier_id field (optional)
    pub fn delivery_tier_id(mut self, value: Uuid) -> Self {
        self.delivery_tier_id = Some(value);
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

    /// Build the OrderItem entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<OrderItem, String> {
        let order_id = self.order_id.ok_or_else(|| "order_id is required".to_string())?;
        let unit_price = self.unit_price.ok_or_else(|| "unit_price is required".to_string())?;
        let subtotal = self.subtotal.ok_or_else(|| "subtotal is required".to_string())?;
        let total = self.total.ok_or_else(|| "total is required".to_string())?;

        Ok(OrderItem {
            id: Uuid::new_v4(),
            order_id,
            service_id: self.service_id,
            item_name: self.item_name,
            quantity: self.quantity.unwrap_or(1),
            unit_type: self.unit_type.unwrap_or(UnitType::default()),
            unit_price,
            subtotal,
            discount: self.discount.unwrap_or(Decimal::from(0)),
            addon_total: self.addon_total.unwrap_or(Decimal::from(0)),
            total,
            currency: self.currency.unwrap_or("IDR".to_string()),
            addons: self.addons.unwrap_or(serde_json::json!([])),
            condition_on_receipt: self.condition_on_receipt.unwrap_or(ItemCondition::default()),
            condition_notes: self.condition_notes,
            processing_status: self.processing_status.unwrap_or(ItemProcessingStatus::default()),
            special_instructions: self.special_instructions,
            processing_notes: self.processing_notes,
            delivery_tier_id: self.delivery_tier_id,
            started_at: self.started_at,
            completed_at: self.completed_at,
            metadata: AuditMetadata::default(),
        })
    }
}
