// <FILE>tools/pipeline-validator/tests/test_fnc_validate_profile.rs</FILE> - <DESC>Tests for profile stage validation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Simplified tests using actual recipe files</CLOG>

use pipeline_validator::cli::Args;
use pipeline_validator::stages::validate_profile;
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
fn test_validate_profile_passes_for_valid_config() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_profile(&config, &args);

    assert!(
        result.passed,
        "Profile validation should pass for valid config"
    );
    assert_eq!(result.stage, "profile");
}

#[test]
fn test_validate_profile_reports_enter_duration() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_profile(&config, &args);

    let has_enter_msg = result
        .messages
        .iter()
        .any(|m| m.contains("enter:") && m.contains("duration="));
    assert!(
        has_enter_msg,
        "Should report enter duration: {:?}",
        result.messages
    );
}

#[test]
fn test_validate_profile_reports_exit_duration() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_profile(&config, &args);

    let has_exit_msg = result
        .messages
        .iter()
        .any(|m| m.contains("exit:") && m.contains("duration="));
    assert!(
        has_exit_msg,
        "Should report exit duration: {:?}",
        result.messages
    );
}

#[test]
fn test_validate_profile_reports_style_layers() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_profile(&config, &args);

    let has_layers_msg = result.messages.iter().any(|m| m.contains("style_layers:"));
    assert!(
        has_layers_msg,
        "Should report style layers: {:?}",
        result.messages
    );
}

// <FILE>tools/pipeline-validator/tests/stages/functions/test_fnc_validate_profile.rs</FILE> - <DESC>Tests for profile stage validation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
