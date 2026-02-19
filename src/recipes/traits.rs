// <FILE>src/recipes/traits.rs</FILE> - <DESC>Recipe trait interfaces</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF normalization</WCTX>
// <CLOG>Moved Recipe/RecipeExport into a dedicated traits file</CLOG>

use tui_vfx_core::SchemaNode;

use super::types::{RecipeMeta, RustSnippet};

/// A reusable preset (recipe) that can be configured and exported.
///
/// - `Config` should be ConfigSchema-friendly.
/// - `Output` is typically a domain object (e.g. a `Notification`), or a domain configuration bundle.
pub trait Recipe {
    type Config: Clone + Default;
    type Output;

    fn meta(&self) -> RecipeMeta;

    fn schema() -> SchemaNode
    where
        Self::Config: tui_vfx_core::ConfigSchema,
    {
        <Self::Config as tui_vfx_core::ConfigSchema>::schema()
    }

    fn build(&self, cfg: &Self::Config) -> Self::Output;
}

/// Optional trait for recipes that can export a Rust builder snippet.
pub trait RecipeExport: Recipe {
    fn export_rust(&self, cfg: &Self::Config) -> RustSnippet;
}

// <FILE>src/recipes/traits.rs</FILE> - <DESC>Recipe trait interfaces</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
