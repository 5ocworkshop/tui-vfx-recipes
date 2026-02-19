// <FILE>tools/pipeline-validator/src/rules/functions/fnc_evaluate_condition.rs</FILE> - <DESC>Evaluate condition expression against context</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Pipeline debugging tools - physics/anchor compatibility validation</WCTX>
// <CLOG>Add 'in' operator for list membership: value in ['a', 'b', 'c']</CLOG>

use super::fnc_compare_values::compare_values;
use super::fnc_evaluate_value::evaluate_value;
use super::fnc_split_on_operator::split_on_operator;
use crate::types::EvalContext;

/// Evaluate a condition expression against the context
pub fn evaluate_condition(condition: &str, ctx: &EvalContext) -> Result<bool, String> {
    let expr = condition.trim();

    // Handle boolean operators (split from outermost to innermost)
    if let Some((left, right)) = split_on_operator(expr, " or ") {
        let left_result = evaluate_condition(left, ctx)?;
        let right_result = evaluate_condition(right, ctx)?;
        return Ok(left_result || right_result);
    }

    if let Some((left, right)) = split_on_operator(expr, " and ") {
        let left_result = evaluate_condition(left, ctx)?;
        let right_result = evaluate_condition(right, ctx)?;
        return Ok(left_result && right_result);
    }

    // Handle 'not' operator
    if expr.starts_with("not ") {
        let inner = expr.trim_start_matches("not ").trim();
        return Ok(!evaluate_condition(inner, ctx)?);
    }

    // Handle 'in' operator for list membership: value in ['a', 'b', 'c']
    if let Some((left, right)) = split_on_operator(expr, " in ") {
        let left_val = evaluate_value(left.trim(), ctx)?;

        // Parse the list on the right side: ['a', 'b', 'c']
        let right_trimmed = right.trim();
        if right_trimmed.starts_with('[') && right_trimmed.ends_with(']') {
            let inner = &right_trimmed[1..right_trimmed.len() - 1];
            // Split on comma, handling quoted strings
            for item in inner.split(',') {
                let item = item.trim().trim_matches('\'').trim_matches('"');
                let item_val = crate::types::ValueType::String(item.to_string());
                if compare_values(&left_val, &item_val, |a, b| a == b)? {
                    return Ok(true);
                }
            }
            return Ok(false);
        }

        return Err(format!(
            "'in' operator requires array literal, got: {}",
            right_trimmed
        ));
    }

    // Handle comparison operators
    for op in &[">=", "<=", "==", "!=", ">", "<"] {
        if let Some((left, right)) = split_on_operator(expr, op) {
            let left_val = evaluate_value(left.trim(), ctx)?;
            let right_val = evaluate_value(right.trim(), ctx)?;

            return match *op {
                ">=" => compare_values(&left_val, &right_val, |a, b| a >= b),
                "<=" => compare_values(&left_val, &right_val, |a, b| a <= b),
                "==" => compare_values(&left_val, &right_val, |a, b| a == b),
                "!=" => compare_values(&left_val, &right_val, |a, b| a != b),
                ">" => compare_values(&left_val, &right_val, |a, b| a > b),
                "<" => compare_values(&left_val, &right_val, |a, b| a < b),
                _ => unreachable!(),
            };
        }
    }

    Err(format!("Invalid condition expression: {}", expr))
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_evaluate_condition.rs</FILE> - <DESC>Evaluate condition expression against context</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
