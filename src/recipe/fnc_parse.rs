// <FILE>src/recipe/fnc_parse.rs</FILE> - <DESC>Parse recipe from JSON string</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Unified pipeline architecture - Phase 1</WCTX>
// <CLOG>Implement parse function for creating Recipe from JSON string</CLOG>

use crate::recipe::{Recipe, RecipeError, from_value};
use serde_json::Value;

/// Parse a recipe from a JSON string.
///
/// Use this when you have recipe JSON in memory (no template resolution).
///
/// # Arguments
/// * `json` - The JSON string to parse
///
/// # Returns
/// A `Recipe` instance ready for use
///
/// # Errors
/// Returns `RecipeError::ParseError` if:
/// - JSON string is malformed
/// - `schema_version` is missing or < 2
/// - JSON structure doesn't match V2 recipe schema
/// - Required fields are missing or invalid
///
/// # Example
/// ```ignore
/// use tui_vfx_recipes::recipe::parse;
///
/// let json = r#"{"schema_version": 2, "id": "test", ...}"#;
/// let recipe = parse(json)?;
/// ```
pub fn parse(json: &str) -> Result<Recipe, RecipeError> {
    // Parse JSON string to Value
    let value: Value = serde_json::from_str(json).map_err(|e| {
        let snippet = if json.len() > 200 {
            format!("{}...", &json[..200])
        } else {
            json.to_string()
        };
        RecipeError::ParseError(format!("Failed to parse JSON: {}. Input: {}", e, snippet))
    })?;

    // Use from_value to create Recipe
    from_value(value)
}

// <FILE>src/recipe/fnc_parse.rs</FILE> - <DESC>Parse recipe from JSON string</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
