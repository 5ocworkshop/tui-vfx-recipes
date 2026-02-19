// <FILE>tools/pipeline-validator/src/types/cls_rule_set.rs</FILE> - <DESC>Container for validation rules</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from rules/mod.rs</CLOG>

use super::Rule;
use serde::{Deserialize, Serialize};

/// Container for a set of loaded validation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    /// Version of the ruleset format
    pub version: String,
    /// Description of what this ruleset validates
    pub description: String,
    /// The actual rules to apply
    pub rules: Vec<Rule>,
}

// <FILE>tools/pipeline-validator/src/types/cls_rule_set.rs</FILE> - <DESC>Container for validation rules</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
