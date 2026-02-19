// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_output.rs</FILE> - <DESC>Output stage validation function</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Initial creation with proper OFPF naming, helpers extracted</CLOG>

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::time::{Duration, Instant};
use tui_vfx_recipes::inspector::InspectorContext;
use tui_vfx_recipes::inspector::impls::{TraceInspector, TraceVerbosity, ValidationInspector};
use tui_vfx_recipes::preview::{PreviewManager, preview_from_recipe_config};
use tui_vfx_recipes::recipe_schema::config::RaRecipeConfig;
use tui_vfx_recipes::state::AnimationPhase;
use tui_vfx_recipes::traits::Animated;

use super::fnc_count_buffer_cells::count_buffer_cells;
use super::fnc_sample_buffer_cells::sample_buffer_cells_at;
use crate::cli::{Args, Phase};
use crate::stages::StageResult;

/// Validate the output stage using the library's animation/preview infrastructure.
///
/// This stage exercises the full library code path:
/// - `preview_from_recipe_config` to build PreviewItem from config
/// - `PreviewManager` to manage lifecycle, timing, and rendering
/// - Actual rendering to a buffer
///
/// If the library has bugs, they'll surface here as panics, incorrect output,
/// or rendering failures.
pub fn validate_output(config: &RaRecipeConfig, args: &Args) -> StageResult {
    let mut result = StageResult::pass("output");

    // Build profile using the inspector-enabled method for tracing
    let mut inspector_ctx = if args.trace {
        let trace = TraceInspector::new().with_verbosity(TraceVerbosity::Verbose);
        InspectorContext::with_inspector(Box::new(trace))
    } else {
        let validator = ValidationInspector::new();
        InspectorContext::with_inspector(Box::new(validator))
    };

    // Exercise library's inspector-enabled profile construction
    let _profile = config.to_animation_profile_with_inspector(&mut inspector_ctx);

    // Build preview item using library's preview_from_recipe_config
    let preview_item = preview_from_recipe_config(config);

    // Get dimensions
    let width = config.layout.width;
    let height = config.layout.height;
    result = result.with_message(format!("buffer: {}x{}", width, height));

    // Create PreviewManager (wraps AnimationManager with rendering)
    let mut manager = PreviewManager::new();
    let now = Instant::now();
    manager.add(preview_item.clone(), now);

    // Debug: Check what was added
    if args.verbose >= 3 {
        for state in manager.manager_mut().states() {
            result = result.with_detail(format!(
                "  added item: phase={:?}, size={}x{}, anchor={:?}",
                state.phase,
                state.item.width(),
                state.item.height(),
                Animated::anchor(&state.item),
            ));
        }
        // Debug by_anchor
        for (anchor, ids) in manager.manager_mut().by_anchor() {
            result = result.with_detail(format!("  by_anchor[{:?}]: {:?}", anchor, ids));
        }
    }

    // Simulate animation through phases
    let phases = build_phase_samples(args, now);

    // Use larger buffer to accommodate centered widgets - they need space around them
    // Center anchor places widget in middle of frame, so frame must be larger than widget
    let frame_width = width.max(80);
    let frame_height = height.max(24);
    let area = Rect::new(0, 0, frame_width, frame_height);
    let mut buffer = Buffer::empty(area);
    let mut phases_rendered = 0;

    for (expected_phase, offset) in &phases {
        let render_time = now + *offset;
        manager.tick(render_time);

        // Debug: check render plan
        let plan = manager.render_plan(area, render_time);
        if args.verbose >= 3 {
            result = result.with_detail(format!(
                "  {:?} +{:?}: render_plan has {} items",
                expected_phase,
                offset,
                plan.len()
            ));
            for item in &plan {
                result = result.with_detail(format!(
                    "    item {}: area=({},{} {}x{}), phase={:?}, t={:.2}",
                    item.id,
                    item.area.x,
                    item.area.y,
                    item.area.width,
                    item.area.height,
                    item.phase,
                    item.t
                ));
            }
        }

        // Clear buffer and render using PreviewManager's full pipeline
        buffer = Buffer::empty(area);
        manager.render(area, &mut buffer, render_time);
        phases_rendered += 1;

        if args.verbose >= 2 {
            let cells_with_content = count_buffer_cells(&buffer);
            result = result.with_detail(format!(
                "  {:?} +{:?}: {}/{} cells have content",
                expected_phase,
                offset,
                cells_with_content,
                (width as usize) * (height as usize)
            ));
        }
    }

    result = result.with_message(format!("rendered: {} samples", phases_rendered));

    // Sample cells in verbose mode - find where the widget actually rendered
    if args.verbose >= 2 {
        // For centered widgets, calculate where they render in the larger frame
        // Center position = (frame_width/2 - widget_width/2, frame_height/2 - widget_height/2)
        let widget_x = (frame_width - width) / 2;
        let widget_y = (frame_height - height) / 2;
        result = result.with_detail(format!(
            "Widget renders at ({}, {}) in {}x{} frame",
            widget_x, widget_y, frame_width, frame_height
        ));
        // Sample from the widget's actual position
        result = sample_buffer_cells_at(&buffer, widget_x, widget_y, width, height, result);
    }

    result
}

/// Build phase sample points based on CLI args.
fn build_phase_samples(args: &Args, now: Instant) -> Vec<(AnimationPhase, Duration)> {
    let _ = now; // Available for future use
    match args.phase {
        Some(Phase::Entering) => vec![(AnimationPhase::Entering, Duration::from_millis(0))],
        Some(Phase::Dwelling) => vec![(AnimationPhase::Dwelling, Duration::from_millis(600))],
        Some(Phase::Exiting) => vec![(AnimationPhase::Exiting, Duration::from_millis(5000))],
        None => vec![
            (AnimationPhase::Entering, Duration::from_millis(0)),
            (AnimationPhase::Entering, Duration::from_millis(250)),
            (AnimationPhase::Dwelling, Duration::from_millis(600)),
            (AnimationPhase::Exiting, Duration::from_millis(5000)),
        ],
    }
}

// <FILE>tools/pipeline-validator/src/stages/functions/fnc_validate_output.rs</FILE> - <DESC>Output stage validation function</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
