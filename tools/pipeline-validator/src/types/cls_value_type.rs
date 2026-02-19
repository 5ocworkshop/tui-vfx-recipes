// <FILE>tools/pipeline-validator/src/types/cls_value_type.rs</FILE> - <DESC>Internal value type for expression evaluation</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from evaluator.rs</CLOG>

use std::cmp::Ordering;

/// Internal value type for comparisons in rule conditions
#[derive(Debug, Clone)]
pub enum ValueType {
    String(String),
    Number(f64),
    Bool(bool),
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::String(a), ValueType::String(b)) => a == b,
            (ValueType::Number(a), ValueType::Number(b)) => (a - b).abs() < 1e-10,
            (ValueType::Bool(a), ValueType::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for ValueType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ValueType::Number(a), ValueType::Number(b)) => a.partial_cmp(b),
            (ValueType::String(a), ValueType::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

// <FILE>tools/pipeline-validator/src/types/cls_value_type.rs</FILE> - <DESC>Internal value type for expression evaluation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
