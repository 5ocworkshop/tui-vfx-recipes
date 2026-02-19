// <FILE>tools/pipeline-validator/tests/test_fnc_validate_render.rs</FILE> - <DESC>Tests for render stage validation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Simplified tests using actual recipe files</CLOG>

use pipeline_validator::cli::Args;
use pipeline_validator::stages::validate_render;
use std::path::Path;
use tui_vfx_recipes::recipe::load;

fn default_args() -> Args {
    Args {
        paths: vec![],
        verbose: 0,
        format: pipeline_validator::cli::OutputFormat::Text,
        stage: None,
        phase: None,
        sample_t: None,
        dump: false,
        trace: false,
        rules: false,
        rules_file: None,
        strict: false,
        errors_only: false,
        stages: false,
        bench: false,
        iterations: 100,
    }
}

fn load_config(recipe_path: &str) -> tui_vfx_recipes::recipe_schema::config::RaRecipeConfig {
    // Tests run from tools/pipeline-validator, so go up to project root
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = project_root.join(recipe_path);
    let recipe = load(&path, &project_root).expect("load recipe");
    recipe.config().clone()
}

#[test]
fn test_validate_render_passes() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_render(&config, &args);

    assert!(result.passed, "Render validation should pass");
    assert_eq!(result.stage, "render");
}

#[test]
fn test_validate_render_reports_sample_points() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_render(&config, &args);

    let has_sample_msg = result.messages.iter().any(|m| m.contains("sample_t:"));
    assert!(
        has_sample_msg,
        "Should report sample points: {:?}",
        result.messages
    );
}

#[test]
fn test_validate_render_counts_filters() {
    let config = load_config("recipes/debug_recipes/filters/filter_tint.json");
    let args = default_args();

    let result = validate_render(&config, &args);

    // Filter recipe should have filters
    let has_filter_count = result.messages.iter().any(|m| m.contains("filter(s)"));
    assert!(
        has_filter_count,
        "Should count filters: {:?}",
        result.messages
    );
}

#[test]
fn test_validate_render_counts_masks() {
    let config = load_config("recipes/debug_recipes/masks/mask_wipe.json");
    let args = default_args();

    let result = validate_render(&config, &args);

    // Mask recipe should have masks
    let has_mask_count = result.messages.iter().any(|m| m.contains("mask(s)"));
    assert!(has_mask_count, "Should count masks: {:?}", result.messages);
}

// <FILE>tools/pipeline-validator/tests/stages/functions/test_fnc_validate_render.rs</FILE> - <DESC>Tests for render stage validation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
