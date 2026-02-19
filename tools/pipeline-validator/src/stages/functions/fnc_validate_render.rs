// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_render.rs</FILE> - <DESC>Render stage validation function</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Initial creation with proper OFPF naming</CLOG>

use tui_vfx_compositor::types::SamplerSpec;
use tui_vfx_recipes::recipe_schema::config::RaRecipeConfig;
use tui_vfx_recipes::state::AnimationPhase;

use crate::cli::{Args, Phase};
use crate::stages::StageResult;

/// Validate the render stage: examine pipeline configuration for each phase.
///
/// This stage reports what pipeline effects (masks, filters, samplers) are
/// configured for each phase. It exercises the library's V2 config deserialization.
pub fn validate_render(config: &RaRecipeConfig, args: &Args) -> StageResult {
    let mut result = StageResult::pass("render");

    let sample_points = args.sample_points();
    result = result.with_message(format!("sample_t: {:?}", sample_points));

    // Get phases to test
    let phases = match args.phase {
        Some(Phase::Entering) => vec![AnimationPhase::Entering],
        Some(Phase::Dwelling) => vec![AnimationPhase::Dwelling],
        Some(Phase::Exiting) => vec![AnimationPhase::Exiting],
        None => vec![
            AnimationPhase::Entering,
            AnimationPhase::Dwelling,
            AnimationPhase::Exiting,
        ],
    };

    // Extract pipeline configuration
    let pipeline = &config.pipeline;

    for phase in phases {
        if args.verbose >= 2 {
            result = result.with_detail(format!("Phase: {:?}", phase));
        }

        // Report phase-specific configuration
        let (mask_count, filter_count, sampler_active) = match phase {
            AnimationPhase::Entering => (
                pipeline.mask.enter.len(),
                pipeline.filter.enter.len(),
                !matches!(pipeline.sampler.enter, SamplerSpec::None),
            ),
            AnimationPhase::Dwelling => (
                pipeline.mask.dwell.len(),
                pipeline.filter.dwell.len(),
                !matches!(pipeline.sampler.dwell, SamplerSpec::None),
            ),
            AnimationPhase::Exiting => (
                pipeline.mask.exit.len(),
                pipeline.filter.exit.len(),
                !matches!(pipeline.sampler.exit, SamplerSpec::None),
            ),
            AnimationPhase::Finished => (0, 0, false),
        };

        if args.verbose >= 2 {
            result = result.with_detail(format!(
                "  masks={}, filters={}, sampler={}",
                mask_count,
                filter_count,
                if sampler_active { "active" } else { "none" }
            ));
        }
    }

    // Summary counts
    let total_masks =
        pipeline.mask.enter.len() + pipeline.mask.dwell.len() + pipeline.mask.exit.len();
    let total_filters =
        pipeline.filter.enter.len() + pipeline.filter.dwell.len() + pipeline.filter.exit.len();

    result = result.with_message(format!(
        "pipeline: {} mask(s), {} filter(s)",
        total_masks, total_filters
    ));

    result
}

// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_render.rs</FILE> - <DESC>Render stage validation function</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
