// <FILE>src/recipe/mod.rs</FILE> - <DESC>Unified recipe module - single entry point for recipe loading and effect access</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Embedded recipe library extraction</WCTX>
// <CLOG>Added embedded module for hybrid loading primitives</CLOG>

pub mod embedded;
pub mod fnc_from_value; // from_value() function
pub mod fnc_load; // load() function
pub mod fnc_parse; // parse() function
pub mod types; // Core types (Recipe, ShaderRef, etc.) // Hybrid loading for embedded recipes

// Re-exports
pub use embedded::{ConfigLoadError, load_config_hybrid, load_recipe_hybrid};
pub use fnc_from_value::from_value;
pub use fnc_load::load;
pub use fnc_parse::parse;
pub use types::{
    FilterRef, MaskRef, Phase, Recipe, RecipeError, SamplerRef, ShaderRef, StyleLayerRef,
};

// <FILE>src/recipe/mod.rs</FILE> - <DESC>Unified recipe module - single entry point for recipe loading and effect access</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
