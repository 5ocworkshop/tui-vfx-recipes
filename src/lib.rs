// <FILE>src/lib.rs</FILE> - <DESC>Library entry point for tui-vfx-recipes</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Add prelude module for single-import convenience</WCTX>
// <CLOG>Add prelude module re-exporting common types from all dependencies</CLOG>

//! # TUI VFX Recipes
//!
//! JSON recipe loading, parsing, and validation for the tui-vfx ecosystem.
//!
//! This crate provides:
//! - JSON schema definitions for visual effect recipes
//! - Recipe parsing and validation
//! - Template inheritance and resolution
//! - Recipe registry for managing collections of recipes
//! - Preview and animation management
//! - Ratatui buffer integration via Grid adapter
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use tui_vfx_recipes::prelude::*;
//!
//! // Load a recipe
//! let recipe = load(Path::new("recipes/my_effect.json"), Path::new("."))?;
//!
//! // Create a preview manager
//! let mut manager = PreviewManager::new();
//! let item = preview_from_recipe_config(recipe.config());
//! manager.add(item, Instant::now());
//!
//! // Render in your loop
//! manager.tick(now);
//! manager.render(frame_area, &mut buffer, now);
//! ```
//!
//! ## Features
//!
//! - `async` - Enable async recipe loading with tokio
//!
//! ## Note
//!
//! This crate is under active development. Some imports may need adjustment
//! as the tui-vfx library stabilizes.

// Prelude for convenient single-import access
pub mod prelude;

// Application-level types for recipe system
pub mod types;

// Interaction state types
pub mod interactions;

// Pipeline inspection infrastructure
#[macro_use]
pub mod inspector;

// Animation state management
pub mod state;

// Rendering types and functions
pub mod rendering;

// Preview functionality
pub mod preview;

// Theme and appearance
pub mod theme;

// Animated trait
pub mod traits;

// Animation manager
pub mod manager;

// Ratatui compatibility
pub mod compat;

// Recipe schema definitions
pub mod recipe_schema;

// Recipe loading and parsing
pub mod recipe;

// Built-in recipe definitions
pub mod recipes;

// Recipe registry
pub mod registry;

// Re-exports for convenience
pub use inspector::InspectorContext;
pub use recipe::{Recipe, RecipeError, load, parse};
pub use registry::cls_recipe_registry::RecipeRegistry;
pub use state::AnimationPhase;

// <FILE>src/lib.rs</FILE> - <DESC>Library entry point for tui-vfx-recipes</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
