// <FILE>src/recipe/fnc_load.rs</FILE> - <DESC>Load recipe from file path with template resolution</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Unified pipeline architecture - Phase 1</WCTX>
// <CLOG>Implement load function for loading Recipe from file path with template resolution</CLOG>

use crate::recipe::{Recipe, RecipeError, from_value};
use crate::recipe_schema::functions::resolve_recipe_with_template;
use std::collections::HashSet;
use std::path::Path;

/// Load a recipe from a JSON file with template resolution.
///
/// This is THE entry point for loading recipes. All consumers should use this.
///
/// # Arguments
/// * `path` - Path to the JSON recipe file
/// * `project_root` - Project root for template resolution (templates must be within)
///
/// # Returns
/// A `Recipe` instance ready for use
///
/// # Errors
/// Returns error if:
/// - Recipe file cannot be read (`RecipeError::IoError`)
/// - JSON is malformed (`RecipeError::ParseError`)
/// - Template resolution fails (`RecipeError::TemplateError`)
/// - Recipe schema is invalid (`RecipeError::ParseError`)
///
/// # Example
/// ```ignore
/// use std::path::Path;
/// use tui_vfx_recipes::recipe::load;
///
/// let recipe = load(
///     Path::new("recipes/my_notification.json"),
///     Path::new("/project")
/// )?;
/// ```
pub fn load(path: &Path, project_root: &Path) -> Result<Recipe, RecipeError> {
    // Resolve templates recursively
    let mut visited = HashSet::new();
    let resolved_json =
        resolve_recipe_with_template(path, project_root, &mut visited, 0).map_err(|e| {
            // Map TemplateResolutionError to RecipeError
            match e {
                crate::recipe_schema::functions::TemplateResolutionError::IoError {
                    path,
                    source,
                } => {
                    RecipeError::IoError(format!("Failed to read recipe at {:?}: {}", path, source))
                }
                crate::recipe_schema::functions::TemplateResolutionError::CircularReference(
                    err,
                ) => RecipeError::TemplateError(format!("Circular template reference: {}", err)),
                crate::recipe_schema::functions::TemplateResolutionError::PathError(err) => {
                    RecipeError::TemplateError(format!("Template path error: {}", err))
                }
                crate::recipe_schema::functions::TemplateResolutionError::MaxDepthExceeded => {
                    RecipeError::TemplateError(
                        "Template inheritance depth exceeded maximum".to_string(),
                    )
                }
                crate::recipe_schema::functions::TemplateResolutionError::InvalidJson {
                    path,
                    source,
                } => RecipeError::ParseError(format!("Invalid JSON in {:?}: {}", path, source)),
            }
        })?;

    // Use from_value to create Recipe
    from_value(resolved_json)
}

// <FILE>src/recipe/fnc_load.rs</FILE> - <DESC>Load recipe from file path with template resolution</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
