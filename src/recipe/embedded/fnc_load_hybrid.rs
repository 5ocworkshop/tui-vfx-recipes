// <FILE>src/recipe/embedded/fnc_load_hybrid.rs</FILE> - <DESC>Hybrid recipe loading: filesystem with embedded fallback</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Embedded recipe library extraction</WCTX>
// <CLOG>Initial implementation of load_recipe_hybrid function</CLOG>

//! Load V2 Recipe with hybrid strategy: try filesystem, fall back to embedded.

use crate::recipe::{Recipe, RecipeError, load as load_from_file, parse as parse_embedded};
use std::path::Path;

/// Load a V2 recipe with hybrid strategy.
///
/// Strategy:
/// 1. If `runtime_path` is Some and the file exists, load from filesystem (with template resolution)
/// 2. Otherwise, parse from `embedded_json` (no template resolution - must be fully resolved)
///
/// # Arguments
/// * `runtime_path` - Optional path to check for runtime override file
/// * `project_root` - Project root for template resolution (only used if loading from filesystem)
/// * `embedded_json` - The embedded JSON string to use as fallback
/// * `name` - Name for error messages (e.g., "status_toast")
///
/// # Returns
/// The loaded Recipe
///
/// # Errors
/// Returns `RecipeError` if:
/// - Runtime file exists but fails to parse, AND embedded also fails
/// - Embedded JSON fails to parse (indicates compile-time validation missed an error)
///
/// # Example
/// ```ignore
/// use tui_vfx_recipes::recipe::load_recipe_hybrid;
/// use std::path::Path;
///
/// const EMBEDDED: &str = include_str!("../recipes/my_recipe.json");
///
/// let recipe = load_recipe_hybrid(
///     Some(Path::new("config/recipes/my_recipe.json")),
///     Path::new("/project"),
///     EMBEDDED,
///     "my_recipe",
/// )?;
/// ```
pub fn load_recipe_hybrid(
    runtime_path: Option<&Path>,
    project_root: &Path,
    embedded_json: &str,
    name: &str,
) -> Result<Recipe, RecipeError> {
    // Try runtime file first (if provided and exists)
    if let Some(path) = runtime_path {
        if path.exists() {
            match load_from_file(path, project_root) {
                Ok(recipe) => {
                    #[cfg(debug_assertions)]
                    eprintln!("[embedded] Loaded {} from filesystem", name);
                    return Ok(recipe);
                }
                Err(e) => {
                    // Log warning but fall through to embedded
                    eprintln!(
                        "[embedded] Warning: Failed to load {} from {:?}, using embedded: {:?}",
                        name, path, e
                    );
                }
            }
        }
    }

    // Fall back to embedded
    parse_embedded(embedded_json)
        .map_err(|e| RecipeError::ParseError(format!("Failed to parse embedded {}: {:?}", name, e)))
}

// <FILE>src/recipe/embedded/fnc_load_hybrid.rs</FILE> - <DESC>Hybrid recipe loading: filesystem with embedded fallback</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
