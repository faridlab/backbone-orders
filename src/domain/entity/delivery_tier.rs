use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for DeliveryTier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeliveryTierId(pub Uuid);

impl DeliveryTierId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for DeliveryTierId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for DeliveryTierId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for DeliveryTierId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<DeliveryTierId> for Uuid {
    fn from(id: DeliveryTierId) -> Self { id.0 }
}

impl AsRef<Uuid> for DeliveryTierId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for DeliveryTierId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeliveryTier {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub duration_hours: i32,
    pub duration_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub is_urgent: bool,
    pub sort_order: i32,
    pub is_active: bool,
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlet_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<Uuid>,
    pub is_cloned: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl DeliveryTier {
    /// Create a builder for DeliveryTier
    pub fn builder() -> DeliveryTierBuilder {
        DeliveryTierBuilder::default()
    }

    /// Create a new DeliveryTier with required fields
    pub fn new(code: String, name: String, duration_hours: i32, duration_label: String, is_urgent: bool, sort_order: i32, is_active: bool, is_default: bool, is_cloned: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            code,
            name,
            name_en: None,
            description: None,
            duration_hours,
            duration_label,
            color: None,
            icon: None,
            is_urgent,
            sort_order,
            is_active,
            is_default,
            provider_id: None,
            outlet_id: None,
            source_id: None,
            is_cloned,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> DeliveryTierId {
        DeliveryTierId(self.id)
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

    /// Set the name_en field (chainable)
    pub fn with_name_en(mut self, value: String) -> Self {
        self.name_en = Some(value);
        self
    }

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the color field (chainable)
    pub fn with_color(mut self, value: String) -> Self {
        self.color = Some(value);
        self
    }

    /// Set the icon field (chainable)
    pub fn with_icon(mut self, value: String) -> Self {
        self.icon = Some(value);
        self
    }

    /// Set the provider_id field (chainable)
    pub fn with_provider_id(mut self, value: Uuid) -> Self {
        self.provider_id = Some(value);
        self
    }

    /// Set the outlet_id field (chainable)
    pub fn with_outlet_id(mut self, value: Uuid) -> Self {
        self.outlet_id = Some(value);
        self
    }

    /// Set the source_id field (chainable)
    pub fn with_source_id(mut self, value: Uuid) -> Self {
        self.source_id = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "code" => {
                    if let Ok(v) = serde_json::from_value(value) { self.code = v; }
                }
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "name_en" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name_en = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "duration_hours" => {
                    if let Ok(v) = serde_json::from_value(value) { self.duration_hours = v; }
                }
                "duration_label" => {
                    if let Ok(v) = serde_json::from_value(value) { self.duration_label = v; }
                }
                "color" => {
                    if let Ok(v) = serde_json::from_value(value) { self.color = v; }
                }
                "icon" => {
                    if let Ok(v) = serde_json::from_value(value) { self.icon = v; }
                }
                "is_urgent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_urgent = v; }
                }
                "sort_order" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sort_order = v; }
                }
                "is_active" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_active = v; }
                }
                "is_default" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_default = v; }
                }
                "provider_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.provider_id = v; }
                }
                "outlet_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.outlet_id = v; }
                }
                "source_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.source_id = v; }
                }
                "is_cloned" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_cloned = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for DeliveryTier {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "DeliveryTier"
    }
}

impl backbone_core::PersistentEntity for DeliveryTier {
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

impl backbone_orm::EntityRepoMeta for DeliveryTier {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("provider_id".to_string(), "uuid".to_string());
        m.insert("outlet_id".to_string(), "uuid".to_string());
        m.insert("source_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["code", "name", "duration_label"]
    }
}

/// Builder for DeliveryTier entity
///
/// Provides a fluent API for constructing DeliveryTier instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct DeliveryTierBuilder {
    code: Option<String>,
    name: Option<String>,
    name_en: Option<String>,
    description: Option<String>,
    duration_hours: Option<i32>,
    duration_label: Option<String>,
    color: Option<String>,
    icon: Option<String>,
    is_urgent: Option<bool>,
    sort_order: Option<i32>,
    is_active: Option<bool>,
    is_default: Option<bool>,
    provider_id: Option<Uuid>,
    outlet_id: Option<Uuid>,
    source_id: Option<Uuid>,
    is_cloned: Option<bool>,
}

impl DeliveryTierBuilder {
    /// Set the code field (required)
    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }

    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the name_en field (optional)
    pub fn name_en(mut self, value: String) -> Self {
        self.name_en = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the duration_hours field (required)
    pub fn duration_hours(mut self, value: i32) -> Self {
        self.duration_hours = Some(value);
        self
    }

    /// Set the duration_label field (required)
    pub fn duration_label(mut self, value: String) -> Self {
        self.duration_label = Some(value);
        self
    }

    /// Set the color field (optional)
    pub fn color(mut self, value: String) -> Self {
        self.color = Some(value);
        self
    }

    /// Set the icon field (optional)
    pub fn icon(mut self, value: String) -> Self {
        self.icon = Some(value);
        self
    }

    /// Set the is_urgent field (default: `false`)
    pub fn is_urgent(mut self, value: bool) -> Self {
        self.is_urgent = Some(value);
        self
    }

    /// Set the sort_order field (default: `0`)
    pub fn sort_order(mut self, value: i32) -> Self {
        self.sort_order = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the is_default field (default: `false`)
    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Set the provider_id field (optional)
    pub fn provider_id(mut self, value: Uuid) -> Self {
        self.provider_id = Some(value);
        self
    }

    /// Set the outlet_id field (optional)
    pub fn outlet_id(mut self, value: Uuid) -> Self {
        self.outlet_id = Some(value);
        self
    }

    /// Set the source_id field (optional)
    pub fn source_id(mut self, value: Uuid) -> Self {
        self.source_id = Some(value);
        self
    }

    /// Set the is_cloned field (default: `false`)
    pub fn is_cloned(mut self, value: bool) -> Self {
        self.is_cloned = Some(value);
        self
    }

    /// Build the DeliveryTier entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<DeliveryTier, String> {
        let code = self.code.ok_or_else(|| "code is required".to_string())?;
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let duration_hours = self.duration_hours.ok_or_else(|| "duration_hours is required".to_string())?;
        let duration_label = self.duration_label.ok_or_else(|| "duration_label is required".to_string())?;

        Ok(DeliveryTier {
            id: Uuid::new_v4(),
            code,
            name,
            name_en: self.name_en,
            description: self.description,
            duration_hours,
            duration_label,
            color: self.color,
            icon: self.icon,
            is_urgent: self.is_urgent.unwrap_or(false),
            sort_order: self.sort_order.unwrap_or(0),
            is_active: self.is_active.unwrap_or(true),
            is_default: self.is_default.unwrap_or(false),
            provider_id: self.provider_id,
            outlet_id: self.outlet_id,
            source_id: self.source_id,
            is_cloned: self.is_cloned.unwrap_or(false),
            metadata: AuditMetadata::default(),
        })
    }
}
