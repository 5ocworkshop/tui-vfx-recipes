// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_shader.rs</FILE> - <DESC>Shader stage validation function</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Region-constrained wipe effect for enter animations</WCTX>
// <CLOG>Add warning for phase durations too short for perceptible spatial effects</CLOG>

use tui_vfx_recipes::recipe_schema::config::RaRecipeConfig;
use tui_vfx_style::models::StyleEffect;

use crate::cli::Args;
use crate::stages::StageResult;

/// Minimum phase duration (ms) for spatial shaders to be perceptible.
/// Effects under this threshold may appear to "pop" rather than animate.
const MIN_PERCEPTIBLE_DURATION_MS: u64 = 100;

/// Validate the shader stage: check for spatial shaders in style layers.
///
/// This stage exercises the library's style layer extraction and reports
/// which layers have spatial shaders configured.
///
/// Also warns when phase durations are too short for spatial effects to be
/// perceptible (< 100ms), which can cause effects to appear truncated or instant.
pub fn validate_shader(config: &RaRecipeConfig, args: &Args) -> StageResult {
    let mut result = StageResult::pass("shader");

    let profile = config.to_animation_profile();
    let layers = profile.effective_style_layers();

    // Get phase durations for warning checks
    let enter_duration_ms = config.pipeline.enter.duration_ms;
    let exit_duration_ms = config.pipeline.exit.duration_ms;

    // Count shaders by phase
    let mut shader_count = 0;
    let mut enter_shader_count = 0;
    let mut exit_shader_count = 0;

    for (i, layer) in layers.iter().enumerate() {
        let enter_has_shader = matches!(&layer.enter_effect, Some(StyleEffect::Spatial { .. }));
        let dwell_has_shader = matches!(&layer.dwell_effect, Some(StyleEffect::Spatial { .. }));
        let exit_has_shader = matches!(&layer.exit_effect, Some(StyleEffect::Spatial { .. }));

        if enter_has_shader {
            enter_shader_count += 1;
        }
        if exit_has_shader {
            exit_shader_count += 1;
        }

        let layer_shader_count =
            enter_has_shader as usize + dwell_has_shader as usize + exit_has_shader as usize;

        if layer_shader_count > 0 {
            shader_count += layer_shader_count;

            if args.verbose >= 2 {
                result = result.with_detail(format!(
                    "Layer {}: region={:?}, shaders: enter={}, dwell={}, exit={}",
                    i, layer.region, enter_has_shader, dwell_has_shader, exit_has_shader
                ));
            }
        }
    }

    result = result.with_message(format!("spatial_shaders: {} configured", shader_count));

    if shader_count == 0 {
        result = result.with_detail("No spatial shaders configured".to_string());
    }

    // Warn if phase durations are too short for perceptible spatial effects
    if enter_shader_count > 0 && enter_duration_ms < MIN_PERCEPTIBLE_DURATION_MS {
        result = result.with_warning(format!(
            "Enter phase has {} spatial shader(s) but duration is only {}ms (< {}ms minimum for perceptible animation)",
            enter_shader_count, enter_duration_ms, MIN_PERCEPTIBLE_DURATION_MS
        ));
    }

    if exit_shader_count > 0 && exit_duration_ms < MIN_PERCEPTIBLE_DURATION_MS {
        result = result.with_warning(format!(
            "Exit phase has {} spatial shader(s) but duration is only {}ms (< {}ms minimum for perceptible animation)",
            exit_shader_count, exit_duration_ms, MIN_PERCEPTIBLE_DURATION_MS
        ));
    }

    result
}

// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_shader.rs</FILE> - <DESC>Shader stage validation function</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
