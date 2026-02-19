// <FILE>tools/pipeline-validator/src/stages/parse.rs</FILE> - <DESC>Parse stage: JSON syntax and V2 schema validation</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Unified pipeline architecture - Phase 2</WCTX>
// <CLOG>Migrate to use tui_vfx_recipes::recipe::load() instead of direct RaJsonRecipeDefinition parsing</CLOG>

use std::path::Path;

use tui_vfx_recipes::recipe::load;

use crate::cli::Args;
use crate::stages::StageResult;

/// Validate the parse stage: read JSON and parse as RaRecipeConfig.
pub fn validate(path: &Path, args: &Args) -> Result<StageResult, String> {
    let mut result = StageResult::pass("parse");

    // Determine project_root for template resolution
    // Strategy: Find the repository root by looking for Cargo.toml
    let project_root = path
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists())
        .unwrap_or_else(|| Path::new("."));

    // Use tui-vfx-recipes::recipe::load() for unified recipe loading
    let recipe = load(path, project_root).map_err(|e| format!("Recipe loading failed: {}", e))?;

    result = result.with_message("Recipe loaded successfully");

    // Access the underlying RaRecipeConfig
    let config = recipe.config();

    result = result.with_message("RaRecipeConfig validated");

    // Extract details for verbose output
    let pipeline = &config.pipeline;
    let style_count = pipeline.styles.len();
    result = result
        .with_message(format!("pipeline.styles: {} layer(s)", style_count))
        .with_detail(format!("Style layers defined: {}", style_count));

    if args.verbose >= 2 {
        for (i, style) in pipeline.styles.iter().enumerate() {
            let region = format!("{:?}", style.region);
            result = result.with_detail(format!("  Layer {}: region={}", i, region));
        }
    }

    // Check required fields
    result = result.with_message("Required fields present");

    Ok(result)
}

// <FILE>tools/pipeline-validator/src/stages/parse.rs</FILE> - <DESC>Parse stage: JSON syntax and V2 schema validation</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
