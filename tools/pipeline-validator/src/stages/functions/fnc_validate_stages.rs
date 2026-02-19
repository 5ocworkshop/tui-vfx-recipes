// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_stages.rs</FILE> - <DESC>Pipeline stage inspection validation</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>Pipeline stage inspection implementation</WCTX>
// <CLOG>Add style interpolation reporting for non-spatial effects

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::borrow::Cow;
use tui_vfx_compositor::types::{FilterSpec, MaskSpec};
use tui_vfx_geometry::types::SignedRect;
use tui_vfx_recipes::inspector::InspectorContext;
use tui_vfx_recipes::inspector::impls::StageInspector;
use tui_vfx_recipes::prelude::CompositorCtx;
use tui_vfx_recipes::preview::{
    PreviewItem, preview_from_recipe_config, render_preview_item_inspected,
};
use tui_vfx_recipes::recipe_schema::config::RaRecipeConfig;
use tui_vfx_recipes::rendering::RenderPlanItem;
use tui_vfx_recipes::state::AnimationPhase;
use tui_vfx_recipes::theme::Theme;
use tui_vfx_recipes::traits::Animated;

use crate::cli::{Args, Phase};
use crate::stages::StageResult;

/// Validate pipeline stages using the REAL rendering path with inspector hooks.
///
/// This implementation exercises the exact same code path that library users use:
/// 1. Config → PreviewItem (via preview_from_recipe_config)
/// 2. PreviewItem → RenderPlanItem (manually constructed with desired phase/t)
/// 3. RenderPlanItem → render_preview_item_inspected → render_animated_with_appearance_inspected → render_pipeline
///
/// The key difference from manual CompositionOptions construction is that pipeline
/// specs (masks, samplers, filters) come from the PreviewItem's profile, which was
/// built through the real config conversion path.
pub fn validate_stages(config: &RaRecipeConfig, args: &Args) -> StageResult {
    let mut result = StageResult::pass("stages");

    let width = config.layout.width;
    let height = config.layout.height;
    result = result.with_message(format!("area: {}x{}", width, height));

    let sample_points = args.sample_points();
    result = result.with_message(format!("sample_t: {:?}", sample_points));

    // Create PreviewItem using the REAL config conversion path
    // This exercises all the config-to-profile logic that users rely on
    let preview_item = preview_from_recipe_config(config);

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

    let area = Rect::new(0, 0, width, height);
    let mut ctx = CompositorCtx::new();
    let theme = Theme::default();

    for phase in phases {
        for &t in &sample_points {
            // Use two inspectors: one for high-level PipelineInspector hooks (style interpolation)
            // and one for low-level CompositorInspector hooks (sampler, mask, shader, filter)
            let pipeline_inspector = StageInspector::new(width, height);
            let mut compositor_inspector = StageInspector::new(width, height);
            let mut buffer = Buffer::empty(area);

            // Build RenderPlanItem with the desired phase/t
            // Pipeline specs come from PreviewItem (built via real config conversion)
            let plan = build_render_plan(&preview_item, area, phase, t);

            // Create InspectorContext with the pipeline inspector for style interpolation hooks
            let mut inspector_ctx = InspectorContext::with_inspector(Box::new(pipeline_inspector));

            // Render using the REAL path with both inspectors
            render_preview_item_inspected(
                &preview_item,
                &plan,
                &theme,
                area,
                &mut buffer,
                &mut ctx,
                &mut inspector_ctx,
                &mut compositor_inspector,
            );

            // Extract the pipeline inspector to get style interpolation results
            let pipeline_inspector = inspector_ctx.take_inspector().and_then(|boxed| {
                // Downcast to StageInspector to access style_results
                // This is safe because we just created it above
                let ptr = Box::into_raw(boxed);
                // SAFETY: We know this is a StageInspector because we created it
                unsafe { Some(Box::from_raw(ptr as *mut StageInspector)) }
            });

            // Report stage results from both inspectors
            result = report_stage_results(
                result,
                phase,
                t,
                &compositor_inspector,
                pipeline_inspector.as_deref(),
                args.verbose,
            );
        }
    }

    result
}

