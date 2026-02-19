// <FILE>src/inspector/impls/cls_diff_inspector.rs</FILE> - <DESC>DiffInspector for capturing before/after cell states</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>TUI VFX recipes extraction - revert to ratatui types for PipelineInspector</WCTX>
// <CLOG>Use ratatui types for PipelineInspector compatibility

use crate::inspector::PipelineInspector;
use ratatui::style::Style;

/// Represents a single cell change from a shader application.
#[derive(Debug, Clone, PartialEq)]
pub struct CellChange {
    pub x: u16,
    pub y: u16,
    pub before: Style,
    pub after: Style,
    pub shader: String,
}

/// Inspector that captures before/after states for diff analysis.
///
/// Records every cell modification with before/after styles and the shader
/// that caused the change. Useful for debugging unexpected style changes.
pub struct DiffInspector {
    changes: Vec<CellChange>,
}

impl DiffInspector {
    /// Create a new DiffInspector.
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    /// Get all recorded changes.
    pub fn changes(&self) -> &[CellChange] {
        &self.changes
    }

    /// Check if any changes were recorded.
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    /// Get the total number of changes.
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// Get changes for a specific shader.
    pub fn changes_by_shader(&self, shader_name: &str) -> Vec<&CellChange> {
        self.changes
            .iter()
            .filter(|c| c.shader == shader_name)
            .collect()
    }

    /// Get changes at a specific position.
    pub fn changes_at(&self, x: u16, y: u16) -> Vec<&CellChange> {
        self.changes
            .iter()
            .filter(|c| c.x == x && c.y == y)
            .collect()
    }
}

impl Default for DiffInspector {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineInspector for DiffInspector {
    fn on_shader_applied(
        &mut self,
        x: u16,
        y: u16,
        before: Style,
        after: Style,
        shader_name: &str,
    ) {
        // Only record if the style actually changed
        if before != after {
            self.changes.push(CellChange {
                x,
                y,
                before,
                after,
                shader: shader_name.to_string(),
            });
        }
    }
}

// <FILE>src/inspector/impls/cls_diff_inspector.rs</FILE> - <DESC>DiffInspector for capturing before/after cell states</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
