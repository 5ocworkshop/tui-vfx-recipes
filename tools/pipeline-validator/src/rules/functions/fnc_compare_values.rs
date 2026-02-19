// <FILE>tools/pipeline-validator/src/rules/functions/fnc_compare_values.rs</FILE> - <DESC>Compare two ValueType values</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from evaluator.rs</CLOG>

use crate::types::ValueType;

/// Compare two values using the given comparison function
pub fn compare_values<F>(left: &ValueType, right: &ValueType, cmp: F) -> Result<bool, String>
where
    F: Fn(&ValueType, &ValueType) -> bool,
{
    // Type checking - values must be comparable
    match (left, right) {
        (ValueType::Number(_), ValueType::Number(_)) => Ok(cmp(left, right)),
        (ValueType::String(_), ValueType::String(_)) => Ok(cmp(left, right)),
        (ValueType::Bool(_), ValueType::Bool(_)) => Ok(cmp(left, right)),
        _ => Err("Cannot compare values of different types".to_string()),
    }
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_compare_values.rs</FILE> - <DESC>Compare two ValueType values</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
