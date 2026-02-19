// <FILE>tools/pipeline-validator/src/types/cls_rule_violation.rs</FILE> - <DESC>Rule violation result</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from rules/mod.rs</CLOG>

use super::Severity;
use serde::Serialize;

/// A rule violation found during validation.
#[derive(Debug, Clone, Serialize)]
pub struct RuleViolation {
    /// ID of the rule that was violated
    pub rule_id: String,
    /// Severity of this violation
    pub severity: Severity,
    /// Interpolated message with placeholders filled in
    pub message: String,
    /// Optional hint for fixing the issue
    pub fix_hint: Option<String>,
    /// Location in the recipe where violation occurred
    /// (e.g., "pipeline.styles[0].dwell_effect.shader")
    pub location: String,
}

// <FILE>tools/pipeline-validator/src/types/cls_rule_violation.rs</FILE> - <DESC>Rule violation result</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
