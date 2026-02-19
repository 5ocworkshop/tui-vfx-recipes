// <FILE>src/registry/dyn_recipe.rs</FILE> - <DESC>Type-erased recipe interface</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>WG6: Recipe String Ownership Refactor</WCTX>
// <CLOG>BREAKING: meta() now returns RecipeMeta with Arc<str> fields (Clone, not Copy). Implementations must use .clone()</CLOG>

use serde_json::Value;
use tui_vfx_core::SchemaNode;

use crate::recipes::types::{RecipeMeta, RustSnippet};

/// Type-erased recipe interface suitable for catalogs/configurators.
///
/// This intentionally focuses on:
/// - listing metadata
/// - exposing schema
/// - exporting code
///
/// It does *not* attempt to type-erase the runtime output (that remains domain-specific).
pub trait DynRecipe {
    fn meta(&self) -> RecipeMeta;
    fn schema(&self) -> SchemaNode;

    /// Returns a JSON-encoded config object suitable for editing.
    fn default_config_json(&self) -> Value;

    /// Exports a Rust snippet based on a JSON-encoded config.
    ///
    /// Implementations should validate and return `None` on invalid configs.
    fn export_rust_from_json(&self, cfg: &Value) -> Option<RustSnippet>;
}

// <FILE>src/registry/dyn_recipe.rs</FILE> - <DESC>Type-erased recipe interface</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
