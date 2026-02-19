// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_profile.rs</FILE> - <DESC>Profile stage validation function</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Initial creation with proper OFPF naming</CLOG>

use tui_vfx_recipes::recipe_schema::config::RaRecipeConfig;

use crate::cli::Args;
use crate::stages::StageResult;

/// Validate the profile stage: build AnimationProfile from RaRecipeConfig.
///
/// This stage exercises the library's `to_animation_profile()` method and reports
/// what was extracted. If the library has bugs, they'll surface here as panics
/// or incorrect values.
pub fn validate_profile(config: &RaRecipeConfig, args: &Args) -> StageResult {
    let mut result = StageResult::pass("profile");

    // Exercise the library's profile construction
    let profile = config.to_animation_profile();

    // Report transition specs
    result = result.with_message(format!(
        "enter: duration={}ms, ease={:?}",
        profile.enter.duration_ms, profile.enter.ease
    ));

    result = result.with_message(format!(
        "exit: duration={}ms, ease={:?}",
        profile.exit.duration_ms, profile.exit.ease
    ));

    // Report style layers
    let layers = profile.effective_style_layers();
    result = result.with_message(format!("style_layers: {} layer(s)", layers.len()));

    if args.verbose >= 2 {
        for (i, layer) in layers.iter().enumerate() {
            result = result.with_detail(format!("  [{}] region={:?}", i, layer.region));
            result = result.with_detail(format!(
                "      enter_effect: {}",
                if layer.enter_effect.is_some() {
                    "Some"
                } else {
                    "None"
                }
            ));
            result = result.with_detail(format!(
                "      dwell_effect: {}",
                if layer.dwell_effect.is_some() {
                    "Some"
                } else {
                    "None"
                }
            ));
            result = result.with_detail(format!(
                "      exit_effect: {}",
                if layer.exit_effect.is_some() {
                    "Some"
                } else {
                    "None"
                }
            ));
        }
    }

    // Report loop period if configured
    if let Some(ref period) = profile.loop_period {
        result = result.with_message(format!("loop_period: {:?}", period));
    }

    result
}

// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_profile.rs</FILE> - <DESC>Profile stage validation function</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