/// Build a RenderPlanItem for a specific phase and t value.
///
/// Pipeline specs are extracted from the PreviewItem, which was
/// built through the real config conversion path. This ensures we're testing
/// the same specs that would be used in production.
fn build_render_plan(
    item: &PreviewItem,
    area: Rect,
    phase: AnimationPhase,
    t: f64,
) -> RenderPlanItem<'static> {
    let animation = item.animation();
    let anchor = item.anchor();

    // Extract phase-specific pipeline specs from the PreviewItem
    // These come from the real config conversion, not manual extraction
    let (mask, sampler_spec, filter) = match phase {
        AnimationPhase::Entering => (
            item.enter_mask.clone(),
            item.enter_sampler.clone(),
            item.enter_filter.clone(),
        ),
        AnimationPhase::Dwelling => (
            item.dwell_mask.clone(),
            item.dwell_sampler.clone(),
            item.dwell_filter.clone(),
        ),
        AnimationPhase::Exiting => (
            item.exit_mask.clone(),
            item.exit_sampler.clone(),
            item.exit_filter.clone(),
        ),
        AnimationPhase::Finished => (None, None, None),
    };

    // Convert single mask/filter to slices for multi-effect fields
    let masks: Vec<MaskSpec> = mask.into_iter().collect();
    let filters: Vec<FilterSpec> = filter.into_iter().collect();

    #[allow(deprecated)]
    RenderPlanItem {
        id: 0,
        anchor,
        offset_h_percent: item.offset_h_percent,
        offset_v_percent: item.offset_v_percent,
        offset_h_cells: item.offset_h_cells,
        offset_v_cells: item.offset_v_cells,
        offset_h_pixels: item.offset_h_pixels,
        offset_v_pixels: item.offset_v_pixels,
        phase,
        animation,
        dwell_rect: area,
        area,
        signed_area: SignedRect::new(area.x as i32, area.y as i32, area.width, area.height),
        t,
        // Multi-effect fields
        masks: Cow::Owned(masks),
        mask_combine_mode: item.mask_combine_mode,
        filters: Cow::Owned(filters),
        // Legacy single-effect fields
        // mask_spec removed
        sampler_spec,
        // filter_spec removed
        loop_t: Some(t),
    }
}

