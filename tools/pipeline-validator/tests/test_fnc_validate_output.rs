// <FILE>tools/pipeline-validator/tests/test_fnc_validate_output.rs</FILE> - <DESC>Tests for output stage validation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Simplified tests using actual recipe files</CLOG>

use pipeline_validator::cli::Args;
use pipeline_validator::stages::validate_output;
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
fn test_validate_output_passes() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_output(&config, &args);

    assert!(result.passed, "Output validation should pass");
    assert_eq!(result.stage, "output");
}

#[test]
fn test_validate_output_reports_buffer_size() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_output(&config, &args);

    let has_buffer_msg = result.messages.iter().any(|m| m.contains("buffer:"));
    assert!(
        has_buffer_msg,
        "Should report buffer size: {:?}",
        result.messages
    );
}

#[test]
fn test_validate_output_reports_rendered_samples() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_output(&config, &args);

    let has_rendered_msg = result
        .messages
        .iter()
        .any(|m| m.contains("rendered:") && m.contains("samples"));
    assert!(
        has_rendered_msg,
        "Should report rendered samples: {:?}",
        result.messages
    );
}

#[test]
fn test_validate_output_renders_multiple_phases() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_output(&config, &args);

    // Default renders 4 samples (entering x2, dwelling, exiting)
    let has_four_samples = result.messages.iter().any(|m| m.contains("4 samples"));
    assert!(
        has_four_samples,
        "Should render 4 phase samples: {:?}",
        result.messages
    );
}

// <FILE>tools/pipeline-validator/tests/stages/functions/test_fnc_validate_output.rs</FILE> - <DESC>Tests for output stage validation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
