// <FILE>tools/pipeline-validator/src/rules/functions/fnc_interpolate_message.rs</FILE> - <DESC>Interpolate message template with context values</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from evaluator.rs</CLOG>

use super::fnc_evaluate_field::evaluate_field;
use crate::types::{EvalContext, ValueType};

/// Interpolate message template with values from context
pub fn interpolate_message(template: &str, ctx: &EvalContext) -> String {
    let mut result = template.to_string();

    // Find all {placeholder} patterns manually
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    let mut placeholders = Vec::new();

    while i < chars.len() {
        if chars[i] == '{' {
            // Find closing brace
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '}') {
                let placeholder: String = chars[i + 1..i + 1 + end].iter().collect();
                placeholders.push(placeholder);
                i += end + 2;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    // Evaluate and replace each placeholder
    for placeholder in placeholders {
        if let Ok(value) = evaluate_field(&placeholder, ctx) {
            let value_str = match value {
                ValueType::String(s) => s,
                ValueType::Number(n) => format!("{:.2}", n),
                ValueType::Bool(b) => b.to_string(),
            };
            result = result.replace(&format!("{{{}}}", placeholder), &value_str);
        }
    }

    result
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_interpolate_message.rs</FILE> - <DESC>Interpolate message template with context values</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
