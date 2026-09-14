//! backbone-orders — dormant.
//!
//! This module owns the order workflow (pickups, deliveries, the status
//! timeline). It was declared dormant and its generated 4-layer tree was
//! retired; the schema YAML, the config and the docs were deliberately kept so
//! the module can be regenerated when it is revived.
//!
//! The crate keeps a library target on purpose. A package with no target at
//! all cannot be built, and a module that cannot be built also cannot be
//! linted, audited, or carried along by a framework re-pin train — so it
//! drifts silently until someone tries to revive it and finds the manifest
//! pointing at a tree that is not there.
//!
//! To revive: regenerate from the schema (`metaphor make entity <Name>`),
//! which replaces this file with the generated module surface.

/// The module's crate name, as registered in the workspace manifest.
pub const MODULE: &str = "backbone-orders";

/// Whether this module currently exposes a domain surface.
///
/// Dormant means: schema and config are present, generated code is not. A
/// composing service must not mount this module while it reads false.
pub const IS_DORMANT: bool = true;
