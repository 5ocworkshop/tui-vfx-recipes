// <FILE>tools/pipeline-validator/src/types/cls_severity.rs</FILE> - <DESC>Rule severity level enum</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from rules/mod.rs</CLOG>

use serde::{Deserialize, Serialize};
use std::fmt;

/// Rule severity level indicating impact of violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Must fix - effect is broken or invisible
    Error,
    /// Should consider - effect may be suboptimal
    Warning,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
        }
    }
}

// <FILE>tools/pipeline-validator/src/types/cls_severity.rs</FILE> - <DESC>Rule severity level enum</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
