// <FILE>src/inspector/impls/cls_stage_inspector.rs</FILE> - <DESC>StageInspector for capturing all pipeline stage results</DESC>
// <VERS>VERSION: 1.4.0</VERS>
// <WCTX>TUI VFX recipes extraction - dual type support for both inspector traits</WCTX>
// <CLOG>Use ratatui types for PipelineInspector, convert for CompositorInspector

use crate::compat::vfx_style_to_ratatui;
use crate::inspector::PipelineInspector;
use crate::state::AnimationPhase;
use ratatui::buffer::Cell as RatatuiCell;
use ratatui::style::{Color as RatatuiColor, Style as RatatuiStyle};
use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_types::{Cell as VfxCell, Style as VfxStyle};

/// Result of sampler coordinate transformation.
#[derive(Debug, Clone, PartialEq)]
pub struct SamplerResult {
    pub dest_x: u16,
    pub dest_y: u16,
    pub src_x: Option<u16>,
    pub src_y: Option<u16>,
    pub sampler: String,
}

/// Result of mask visibility check.
#[derive(Debug, Clone, PartialEq)]
pub struct MaskResult {
    pub x: u16,
    pub y: u16,
    pub visible: bool,
    pub mask: String,
}

/// Result of filter application.
#[derive(Debug, Clone, PartialEq)]
pub struct FilterResult {
    pub x: u16,
    pub y: u16,
    pub before_fg: RatatuiColor,
    pub before_bg: RatatuiColor,
    pub after_fg: RatatuiColor,
    pub after_bg: RatatuiColor,
    pub filter: String,
}

/// Result of shader application.
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderResult {
    pub x: u16,
    pub y: u16,
    pub before: RatatuiStyle,
    pub after: RatatuiStyle,
    pub shader: String,
}

/// Result of style interpolation (FadeIn, Pulse, Rainbow, Glitch, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct StyleInterpolationResult {
    pub phase: AnimationPhase,
    pub t: f64,
    pub before: RatatuiStyle,
    pub after: RatatuiStyle,
    pub effect: String,
    pub target: String,
}

/// Inspector that captures all intermediate pipeline stage results.
///
/// Records every transformation at each stage (sampler, mask, filter, shader, style)
/// for comprehensive pipeline debugging and verification.
pub struct StageInspector {
    pub sampler_results: Vec<SamplerResult>,
    pub mask_results: Vec<MaskResult>,
    pub filter_results: Vec<FilterResult>,
    pub shader_results: Vec<ShaderResult>,
    pub style_results: Vec<StyleInterpolationResult>,
    width: u16,
    height: u16,
}

impl StageInspector {
    /// Create a new StageInspector for a given area size.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            sampler_results: Vec::new(),
            mask_results: Vec::new(),
            filter_results: Vec::new(),
            shader_results: Vec::new(),
            style_results: Vec::new(),
            width,
            height,
        }
    }

    /// Clear all captured results.
    pub fn clear(&mut self) {
        self.sampler_results.clear();
        self.mask_results.clear();
        self.filter_results.clear();
        self.shader_results.clear();
        self.style_results.clear();
    }

    /// Get mask coverage as (visible_count, total_count).
    pub fn mask_coverage(&self) -> (usize, usize) {
        let visible = self.mask_results.iter().filter(|r| r.visible).count();
        let total = self.mask_results.len();
        (visible, total)
    }

    /// Get mask coverage as percentage.
    pub fn mask_coverage_percent(&self) -> f64 {
        let (visible, total) = self.mask_coverage();
        if total == 0 {
            100.0
        } else {
            (visible as f64 / total as f64) * 100.0
        }
    }

    /// Get visibility map as 2D grid of bools.
    pub fn visibility_map(&self) -> Vec<Vec<bool>> {
        let mut map = vec![vec![true; self.width as usize]; self.height as usize];
        for result in &self.mask_results {
            if (result.y as usize) < map.len() && (result.x as usize) < self.width as usize {
                map[result.y as usize][result.x as usize] = result.visible;
            }
        }
        map
    }

    /// Get sampler displacement count (cells that moved).
    pub fn sampler_displacement_count(&self) -> usize {
        self.sampler_results
            .iter()
            .filter(|r| r.src_x != Some(r.dest_x) || r.src_y != Some(r.dest_y))
            .count()
    }

    /// Get filter modification count (cells with color changes).
    pub fn filter_modification_count(&self) -> usize {
        self.filter_results
            .iter()
            .filter(|r| r.before_fg != r.after_fg || r.before_bg != r.after_bg)
            .count()
    }

    /// Get shader modification count (cells with style changes).
    pub fn shader_modification_count(&self) -> usize {
        self.shader_results
            .iter()
            .filter(|r| r.before != r.after)
            .count()
    }

    /// Get style interpolation modification count (effects that changed style).
    pub fn style_modification_count(&self) -> usize {
        self.style_results
            .iter()
            .filter(|r| r.before != r.after)
            .count()
    }

    /// Get unique effect names that were applied.
    pub fn style_effects_applied(&self) -> Vec<String> {
        let mut effects: Vec<String> = self
            .style_results
            .iter()
            .map(|r| r.effect.clone())
            .collect();
        effects.sort();
        effects.dedup();
        effects
    }
}

