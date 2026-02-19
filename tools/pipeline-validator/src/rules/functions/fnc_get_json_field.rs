// <FILE>tools/pipeline-validator/src/rules/functions/fnc_get_json_field.rs</FILE> - <DESC>Extract field from JSON value</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from evaluator.rs</CLOG>

use crate::types::ValueType;
use serde_json::Value;

/// Get a field from a JSON value and convert to ValueType
pub fn get_json_field(json: &Value, field: &str) -> Result<ValueType, String> {
    let value = json
        .get(field)
        .ok_or_else(|| format!("Field '{}' not found", field))?;

    match value {
        Value::String(s) => Ok(ValueType::String(s.clone())),
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(ValueType::Number(f))
            } else {
                Err(format!(
                    "Number field '{}' cannot be converted to f64",
                    field
                ))
            }
        }
        Value::Bool(b) => Ok(ValueType::Bool(*b)),
        _ => Err(format!("Field '{}' has unsupported type", field)),
    }
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_get_json_field.rs</FILE> - <DESC>Extract field from JSON value</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
