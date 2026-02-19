// <FILE>tools/pipeline-validator/src/rules/mod.rs</FILE> - <DESC>Visibility rules module exports</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Restructured with OFPF naming conventions</CLOG>

pub mod fnc_load_rules;
pub mod functions;

// Re-export loader functions
pub use fnc_load_rules::{load_default_rules, load_rules_from_file};

// Re-export evaluator functions
pub use functions::{evaluate_condition, interpolate_message};

// Re-export types from types module for convenience
#[allow(unused_imports)]
pub use crate::types::{
    EvalContext, LayoutInfo, Rule, RuleSet, RuleViolation, Severity, ValueType,
};

// <FILE>tools/pipeline-validator/src/rules/mod.rs</FILE> - <DESC>Visibility rules module exports</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
