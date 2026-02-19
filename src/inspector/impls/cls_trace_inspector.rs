// <FILE>src/inspector/impls/cls_trace_inspector.rs</FILE> - <DESC>TraceInspector for logging pipeline events</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>TUI VFX recipes extraction - revert to ratatui types for PipelineInspector</WCTX>
// <CLOG>Use ratatui types for PipelineInspector compatibility

use crate::inspector::PipelineInspector;
use crate::recipe_schema::config::RaRecipeConfig;
use crate::rendering::types::RenderPlanItem;
use crate::state::AnimationPhase;
use crate::types::AnimationProfile;
use ratatui::buffer::{Buffer, Cell};
use ratatui::style::Style;
use tui_vfx_style::models::{StyleEffect, StyleLayer, StyleRegion};

/// Verbosity level for trace output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceVerbosity {
    /// Only major checkpoints (config, profile, layers, phases).
    Minimal,
    /// Include render plan and effect extraction.
    Normal,
    /// Include per-shader and per-cell events.
    Verbose,
}

/// Inspector that logs pipeline events to a string buffer.
///
/// Useful for debugging pipeline flow and understanding event sequences.
pub struct TraceInspector {
    verbosity: TraceVerbosity,
    output: String,
}

impl TraceInspector {
    /// Create a new TraceInspector with Normal verbosity.
    pub fn new() -> Self {
        Self {
            verbosity: TraceVerbosity::Normal,
            output: String::new(),
        }
    }

    /// Set verbosity level.
    pub fn with_verbosity(mut self, verbosity: TraceVerbosity) -> Self {
        self.verbosity = verbosity;
        self
    }

    /// Get the accumulated output.
    pub fn output(&self) -> &str {
        &self.output
    }

    fn log(&mut self, message: String) {
        self.output.push_str(&message);
        self.output.push('\n');
    }
}

impl Default for TraceInspector {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineInspector for TraceInspector {
    fn on_config_parsed(&mut self, config: &RaRecipeConfig) {
        self.log(format!("CONFIG_PARSED: message={:?}", config.message));
    }

    fn on_profile_built(&mut self, _profile: &AnimationProfile) {
        self.log("PROFILE_BUILT".to_string());
    }

    fn on_style_layers_extracted(&mut self, layers: &[StyleLayer]) {
        self.log(format!("STYLE_LAYERS_EXTRACTED: count={}", layers.len()));
    }

    fn on_phase_entered(&mut self, phase: AnimationPhase) {
        self.log(format!("PHASE_ENTERED: {:?}", phase));
    }

    fn on_render_plan_created(&mut self, plan: &RenderPlanItem) {
        if self.verbosity as u8 >= TraceVerbosity::Normal as u8 {
            self.log(format!(
                "RENDER_PLAN_CREATED: id={}, phase={:?}",
                plan.id, plan.phase
            ));
        }
    }

    fn on_effect_extracted(&mut self, phase: AnimationPhase, effect: Option<&StyleEffect>) {
        if self.verbosity as u8 >= TraceVerbosity::Normal as u8 {
            match effect {
                Some(_) => self.log(format!(
                    "EFFECT_EXTRACTED: phase={:?}, effect=Some(...)",
                    phase
                )),
                None => self.log(format!("EFFECT_EXTRACTED: phase={:?}, effect=None", phase)),
            }
        }
    }

    fn on_shader_layer_built(&mut self, shader_name: &str, region: &StyleRegion) {
        if self.verbosity as u8 >= TraceVerbosity::Verbose as u8 {
            self.log(format!(
                "SHADER_LAYER_BUILT: shader={}, region={:?}",
                shader_name, region
            ));
        }
    }

    fn on_shader_applied(
        &mut self,
        x: u16,
        y: u16,
        _before: Style,
        _after: Style,
        shader_name: &str,
    ) {
        if self.verbosity as u8 >= TraceVerbosity::Verbose as u8 {
            self.log(format!(
                "SHADER_APPLIED: x={}, y={}, shader={}",
                x, y, shader_name
            ));
        }
    }

    fn on_cell_rendered(&mut self, x: u16, y: u16, _final_cell: &Cell) {
        if self.verbosity as u8 >= TraceVerbosity::Verbose as u8 {
            self.log(format!("CELL_RENDERED: x={}, y={}", x, y));
        }
    }

    fn on_frame_complete(&mut self, buffer: &Buffer) {
        self.log(format!(
            "FRAME_COMPLETE: buffer={}x{}",
            buffer.area.width, buffer.area.height
        ));
    }
}

// <FILE>src/inspector/impls/cls_trace_inspector.rs</FILE> - <DESC>TraceInspector for logging pipeline events</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
