// <FILE>src/inspector/traits.rs</FILE> - <DESC>PipelineInspector trait definition</DESC>
// <VERS>VERSION: 1.4.0</VERS>
// <WCTX>TUI VFX recipes extraction - keep ratatui types for rendering compatibility</WCTX>
// <CLOG>Revert to ratatui types - rendering pipeline uses ratatui directly

use crate::recipe_schema::config::RaRecipeConfig;
use crate::rendering::types::RenderPlanItem;
use crate::state::AnimationPhase;
use crate::types::AnimationProfile;
use ratatui::buffer::{Buffer, Cell};
use ratatui::style::Style;
use tui_vfx_style::models::{StyleEffect, StyleLayer, StyleRegion};

/// Trait for inspecting the render pipeline at key points.
///
/// All methods have default no-op implementations, allowing inspectors
/// to selectively implement only the hooks they care about.
///
/// Note: This trait uses ratatui types because the rendering pipeline
/// operates on ratatui::Buffer directly. For compositor-level inspection
/// using tui-vfx types, implement CompositorInspector instead.
///
/// The pipeline flow is:
/// 1. JSON parsed → on_config_parsed
/// 2. Profile built → on_profile_built
/// 3. Style layers extracted → on_style_layers_extracted
/// 4. Phase changes → on_phase_entered
/// 5. Render plan created → on_render_plan_created
/// 6. Per-phase effects extracted → on_effect_extracted
/// 7. Style interpolation applied → on_style_interpolated (FadeIn, Pulse, Rainbow, etc.)
/// 8. Per-shader layers built → on_shader_layer_built
/// 9. Per-cell sampler application → on_sampler_applied
/// 10. Per-cell mask check → on_mask_checked
/// 11. Per-cell shader application → on_shader_applied (spatial shaders only)
/// 12. Per-cell filter application → on_filter_applied
/// 13. Per-cell final render → on_cell_rendered
/// 14. Frame complete → on_frame_complete
pub trait PipelineInspector {
    /// Called after JSON config is parsed.
    fn on_config_parsed(&mut self, _config: &RaRecipeConfig) {}

    /// Called after AnimationProfile is built from config.
    fn on_profile_built(&mut self, _profile: &AnimationProfile) {}

    /// Called after style layers are extracted from profile.
    fn on_style_layers_extracted(&mut self, _layers: &[StyleLayer]) {}

    /// Called when animation phase changes (Entering → Dwelling → Exiting).
    fn on_phase_entered(&mut self, _phase: AnimationPhase) {}

    /// Called after render plan item is created for current frame.
    fn on_render_plan_created(&mut self, _plan: &RenderPlanItem) {}

    /// Called when an effect is extracted for a specific phase.
    /// effect will be None if no effect is active for this phase.
    fn on_effect_extracted(&mut self, _phase: AnimationPhase, _effect: Option<&StyleEffect>) {}

    /// Called when a style interpolation effect (FadeIn, Pulse, Rainbow, Glitch, etc.)
    /// is applied to compute the final style. This captures non-spatial style effects
    /// that modify colors/modifiers over time via StyleInterpolator::calculate().
    ///
    /// # Arguments
    /// * `phase` - Current animation phase (Entering, Dwelling, Exiting)
    /// * `t` - Animation progress (0.0 to 1.0)
    /// * `before` - Style before effect application (ratatui::Style)
    /// * `after` - Style after effect application (ratatui::Style)
    /// * `effect_name` - Name of the effect (e.g., "FadeIn", "Pulse", "Rainbow")
    /// * `target` - Which style target this applies to (e.g., "frame", "border", "text")
    fn on_style_interpolated(
        &mut self,
        _phase: AnimationPhase,
        _t: f64,
        _before: Style,
        _after: Style,
        _effect_name: &str,
        _target: &str,
    ) {
    }

    /// Called when a shader layer is built for a specific region.
    /// shader_name identifies the shader type (e.g., "PulseWave", "BorderSweep").
    fn on_shader_layer_built(&mut self, _shader_name: &str, _region: &StyleRegion) {}

    /// Called after sampler transforms coordinates for a cell.
    /// src_x/src_y are None if the sampler skipped the cell (e.g., gap in shredder).
    fn on_sampler_applied(
        &mut self,
        _dest_x: u16,
        _dest_y: u16,
        _src_x: Option<u16>,
        _src_y: Option<u16>,
        _sampler_name: &str,
    ) {
    }

    /// Called after mask visibility is checked for a cell.
    /// visible indicates whether the cell passed the mask check.
    fn on_mask_checked(&mut self, _x: u16, _y: u16, _visible: bool, _mask_name: &str) {}

    /// Called when a shader is applied to a specific cell.
    /// before: original style, after: style after shader application.
    fn on_shader_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _before: Style,
        _after: Style,
        _shader_name: &str,
    ) {
    }

    /// Called after a filter is applied to a cell.
    /// before/after contain the cell state before and after filter application.
    fn on_filter_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _before: &Cell,
        _after: &Cell,
        _filter_name: &str,
    ) {
    }

    /// Called after all effects have been applied to a cell.
    /// final_cell contains the fully rendered cell with all effects.
    fn on_cell_rendered(&mut self, _x: u16, _y: u16, _final_cell: &Cell) {}

    /// Called after entire frame is rendered.
    fn on_frame_complete(&mut self, _buffer: &Buffer) {}
}

// <FILE>src/inspector/traits.rs</FILE> - <DESC>PipelineInspector trait definition</DESC>
// <VERS>END OF VERSION: 1.4.0</VERS>
