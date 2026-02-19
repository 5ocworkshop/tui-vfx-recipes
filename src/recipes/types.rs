// <FILE>src/recipes/types.rs</FILE> - <DESC>Recipe type definitions</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>WG6: Recipe String Ownership Refactor</WCTX>
// <CLOG>BREAKING: Changed RecipeId and RecipeMeta to use Arc<str> instead of &'static str to eliminate Box::leak memory leaks</CLOG>

use std::sync::Arc;

/// Stable identifier for a recipe/preset.
///
/// Changed from `&'static str` to `Arc<str>` to support dynamic recipe loading
/// without memory leaks while maintaining thread-safety and cheap cloning.
pub type RecipeId = Arc<str>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeMeta {
    pub id: RecipeId,
    pub title: Arc<str>,
    pub description: Arc<str>,
}

impl RecipeMeta {
    /// Create RecipeMeta from static strings (embedded recipes).
    ///
    /// This wraps static strings in Arc, which is cheap (no allocation,
    /// Arc points to static storage).
    pub fn from_static(id: &'static str, title: &'static str, description: &'static str) -> Self {
        Self {
            id: Arc::from(id),
            title: Arc::from(title),
            description: Arc::from(description),
        }
    }

    /// Create RecipeMeta from owned strings (dynamic recipes).
    ///
    /// Converts owned Strings into Arc<str> for shared ownership.
    pub fn from_owned(id: String, title: String, description: String) -> Self {
        Self {
            id: Arc::from(id),
            title: Arc::from(title),
            description: Arc::from(description),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustSnippet {
    /// One `use ...;` statement per line.
    pub uses: Vec<String>,
    /// The snippet body (typically ends with `;`).
    pub body: String,
}

// <FILE>src/recipes/types.rs</FILE> - <DESC>Recipe type definitions</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
