// <FILE>src/recipe/embedded/mod.rs</FILE> - <DESC>Embedded recipe loading with filesystem override</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Embedded recipe library extraction</WCTX>
// <CLOG>Initial module creation for hybrid loading primitives</CLOG>

//! Hybrid loading for embedded recipes and configs.
//!
//! Provides utilities for loading recipes/configs with a "try filesystem, fall back to embedded"
//! strategy. This enables:
//! - Release binaries that work without external files (embedded defaults)
//! - Development workflow with hot-reload from filesystem
//!
//! # Example - Loading a V2 Recipe
//! ```ignore
//! use tui_vfx_recipes::recipe::load_recipe_hybrid;
//! use std::path::Path;
//!
//! const EMBEDDED_JSON: &str = include_str!("../config/my_recipe.json");
//!
//! let recipe = load_recipe_hybrid(
//!     Some(Path::new("config/my_recipe.json")),  // Runtime path (optional)
//!     Path::new("/project"),                      // Project root for templates
//!     EMBEDDED_JSON,                              // Fallback
//!     "my_recipe",                                // Name for error messages
//! )?;
//! ```
//!
//! # Example - Loading a Generic Config
//! ```ignore
//! use tui_vfx_recipes::recipe::load_config_hybrid;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct MyConfig { /* ... */ }
//!
//! const EMBEDDED_JSON: &str = include_str!("../config/my_config.json");
//!
//! let config: MyConfig = load_config_hybrid(
//!     Some(Path::new("config/my_config.json")),
//!     EMBEDDED_JSON,
//!     "my_config",
//! )?;
//! ```

pub mod fnc_load_config_hybrid;
pub mod fnc_load_hybrid;

pub use fnc_load_config_hybrid::{ConfigLoadError, load_config_hybrid};
pub use fnc_load_hybrid::load_recipe_hybrid;

// <FILE>src/recipe/embedded/mod.rs</FILE> - <DESC>Embedded recipe loading with filesystem override</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