/// Report stage inspection results to the StageResult.
fn report_stage_results(
    mut result: StageResult,
    phase: AnimationPhase,
    t: f64,
    compositor_inspector: &StageInspector,
    pipeline_inspector: Option<&StageInspector>,
    verbose: u8,
) -> StageResult {
    let phase_str = format!("{:?}", phase);

    // Summary line - compositor results
    let sampler_count = compositor_inspector.sampler_results.len();
    let mask_count = compositor_inspector.mask_results.len();
    let shader_count = compositor_inspector.shader_results.len();
    let filter_count = compositor_inspector.filter_results.len();

    // Style interpolation results from pipeline inspector
    let style_count = pipeline_inspector
        .map(|i| i.style_results.len())
        .unwrap_or(0);

    let summary = format!(
        "{} t={:.2}: sampler={}, mask={}, shader={}, filter={}, style={}",
        phase_str, t, sampler_count, mask_count, shader_count, filter_count, style_count
    );

    if verbose >= 1 {
        result = result.with_message(summary);
    }

    // Detailed stage information at verbose >= 2
    if verbose >= 2 {
        // Sampler stats
        let displaced = compositor_inspector.sampler_displacement_count();
        if sampler_count > 0 {
            result = result.with_detail(format!(
                "  SAMPLER: {}/{} cells displaced",
                displaced, sampler_count
            ));
        }

        // Mask coverage
        if mask_count > 0 {
            let (visible, total) = compositor_inspector.mask_coverage();
            let percent = compositor_inspector.mask_coverage_percent();
            result = result.with_detail(format!(
                "  MASK: {}/{} visible ({:.1}%)",
                visible, total, percent
            ));

            // Visual coverage map at verbose >= 3
            if verbose >= 3 {
                let map = compositor_inspector.visibility_map();
                let coverage_line = render_coverage_bar(&map);
                result = result.with_detail(format!("    coverage: {}", coverage_line));
            }
        }

        // Shader modifications (spatial shaders at compositor level)
        if shader_count > 0 {
            let modified = compositor_inspector.shader_modification_count();
            result = result.with_detail(format!(
                "  SHADER: {}/{} cells modified",
                modified, shader_count
            ));

            // Sample of shader changes at verbose >= 3
            if verbose >= 3 && !compositor_inspector.shader_results.is_empty() {
                let sample = &compositor_inspector.shader_results[0];
                result = result.with_detail(format!(
                    "    sample: ({},{}) {} before={:?} after={:?}",
                    sample.x, sample.y, sample.shader, sample.before, sample.after
                ));
            }
        }

        // Filter modifications
        if filter_count > 0 {
            let modified = compositor_inspector.filter_modification_count();
            result = result.with_detail(format!(
                "  FILTER: {}/{} cells modified",
                modified, filter_count
            ));

            // Sample of filter changes at verbose >= 3
            if verbose >= 3 && !compositor_inspector.filter_results.is_empty() {
                let sample = &compositor_inspector.filter_results[0];
                result = result.with_detail(format!(
                    "    sample: ({},{}) {} fg:{:?}->{:?} bg:{:?}->{:?}",
                    sample.x,
                    sample.y,
                    sample.filter,
                    sample.before_fg,
                    sample.after_fg,
                    sample.before_bg,
                    sample.after_bg
                ));
            }
        }

        // Style interpolation (non-spatial effects like FadeIn, Pulse, Rainbow)
        if let Some(pi) = pipeline_inspector {
            if style_count > 0 {
                let modified = pi.style_modification_count();
                let effects = pi.style_effects_applied();
                result = result.with_detail(format!(
                    "  STYLE: {}/{} modified, effects=[{}]",
                    modified,
                    style_count,
                    effects.join(", ")
                ));

                // Sample of style changes at verbose >= 3
                if verbose >= 3 && !pi.style_results.is_empty() {
                    let sample = &pi.style_results[0];
                    result = result.with_detail(format!(
                        "    sample: {} on {} before={:?} after={:?}",
                        sample.effect, sample.target, sample.before, sample.after
                    ));
                }
            }
        }
    }

    result
}

/// Render a simple ASCII coverage bar from visibility map.
fn render_coverage_bar(map: &[Vec<bool>]) -> String {
    if map.is_empty() {
        return String::from("[empty]");
    }

    // Sample 20 columns across the width
    let height = map.len();
    let width = map.first().map(|r| r.len()).unwrap_or(0);
    if width == 0 || height == 0 {
        return String::from("[empty]");
    }

    let samples = 20.min(width);
    let mut bar = String::with_capacity(samples + 2);
    bar.push('[');

    for i in 0..samples {
        let x = (i * width) / samples;
        // Count visible cells in this column
        let visible_count = map
            .iter()
            .filter(|row| row.get(x).copied().unwrap_or(false))
            .count();
        let ratio = visible_count as f64 / height as f64;

        let char = if ratio >= 0.75 {
            '#'
        } else if ratio >= 0.5 {
            '='
        } else if ratio >= 0.25 {
            '-'
        } else if ratio > 0.0 {
            '.'
        } else {
            ' '
        };
        bar.push(char);
    }

    bar.push(']');
    bar
}

// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_stages.rs</FILE> - <DESC>Pipeline stage inspection validation</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