impl Default for StageInspector {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

impl PipelineInspector for StageInspector {
    fn on_style_interpolated(
        &mut self,
        phase: AnimationPhase,
        t: f64,
        before: RatatuiStyle,
        after: RatatuiStyle,
        effect_name: &str,
        target: &str,
    ) {
        self.style_results.push(StyleInterpolationResult {
            phase,
            t,
            before,
            after,
            effect: effect_name.to_string(),
            target: target.to_string(),
        });
    }

    fn on_sampler_applied(
        &mut self,
        dest_x: u16,
        dest_y: u16,
        src_x: Option<u16>,
        src_y: Option<u16>,
        sampler_name: &str,
    ) {
        self.sampler_results.push(SamplerResult {
            dest_x,
            dest_y,
            src_x,
            src_y,
            sampler: sampler_name.to_string(),
        });
    }

    fn on_mask_checked(&mut self, x: u16, y: u16, visible: bool, mask_name: &str) {
        self.mask_results.push(MaskResult {
            x,
            y,
            visible,
            mask: mask_name.to_string(),
        });
    }

    fn on_shader_applied(
        &mut self,
        x: u16,
        y: u16,
        before: RatatuiStyle,
        after: RatatuiStyle,
        shader_name: &str,
    ) {
        self.shader_results.push(ShaderResult {
            x,
            y,
            before,
            after,
            shader: shader_name.to_string(),
        });
    }

    fn on_filter_applied(
        &mut self,
        x: u16,
        y: u16,
        before: &RatatuiCell,
        after: &RatatuiCell,
        filter_name: &str,
    ) {
        self.filter_results.push(FilterResult {
            x,
            y,
            before_fg: before.fg,
            before_bg: before.bg,
            after_fg: after.fg,
            after_bg: after.bg,
            filter: filter_name.to_string(),
        });
    }
}

/// Implementation of CompositorInspector for use with render_pipeline_inspected.
///
/// The methods delegate to the same internal storage, allowing StageInspector
/// to be used both with the high-level PipelineInspector hooks and the
/// low-level compositor render_pipeline_inspected function.
///
/// Note: CompositorInspector uses tui-vfx types, so we convert to ratatui types
/// before storing in our result vectors.
impl CompositorInspector for StageInspector {
    fn on_sampler_applied(
        &mut self,
        dest_x: u16,
        dest_y: u16,
        src_x: Option<u16>,
        src_y: Option<u16>,
        sampler_name: &str,
    ) {
        self.sampler_results.push(SamplerResult {
            dest_x,
            dest_y,
            src_x,
            src_y,
            sampler: sampler_name.to_string(),
        });
    }

    fn on_mask_checked(&mut self, x: u16, y: u16, visible: bool, mask_name: &str) {
        self.mask_results.push(MaskResult {
            x,
            y,
            visible,
            mask: mask_name.to_string(),
        });
    }

    fn on_shader_applied(
        &mut self,
        x: u16,
        y: u16,
        before: VfxStyle,
        after: VfxStyle,
        shader_name: &str,
    ) {
        // Convert tui-vfx types to ratatui types for storage
        self.shader_results.push(ShaderResult {
            x,
            y,
            before: vfx_style_to_ratatui(before),
            after: vfx_style_to_ratatui(after),
            shader: shader_name.to_string(),
        });
    }

    fn on_filter_applied(
        &mut self,
        x: u16,
        y: u16,
        before: &VfxCell,
        after: &VfxCell,
        filter_name: &str,
    ) {
        // Convert tui-vfx Color to ratatui Color for storage
        use crate::compat::vfx_color_to_ratatui;
        self.filter_results.push(FilterResult {
            x,
            y,
            before_fg: vfx_color_to_ratatui(before.fg),
            before_bg: vfx_color_to_ratatui(before.bg),
            after_fg: vfx_color_to_ratatui(after.fg),
            after_bg: vfx_color_to_ratatui(after.bg),
            filter: filter_name.to_string(),
        });
    }
}

// <FILE>src/inspector/impls/cls_stage_inspector.rs</FILE> - <DESC>StageInspector for capturing all pipeline stage results</DESC>
// <VERS>END OF VERSION: 1.4.0</VERS>
