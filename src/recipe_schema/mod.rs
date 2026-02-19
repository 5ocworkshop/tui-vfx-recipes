// <FILE>src/v2/mod.rs</FILE> - <DESC>V2 recipe schema module with template inheritance</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Interactive elements system implementation - Phase 3</WCTX>
// <CLOG>MINOR: Added interactions module for interactive element schema</CLOG>

//! V2 recipe schema and parser with template inheritance support.
//!
//! This module provides the V2 configuration schema and parsing functions.
//! V2 recipes support template inheritance via the `extends` field, enabling
//! DRY recipe authoring by inheriting layout, styling, and effects from base templates.
//!
//! # Template Inheritance
//! Recipes can reference template files using the `extends` field:
//! ```json
//! {
//!   "schema_version": 2,
//!   "extends": "themes/computer_base.json",
//!   "message": "Hello, World!"
//! }
//! ```
//!
//! # Modules
//! - `config` - V2 configuration types and enums
//! - `parser` - JSON recipe parsing with template resolution (sync + async)
//! - `functions` - Template resolution utilities
//! - `interactions` - Interactive element schema types

pub mod config;
pub mod functions;
pub mod interactions;
pub mod parser;

pub use config::*;
pub use parser::{json_recipe_dyn, json_recipe_dyn_from_file};

// Re-export template resolution functions for advanced use cases
pub use functions::{
    CircularReferenceError, TemplatePathError, TemplateResolutionError, deep_merge_json,
    resolve_recipe_with_template, resolve_template_path, validate_no_circular_ref,
};

// <FILE>src/v2/mod.rs</FILE> - <DESC>V2 recipe schema module with template inheritance</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
