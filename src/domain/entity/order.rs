use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::OrderType;
use super::OrderStatus;
use super::FulfillmentMethod;
use super::PaymentStatus;
use super::CancellationReason;
use super::AuditMetadata;

/// Strongly-typed ID for Order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderId(pub Uuid);

impl OrderId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for OrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OrderId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for OrderId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<OrderId> for Uuid {
    fn from(id: OrderId) -> Self { id.0 }
}

impl AsRef<Uuid> for OrderId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for OrderId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,
    pub customer_id: Uuid,
    pub provider_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlet_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate_order_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_tier_id: Option<Uuid>,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub fulfillment_method: FulfillmentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_ready_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_ready_at: Option<DateTime<Utc>>,
    pub currency: String,
    pub subtotal: Decimal,
    pub discount_amount: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_id: Option<Uuid>,
    pub delivery_fee: Decimal,
    pub pickup_fee: Decimal,
    pub tax_amount: Decimal,
    pub platform_fee: Decimal,
    pub grand_total: Decimal,
    pub payment_status: PaymentStatus,
    pub paid_amount: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<CancellationReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_by: Option<Uuid>,
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Order {
    /// Create a builder for Order
    pub fn builder() -> OrderBuilder {
        OrderBuilder::default()
    }

    /// Create a new Order with required fields
    pub fn new(order_number: String, customer_id: Uuid, provider_id: Uuid, order_type: OrderType, status: OrderStatus, fulfillment_method: FulfillmentMethod, currency: String, subtotal: Decimal, discount_amount: Decimal, delivery_fee: Decimal, pickup_fee: Decimal, tax_amount: Decimal, platform_fee: Decimal, grand_total: Decimal, payment_status: PaymentStatus, paid_amount: Decimal, data: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_number,
            customer_id,
            provider_id,
            outlet_id: None,
            corporate_order_id: None,
            delivery_tier_id: None,
            order_type,
            status,
            fulfillment_method,
            estimated_ready_at: None,
            actual_ready_at: None,
            currency,
            subtotal,
            discount_amount,
            promo_id: None,
            delivery_fee,
            pickup_fee,
            tax_amount,
            platform_fee,
            grand_total,
            payment_status,
            paid_amount,
            paid_at: None,
            special_instructions: None,
            provider_notes: None,
            cancelled_at: None,
            cancellation_reason: None,
            cancellation_note: None,
            cancelled_by: None,
            data,
            source: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> OrderId {
        OrderId(self.id)
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
    pub fn status(&self) -> &OrderStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the outlet_id field (chainable)
    pub fn with_outlet_id(mut self, value: Uuid) -> Self {
        self.outlet_id = Some(value);
        self
    }

    /// Set the corporate_order_id field (chainable)
    pub fn with_corporate_order_id(mut self, value: Uuid) -> Self {
        self.corporate_order_id = Some(value);
        self
    }

    /// Set the delivery_tier_id field (chainable)
    pub fn with_delivery_tier_id(mut self, value: Uuid) -> Self {
        self.delivery_tier_id = Some(value);
        self
    }

    /// Set the estimated_ready_at field (chainable)
    pub fn with_estimated_ready_at(mut self, value: DateTime<Utc>) -> Self {
        self.estimated_ready_at = Some(value);
        self
    }

    /// Set the actual_ready_at field (chainable)
    pub fn with_actual_ready_at(mut self, value: DateTime<Utc>) -> Self {
        self.actual_ready_at = Some(value);
        self
    }

    /// Set the promo_id field (chainable)
    pub fn with_promo_id(mut self, value: Uuid) -> Self {
        self.promo_id = Some(value);
        self
    }

    /// Set the paid_at field (chainable)
    pub fn with_paid_at(mut self, value: DateTime<Utc>) -> Self {
        self.paid_at = Some(value);
        self
    }

    /// Set the special_instructions field (chainable)
    pub fn with_special_instructions(mut self, value: String) -> Self {
        self.special_instructions = Some(value);
        self
    }

    /// Set the provider_notes field (chainable)
    pub fn with_provider_notes(mut self, value: String) -> Self {
        self.provider_notes = Some(value);
        self
    }

    /// Set the cancelled_at field (chainable)
    pub fn with_cancelled_at(mut self, value: DateTime<Utc>) -> Self {
        self.cancelled_at = Some(value);
        self
    }

    /// Set the cancellation_reason field (chainable)
    pub fn with_cancellation_reason(mut self, value: CancellationReason) -> Self {
        self.cancellation_reason = Some(value);
        self
    }

    /// Set the cancellation_note field (chainable)
    pub fn with_cancellation_note(mut self, value: String) -> Self {
        self.cancellation_note = Some(value);
        self
    }

    /// Set the cancelled_by field (chainable)
    pub fn with_cancelled_by(mut self, value: Uuid) -> Self {
        self.cancelled_by = Some(value);
        self
    }

    /// Set the source field (chainable)
    pub fn with_source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "order_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.order_number = v; }
                }
                "customer_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.customer_id = v; }
                }
                "provider_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.provider_id = v; }
                }
                "outlet_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.outlet_id = v; }
                }
                "corporate_order_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.corporate_order_id = v; }
                }
                "delivery_tier_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivery_tier_id = v; }
                }
                "order_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.order_type = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "fulfillment_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.fulfillment_method = v; }
                }
                "estimated_ready_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.estimated_ready_at = v; }
                }
                "actual_ready_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.actual_ready_at = v; }
                }
                "currency" => {
                    if let Ok(v) = serde_json::from_value(value) { self.currency = v; }
                }
                "subtotal" => {
                    if let Ok(v) = serde_json::from_value(value) { self.subtotal = v; }
                }
                "discount_amount" => {
                    if let Ok(v) = serde_json::from_value(value) { self.discount_amount = v; }
                }
                "promo_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.promo_id = v; }
                }
                "delivery_fee" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivery_fee = v; }
                }
                "pickup_fee" => {
                    if let Ok(v) = serde_json::from_value(value) { self.pickup_fee = v; }
                }
                "tax_amount" => {
                    if let Ok(v) = serde_json::from_value(value) { self.tax_amount = v; }
                }
                "platform_fee" => {
                    if let Ok(v) = serde_json::from_value(value) { self.platform_fee = v; }
                }
                "grand_total" => {
                    if let Ok(v) = serde_json::from_value(value) { self.grand_total = v; }
                }
                "payment_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.payment_status = v; }
                }
                "paid_amount" => {
                    if let Ok(v) = serde_json::from_value(value) { self.paid_amount = v; }
                }
                "paid_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.paid_at = v; }
                }
                "special_instructions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.special_instructions = v; }
                }
                "provider_notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.provider_notes = v; }
                }
                "cancelled_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cancelled_at = v; }
                }
                "cancellation_reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cancellation_reason = v; }
                }
                "cancellation_note" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cancellation_note = v; }
                }
                "cancelled_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cancelled_by = v; }
                }
                "data" => {
                    if let Ok(v) = serde_json::from_value(value) { self.data = v; }
                }
                "source" => {
                    if let Ok(v) = serde_json::from_value(value) { self.source = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Order {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Order"
    }
}

impl backbone_core::PersistentEntity for Order {
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

impl backbone_orm::EntityRepoMeta for Order {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("customer_id".to_string(), "uuid".to_string());
        m.insert("provider_id".to_string(), "uuid".to_string());
        m.insert("outlet_id".to_string(), "uuid".to_string());
        m.insert("corporate_order_id".to_string(), "uuid".to_string());
        m.insert("delivery_tier_id".to_string(), "uuid".to_string());
        m.insert("promo_id".to_string(), "uuid".to_string());
        m.insert("order_type".to_string(), "order_type".to_string());
        m.insert("status".to_string(), "order_status".to_string());
        m.insert("fulfillment_method".to_string(), "fulfillment_method".to_string());
        m.insert("payment_status".to_string(), "payment_status".to_string());
        m.insert("cancellation_reason".to_string(), "cancellation_reason".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["order_number", "currency"]
    }
}

/// Builder for Order entity
///
/// Provides a fluent API for constructing Order instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct OrderBuilder {
    order_number: Option<String>,
    customer_id: Option<Uuid>,
    provider_id: Option<Uuid>,
    outlet_id: Option<Uuid>,
    corporate_order_id: Option<Uuid>,
    delivery_tier_id: Option<Uuid>,
    order_type: Option<OrderType>,
    status: Option<OrderStatus>,
    fulfillment_method: Option<FulfillmentMethod>,
    estimated_ready_at: Option<DateTime<Utc>>,
    actual_ready_at: Option<DateTime<Utc>>,
    currency: Option<String>,
    subtotal: Option<Decimal>,
    discount_amount: Option<Decimal>,
    promo_id: Option<Uuid>,
    delivery_fee: Option<Decimal>,
    pickup_fee: Option<Decimal>,
    tax_amount: Option<Decimal>,
    platform_fee: Option<Decimal>,
    grand_total: Option<Decimal>,
    payment_status: Option<PaymentStatus>,
    paid_amount: Option<Decimal>,
    paid_at: Option<DateTime<Utc>>,
    special_instructions: Option<String>,
    provider_notes: Option<String>,
    cancelled_at: Option<DateTime<Utc>>,
    cancellation_reason: Option<CancellationReason>,
    cancellation_note: Option<String>,
    cancelled_by: Option<Uuid>,
    data: Option<serde_json::Value>,
    source: Option<String>,
}

impl OrderBuilder {
    /// Set the order_number field (required)
    pub fn order_number(mut self, value: String) -> Self {
        self.order_number = Some(value);
        self
    }

    /// Set the customer_id field (required)
    pub fn customer_id(mut self, value: Uuid) -> Self {
        self.customer_id = Some(value);
        self
    }

    /// Set the provider_id field (required)
    pub fn provider_id(mut self, value: Uuid) -> Self {
        self.provider_id = Some(value);
        self
    }

    /// Set the outlet_id field (optional)
    pub fn outlet_id(mut self, value: Uuid) -> Self {
        self.outlet_id = Some(value);
        self
    }

    /// Set the corporate_order_id field (optional)
    pub fn corporate_order_id(mut self, value: Uuid) -> Self {
        self.corporate_order_id = Some(value);
        self
    }

    /// Set the delivery_tier_id field (optional)
    pub fn delivery_tier_id(mut self, value: Uuid) -> Self {
        self.delivery_tier_id = Some(value);
        self
    }

    /// Set the order_type field (default: `OrderType::default()`)
    pub fn order_type(mut self, value: OrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    /// Set the status field (default: `OrderStatus::default()`)
    pub fn status(mut self, value: OrderStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the fulfillment_method field (default: `FulfillmentMethod::default()`)
    pub fn fulfillment_method(mut self, value: FulfillmentMethod) -> Self {
        self.fulfillment_method = Some(value);
        self
    }

    /// Set the estimated_ready_at field (optional)
    pub fn estimated_ready_at(mut self, value: DateTime<Utc>) -> Self {
        self.estimated_ready_at = Some(value);
        self
    }

    /// Set the actual_ready_at field (optional)
    pub fn actual_ready_at(mut self, value: DateTime<Utc>) -> Self {
        self.actual_ready_at = Some(value);
        self
    }

    /// Set the currency field (default: `"IDR".to_string()`)
    pub fn currency(mut self, value: String) -> Self {
        self.currency = Some(value);
        self
    }

    /// Set the subtotal field (required)
    pub fn subtotal(mut self, value: Decimal) -> Self {
        self.subtotal = Some(value);
        self
    }

    /// Set the discount_amount field (default: `Decimal::from(0)`)
    pub fn discount_amount(mut self, value: Decimal) -> Self {
        self.discount_amount = Some(value);
        self
    }

    /// Set the promo_id field (optional)
    pub fn promo_id(mut self, value: Uuid) -> Self {
        self.promo_id = Some(value);
        self
    }

    /// Set the delivery_fee field (default: `Decimal::from(0)`)
    pub fn delivery_fee(mut self, value: Decimal) -> Self {
        self.delivery_fee = Some(value);
        self
    }

    /// Set the pickup_fee field (default: `Decimal::from(0)`)
    pub fn pickup_fee(mut self, value: Decimal) -> Self {
        self.pickup_fee = Some(value);
        self
    }

    /// Set the tax_amount field (default: `Decimal::from(0)`)
    pub fn tax_amount(mut self, value: Decimal) -> Self {
        self.tax_amount = Some(value);
        self
    }

    /// Set the platform_fee field (default: `Decimal::from(0)`)
    pub fn platform_fee(mut self, value: Decimal) -> Self {
        self.platform_fee = Some(value);
        self
    }

    /// Set the grand_total field (required)
    pub fn grand_total(mut self, value: Decimal) -> Self {
        self.grand_total = Some(value);
        self
    }

    /// Set the payment_status field (default: `PaymentStatus::default()`)
    pub fn payment_status(mut self, value: PaymentStatus) -> Self {
        self.payment_status = Some(value);
        self
    }

    /// Set the paid_amount field (default: `Decimal::from(0)`)
    pub fn paid_amount(mut self, value: Decimal) -> Self {
        self.paid_amount = Some(value);
        self
    }

    /// Set the paid_at field (optional)
    pub fn paid_at(mut self, value: DateTime<Utc>) -> Self {
        self.paid_at = Some(value);
        self
    }

    /// Set the special_instructions field (optional)
    pub fn special_instructions(mut self, value: String) -> Self {
        self.special_instructions = Some(value);
        self
    }

    /// Set the provider_notes field (optional)
    pub fn provider_notes(mut self, value: String) -> Self {
        self.provider_notes = Some(value);
        self
    }

    /// Set the cancelled_at field (optional)
    pub fn cancelled_at(mut self, value: DateTime<Utc>) -> Self {
        self.cancelled_at = Some(value);
        self
    }

    /// Set the cancellation_reason field (optional)
    pub fn cancellation_reason(mut self, value: CancellationReason) -> Self {
        self.cancellation_reason = Some(value);
        self
    }

    /// Set the cancellation_note field (optional)
    pub fn cancellation_note(mut self, value: String) -> Self {
        self.cancellation_note = Some(value);
        self
    }

    /// Set the cancelled_by field (optional)
    pub fn cancelled_by(mut self, value: Uuid) -> Self {
        self.cancelled_by = Some(value);
        self
    }

    /// Set the data field (default: `serde_json::json!({"total_items":0,"total_weight_kg":null,"actual_weight_kg":null,"is_recurring":false,"is_overflow":false,"points_earned":0,"points_redeemed":0,"points_discount":0})`)
    pub fn data(mut self, value: serde_json::Value) -> Self {
        self.data = Some(value);
        self
    }

    /// Set the source field (optional)
    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }

    /// Build the Order entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Order, String> {
        let order_number = self.order_number.ok_or_else(|| "order_number is required".to_string())?;
        let customer_id = self.customer_id.ok_or_else(|| "customer_id is required".to_string())?;
        let provider_id = self.provider_id.ok_or_else(|| "provider_id is required".to_string())?;
        let subtotal = self.subtotal.ok_or_else(|| "subtotal is required".to_string())?;
        let grand_total = self.grand_total.ok_or_else(|| "grand_total is required".to_string())?;

        Ok(Order {
            id: Uuid::new_v4(),
            order_number,
            customer_id,
            provider_id,
            outlet_id: self.outlet_id,
            corporate_order_id: self.corporate_order_id,
            delivery_tier_id: self.delivery_tier_id,
            order_type: self.order_type.unwrap_or(OrderType::default()),
            status: self.status.unwrap_or(OrderStatus::default()),
            fulfillment_method: self.fulfillment_method.unwrap_or(FulfillmentMethod::default()),
            estimated_ready_at: self.estimated_ready_at,
            actual_ready_at: self.actual_ready_at,
            currency: self.currency.unwrap_or("IDR".to_string()),
            subtotal,
            discount_amount: self.discount_amount.unwrap_or(Decimal::from(0)),
            promo_id: self.promo_id,
            delivery_fee: self.delivery_fee.unwrap_or(Decimal::from(0)),
            pickup_fee: self.pickup_fee.unwrap_or(Decimal::from(0)),
            tax_amount: self.tax_amount.unwrap_or(Decimal::from(0)),
            platform_fee: self.platform_fee.unwrap_or(Decimal::from(0)),
            grand_total,
            payment_status: self.payment_status.unwrap_or(PaymentStatus::default()),
            paid_amount: self.paid_amount.unwrap_or(Decimal::from(0)),
            paid_at: self.paid_at,
            special_instructions: self.special_instructions,
            provider_notes: self.provider_notes,
            cancelled_at: self.cancelled_at,
            cancellation_reason: self.cancellation_reason,
            cancellation_note: self.cancellation_note,
            cancelled_by: self.cancelled_by,
            data: self.data.unwrap_or(serde_json::json!({"total_items":0,"total_weight_kg":null,"actual_weight_kg":null,"is_recurring":false,"is_overflow":false,"points_earned":0,"points_redeemed":0,"points_discount":0})),
            source: self.source,
            metadata: AuditMetadata::default(),
        })
    }
}
