// <FILE>src/v2/functions/mod.rs</FILE> - <DESC>Template resolution functions module</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Implementing Recipe Template Inheritance per PRD v1.1.0</WCTX>
// <CLOG>Initial creation - Module orchestrator for template resolution functions</CLOG>

//! Template resolution functions for V2 recipe inheritance.
//!
//! This module provides the core functionality for resolving template references
//! in V2 recipes via the `extends` field. Templates enable DRY recipe authoring
//! by allowing recipes to inherit layout, styling, effects, and timing from base
//! templates while overriding only specific fields.
//!
//! # Security
//! The path resolution function enforces strict security boundaries to prevent
//! path traversal attacks. All template files must reside within the project root.
//!
//! # Architecture
//! - `fnc_resolve_recipe_template` - Main orchestrator (recursive)
//! - `fnc_deep_merge_json` - JSON-level merge (preserves user intent vs serde defaults)
//! - `fnc_resolve_template_path` - Secure path resolution with validation
//! - `fnc_validate_template_refs` - Circular reference detection

mod fnc_deep_merge_json;
mod fnc_resolve_recipe_template;
mod fnc_resolve_template_path;
mod fnc_validate_template_refs;

// Re-export functions
pub use fnc_deep_merge_json::deep_merge_json;
pub use fnc_resolve_recipe_template::{TemplateResolutionError, resolve_recipe_with_template};
pub use fnc_resolve_template_path::{TemplatePathError, resolve_template_path};
pub use fnc_validate_template_refs::{CircularReferenceError, validate_no_circular_ref};

// <FILE>src/v2/functions/mod.rs</FILE> - <DESC>Template resolution functions module</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
