// <FILE>tools/pipeline-validator/tests/test_fnc_validate_shader.rs</FILE> - <DESC>Tests for shader stage validation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Simplified tests using actual recipe files</CLOG>

use pipeline_validator::cli::Args;
use pipeline_validator::stages::validate_shader;
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
fn test_validate_shader_passes() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_shader(&config, &args);

    assert!(result.passed, "Shader validation should pass");
    assert_eq!(result.stage, "shader");
}

#[test]
fn test_validate_shader_reports_zero_when_none() {
    let config = load_config("recipes/debug_recipes/baseline.json");
    let args = default_args();

    let result = validate_shader(&config, &args);

    let has_zero_shaders = result.messages.iter().any(|m| m.contains("0 configured"));
    assert!(
        has_zero_shaders,
        "Should report 0 shaders: {:?}",
        result.messages
    );
}

#[test]
fn test_validate_shader_detects_spatial_shader() {
    let config = load_config("recipes/debug_recipes/shaders/shader_glisten_band.json");
    let args = default_args();

    let result = validate_shader(&config, &args);

    let has_shader = result
        .messages
        .iter()
        .any(|m| m.contains("configured") && !m.contains("0 configured"));
    assert!(
        has_shader,
        "Should detect spatial shader: {:?}",
        result.messages
    );
}

// <FILE>tools/pipeline-validator/tests/stages/functions/test_fnc_validate_shader.rs</FILE> - <DESC>Tests for shader stage validation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
