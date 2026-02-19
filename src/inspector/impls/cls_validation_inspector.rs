// <FILE>src/inspector/impls/cls_validation_inspector.rs</FILE> - <DESC>ValidationInspector for verifying pipeline state</DESC>
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
use std::collections::HashSet;
use tui_vfx_style::models::{StyleEffect, StyleLayer, StyleRegion};

/// Inspector that validates pipeline state meets expectations.
///
/// Tracks key checkpoints and flags errors/warnings when state is invalid.
pub struct ValidationInspector {
    config_parsed: bool,
    profile_built: bool,
    style_layers_count: usize,
    effects_seen: HashSet<String>,
    shaders_built: Vec<String>,
    cells_modified: usize,
    total_cells: usize,
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl ValidationInspector {
    /// Create a new ValidationInspector.
    pub fn new() -> Self {
        Self {
            config_parsed: false,
            profile_built: false,
            style_layers_count: 0,
            effects_seen: HashSet::new(),
            shaders_built: Vec::new(),
            cells_modified: 0,
            total_cells: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Check if any errors were detected.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get all errors.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Get all warnings.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Get percentage of cells modified.
    pub fn modification_percentage(&self) -> f64 {
        if self.total_cells == 0 {
            0.0
        } else {
            (self.cells_modified as f64 / self.total_cells as f64) * 100.0
        }
    }

    /// Get the number of unique effects seen.
    pub fn effects_count(&self) -> usize {
        self.effects_seen.len()
    }

    /// Get the number of shader layers built.
    pub fn shaders_count(&self) -> usize {
        self.shaders_built.len()
    }

    /// Get the number of style layers extracted.
    pub fn style_layers_count(&self) -> usize {
        self.style_layers_count
    }
}

impl Default for ValidationInspector {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineInspector for ValidationInspector {
    fn on_config_parsed(&mut self, _config: &RaRecipeConfig) {
        self.config_parsed = true;
    }

    fn on_profile_built(&mut self, _profile: &AnimationProfile) {
        if !self.config_parsed {
            self.errors
                .push("Profile built before config parsed".to_string());
        }
        self.profile_built = true;
    }

    fn on_style_layers_extracted(&mut self, layers: &[StyleLayer]) {
        if !self.profile_built {
            self.errors
                .push("Style layers extracted before profile built".to_string());
        }

        self.style_layers_count = layers.len();

        if layers.is_empty() {
            self.warnings.push("No style layers extracted".to_string());
        }
    }

    fn on_phase_entered(&mut self, _phase: AnimationPhase) {
        if !self.profile_built {
            self.errors
                .push("Phase entered before profile built".to_string());
        }
    }

    fn on_render_plan_created(&mut self, _plan: &RenderPlanItem) {
        // Render plan created - no validation needed here
    }

    fn on_effect_extracted(&mut self, phase: AnimationPhase, effect: Option<&StyleEffect>) {
        if let Some(_e) = effect {
            self.effects_seen.insert(format!("{:?}", phase));
        }
    }

    fn on_shader_layer_built(&mut self, shader_name: &str, _region: &StyleRegion) {
        self.shaders_built.push(shader_name.to_string());
    }

    fn on_shader_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _before: Style,
        _after: Style,
        _shader_name: &str,
    ) {
        self.cells_modified += 1;
    }

    fn on_cell_rendered(&mut self, _x: u16, _y: u16, _final_cell: &Cell) {
        // Cell rendered - no validation needed here
    }

    fn on_frame_complete(&mut self, buffer: &Buffer) {
        self.total_cells = (buffer.area.width as usize) * (buffer.area.height as usize);

        if self.cells_modified == 0 {
            self.warnings
                .push("Frame complete but no cells were modified".to_string());
        }
    }
}

// <FILE>src/inspector/impls/cls_validation_inspector.rs</FILE> - <DESC>ValidationInspector for verifying pipeline state</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
