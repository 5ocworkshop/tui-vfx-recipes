// <FILE>tools/pipeline-validator/src/stages/mod.rs</FILE> - <DESC>Validation stages module</DESC>
// <VERS>VERSION: 2.3.0</VERS>
// <WCTX>Region-constrained wipe effect for enter animations</WCTX>
// <CLOG>Add warnings field and with_warning method for non-fatal validation issues</CLOG>

pub mod functions;
pub mod parse;
pub mod rules;

// Re-export validation functions for convenience
#[allow(unused_imports)]
pub use functions::{
    benchmark_stages, count_buffer_cells, sample_buffer_cells, validate_output, validate_profile,
    validate_render, validate_shader, validate_stages,
};

use serde::Serialize;

/// Result from a validation stage.
#[derive(Debug, Clone, Serialize)]
pub struct StageResult {
    /// Name of the stage
    pub stage: String,
    /// Whether the stage passed
    pub passed: bool,
    /// Diagnostic messages
    pub messages: Vec<String>,
    /// Detailed information (for verbose mode)
    pub details: Vec<String>,
    /// Non-fatal warnings (issues that don't fail validation but may cause unexpected behavior)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

impl StageResult {
    /// Create a passing result.
    pub fn pass(stage: &str) -> Self {
        Self {
            stage: stage.to_string(),
            passed: true,
            messages: Vec::new(),
            details: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create a failing result with an error message.
    pub fn fail(stage: &str, error: impl Into<String>) -> Self {
        Self {
            stage: stage.to_string(),
            passed: false,
            messages: vec![error.into()],
            details: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Add a success message.
    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.messages.push(msg.into());
        self
    }

    /// Add a detail (shown in verbose mode).
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.details.push(detail.into());
        self
    }

    /// Add a warning (non-fatal issue that may cause unexpected behavior).
    ///
    /// Warnings don't fail validation but indicate potential configuration issues
    /// like phase durations too short for spatial effects to be perceptible.
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }
}

// <FILE>tools/pipeline-validator/src/stages/mod.rs</FILE> - <DESC>Validation stages module</DESC>
// <VERS>END OF VERSION: 2.3.0</VERS>
