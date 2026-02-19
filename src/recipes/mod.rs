// <FILE>src/recipes/mod.rs</FILE> - <DESC>Recipes/presets module root</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF normalization</WCTX>
// <CLOG>Split recipe interfaces into types + traits</CLOG>

pub mod traits;
pub mod types;

pub use traits::{Recipe, RecipeExport};
pub use types::{RecipeId, RecipeMeta, RustSnippet};

// <FILE>src/recipes/mod.rs</FILE> - <DESC>Recipes/presets module root</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
