// <FILE>tools/pipeline-validator/src/stages/functions/fnc_benchmark_stages.rs</FILE> - <DESC>Pipeline performance benchmarking</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline performance debugging</WCTX>
// <CLOG>Initial creation - timing instrumentation for render pipeline</CLOG>

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::borrow::Cow;
use std::time::Instant;
use tui_vfx_compositor::types::{FilterSpec, MaskSpec};
use tui_vfx_geometry::types::SignedRect;
use tui_vfx_recipes::prelude::CompositorCtx;
use tui_vfx_recipes::preview::{PreviewItem, preview_from_recipe_config};
use tui_vfx_recipes::recipe_schema::config::RaRecipeConfig;
use tui_vfx_recipes::rendering::RenderPlanItem;
use tui_vfx_recipes::state::AnimationPhase;
use tui_vfx_recipes::theme::Theme;
use tui_vfx_recipes::traits::Animated;

use crate::cli::{Args, Phase};
use crate::stages::StageResult;

/// Timing results for a single benchmark run.
#[derive(Debug, Clone)]
pub struct TimingResult {
    pub phase: AnimationPhase,
    pub t: f64,
    pub total_ns: u64,
    #[allow(dead_code)]
    pub iterations: usize,
}

/// Aggregated benchmark results.
#[derive(Debug)]
pub struct BenchmarkResults {
    pub timings: Vec<TimingResult>,
    #[allow(dead_code)]
    pub area: (u16, u16),
    pub cells: usize,
    #[allow(dead_code)]
    pub iterations: usize,
}

impl BenchmarkResults {
    /// Average time per render in nanoseconds.
    pub fn avg_ns(&self) -> f64 {
        if self.timings.is_empty() {
            return 0.0;
        }
        let total: u64 = self.timings.iter().map(|t| t.total_ns).sum();
        total as f64 / self.timings.len() as f64
    }

    /// Frames per second based on average render time.
    pub fn fps(&self) -> f64 {
        let avg_ns = self.avg_ns();
        if avg_ns <= 0.0 {
            return 0.0;
        }
        1_000_000_000.0 / avg_ns
    }

    /// Time per cell in nanoseconds.
    pub fn ns_per_cell(&self) -> f64 {
        if self.cells == 0 {
            return 0.0;
        }
        self.avg_ns() / self.cells as f64
    }
}

/// Run performance benchmark on the render pipeline.
///
/// Measures:
/// - Total render time per frame
/// - Time breakdown by phase (entering, dwelling, exiting)
/// - Time at different t values
pub fn benchmark_stages(config: &RaRecipeConfig, args: &Args) -> StageResult {
    let mut result = StageResult::pass("benchmark");

    let width = config.layout.width;
    let height = config.layout.height;
    let cells = (width as usize) * (height as usize);
    let iterations = args.iterations;

    result = result.with_message(format!("area: {}x{} ({} cells)", width, height, cells));
    result = result.with_message(format!("iterations: {}", iterations));

    // Create PreviewItem using the REAL config conversion path
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

    let sample_points = args.sample_points();
    let area = Rect::new(0, 0, width, height);

    let mut timings = Vec::new();

    // Warm-up run
    {
        let mut ctx = CompositorCtx::new();
        let theme = Theme::default();
        let mut buffer = Buffer::empty(area);
        let plan = build_render_plan(&preview_item, area, AnimationPhase::Dwelling, 0.5);
        tui_vfx_recipes::preview::render_preview_item(
            &preview_item,
            &plan,
            &theme,
            area,
            &mut buffer,
            &mut ctx,
        );
    }

    // Benchmark each phase/t combination
    for phase in &phases {
        for &t in &sample_points {
            let timing = benchmark_single(&preview_item, area, *phase, t, iterations);
            timings.push(timing);
        }
    }

    let bench_results = BenchmarkResults {
        timings: timings.clone(),
        area: (width, height),
        cells,
        iterations,
    };

    // Report results
    result = result.with_message(format!(
        "avg: {:.2} µs/frame ({:.1} FPS)",
        bench_results.avg_ns() / 1000.0,
        bench_results.fps()
    ));
    result = result.with_message(format!(
        "per-cell: {:.2} ns/cell",
        bench_results.ns_per_cell()
    ));

    // Detailed breakdown by phase
    for phase in &phases {
        let phase_timings: Vec<_> = timings.iter().filter(|t| t.phase == *phase).collect();

        if phase_timings.is_empty() {
            continue;
        }

        let phase_avg: f64 = phase_timings.iter().map(|t| t.total_ns as f64).sum::<f64>()
            / phase_timings.len() as f64;

        result = result.with_detail(format!(
            "{:?}: {:.2} µs/frame ({:.1} FPS)",
            phase,
            phase_avg / 1000.0,
            1_000_000_000.0 / phase_avg
        ));

        // Show individual t values at verbose >= 2
        if args.verbose >= 2 {
            for timing in phase_timings {
                let fps = 1_000_000_000.0 / timing.total_ns as f64;
                result = result.with_detail(format!(
                    "  t={:.2}: {:.2} µs ({:.1} FPS)",
                    timing.t,
                    timing.total_ns as f64 / 1000.0,
                    fps
                ));
            }
        }
    }

    // Performance analysis
    let avg_fps = bench_results.fps();
    if avg_fps < 60.0 {
        result = result.with_message(format!("WARNING: Below 60 FPS target ({:.1} FPS)", avg_fps));

        // Identify slow phases
        for phase in &phases {
            let phase_timings: Vec<_> = timings.iter().filter(|t| t.phase == *phase).collect();

            if phase_timings.is_empty() {
                continue;
            }

            let phase_avg: f64 = phase_timings.iter().map(|t| t.total_ns as f64).sum::<f64>()
                / phase_timings.len() as f64;
            let phase_fps = 1_000_000_000.0 / phase_avg;

            if phase_fps < 60.0 {
                result = result
                    .with_message(format!("  SLOW: {:?} phase ({:.1} FPS)", phase, phase_fps));
            }
        }
    }

    result
}

/// Benchmark a single phase/t combination.
fn benchmark_single(
    item: &PreviewItem,
    area: Rect,
    phase: AnimationPhase,
    t: f64,
    iterations: usize,
) -> TimingResult {
    let theme = Theme::default();
    let plan = build_render_plan(item, area, phase, t);

    // Pre-allocate buffer outside timing loop
    let mut buffer = Buffer::empty(area);
    let mut ctx = CompositorCtx::new();

    let start = Instant::now();
    for _ in 0..iterations {
        // Reset buffer to empty state
        buffer.reset();
        // Run the full render pipeline
        tui_vfx_recipes::preview::render_preview_item(
            item,
            &plan,
            &theme,
            area,
            &mut buffer,
            &mut ctx,
        );
    }
    let elapsed = start.elapsed();

    TimingResult {
        phase,
        t,
        total_ns: elapsed.as_nanos() as u64 / iterations as u64,
        iterations,
    }
}

/// Build a RenderPlanItem for a specific phase and t value.
fn build_render_plan(
    item: &PreviewItem,
    area: Rect,
    phase: AnimationPhase,
    t: f64,
) -> RenderPlanItem<'static> {
    let animation = item.animation();
    let anchor = item.anchor();

    // Extract phase-specific pipeline specs from the PreviewItem
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

// <FILE>tools/pipeline-validator/src/stages/functions/fnc_benchmark_stages.rs</FILE> - <DESC>Pipeline performance benchmarking</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
