// <FILE>src/recipe/fnc_from_value.rs</FILE> - <DESC>Create recipe from serde_json::Value</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Unified pipeline architecture - Phase 1</WCTX>
// <CLOG>Implement from_value function for creating Recipe from serde_json::Value</CLOG>

use crate::recipe::{Recipe, RecipeError};
use crate::recipe_schema::parser::RaJsonRecipeDefinition;
use serde_json::Value;

/// Create a recipe from a serde_json::Value.
///
/// Use this when you already have parsed JSON (no file I/O, no template resolution).
///
/// # Arguments
/// * `value` - The JSON Value to convert
///
/// # Returns
/// A `Recipe` instance ready for use
///
/// # Errors
/// Returns `RecipeError::ParseError` if:
/// - `schema_version` is missing or < 2
/// - JSON structure doesn't match V2 recipe schema
/// - Required fields are missing or invalid
///
/// # Example
/// ```ignore
/// use serde_json::Value;
/// use tui_vfx_recipes::recipe::from_value;
///
/// let value: Value = serde_json::from_str(json_str)?;
/// let recipe = from_value(value)?;
/// ```
pub fn from_value(value: Value) -> Result<Recipe, RecipeError> {
    // Validate schema version
    let schema_version = value
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| {
            RecipeError::ParseError(
                "Missing or invalid 'schema_version' field. Recipes require schema_version >= 1"
                    .to_string(),
            )
        })?;

    if schema_version < 1 {
        return Err(RecipeError::ParseError(format!(
            "Invalid schema_version={}. Recipes require schema_version >= 1.",
            schema_version
        )));
    }

    // Parse to RaJsonRecipeDefinition
    let definition: RaJsonRecipeDefinition = serde_json::from_value(value)
        .map_err(|e| RecipeError::ParseError(format!("Failed to parse V2 recipe: {}", e)))?;

    // Create Recipe
    Ok(Recipe::new(definition))
}

// <FILE>src/recipe/fnc_from_value.rs</FILE> - <DESC>Create recipe from serde_json::Value</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
