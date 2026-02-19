// <FILE>tools/pipeline-validator/src/rules/functions/fnc_evaluate_value.rs</FILE> - <DESC>Evaluate value expression</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from evaluator.rs</CLOG>

use super::fnc_evaluate_field::evaluate_field;
use super::fnc_evaluate_function::evaluate_function;
use crate::types::{EvalContext, ValueType};

/// Evaluate a value expression (field access, literal, or function call)
pub fn evaluate_value(expr: &str, ctx: &EvalContext) -> Result<ValueType, String> {
    let expr = expr.trim();

    // String literal
    if (expr.starts_with('"') && expr.ends_with('"'))
        || (expr.starts_with('\'') && expr.ends_with('\''))
    {
        return Ok(ValueType::String(expr[1..expr.len() - 1].to_string()));
    }

    // Number literal
    if let Ok(num) = expr.parse::<f64>() {
        return Ok(ValueType::Number(num));
    }

    // Function call
    if expr.contains('(') && expr.ends_with(')') {
        return evaluate_function(expr, ctx);
    }

    // Field access
    evaluate_field(expr, ctx)
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_evaluate_value.rs</FILE> - <DESC>Evaluate value expression</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
