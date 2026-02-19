// <FILE>tools/pipeline-validator/src/types/cls_rule.rs</FILE> - <DESC>Validation rule definition</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from rules/mod.rs</CLOG>

use super::Severity;
use serde::{Deserialize, Serialize};

/// A single validation rule loaded from TOML configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Unique identifier for this rule (e.g., "glisten-band-coverage")
    pub id: String,
    /// Severity level when rule is violated
    pub severity: Severity,
    /// Category for grouping related rules (e.g., "spatial.glisten_band")
    pub category: String,
    /// Human-readable description of what the rule checks
    pub description: String,
    /// Expression to evaluate (pseudo-DSL for now, will be parsed later)
    pub condition: String,
    /// Message template with {placeholders} for interpolation
    pub message: String,
    /// Optional hint for how to fix the violation
    pub fix_hint: Option<String>,
    /// Optional numeric threshold for comparison-based rules
    pub threshold: Option<f64>,
}

// <FILE>tools/pipeline-validator/src/types/cls_rule.rs</FILE> - <DESC>Validation rule definition</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
