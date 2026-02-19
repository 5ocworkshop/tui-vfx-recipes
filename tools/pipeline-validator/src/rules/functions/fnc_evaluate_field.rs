// <FILE>tools/pipeline-validator/src/rules/functions/fnc_evaluate_field.rs</FILE> - <DESC>Evaluate field access expression</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Pipeline debugging tools - physics/anchor compatibility validation</WCTX>
// <CLOG>Add layout.anchor, motion_path.from_direction, motion_path.to_direction field access</CLOG>

use super::fnc_get_json_field::get_json_field;
use crate::types::{EvalContext, ValueType};

/// Evaluate a field access expression
pub fn evaluate_field(expr: &str, ctx: &EvalContext) -> Result<ValueType, String> {
    let parts: Vec<&str> = expr.split('.').collect();

    match parts[0] {
        "layout" => {
            if parts.len() != 2 {
                return Err("layout access requires field name (e.g., layout.width)".to_string());
            }
            match parts[1] {
                "width" => Ok(ValueType::Number(ctx.layout.width as f64)),
                "height" => Ok(ValueType::Number(ctx.layout.height as f64)),
                "anchor" => ctx
                    .layout
                    .anchor
                    .as_ref()
                    .map(|a| ValueType::String(a.clone()))
                    .ok_or_else(|| "No anchor in layout".to_string()),
                _ => Err(format!("Unknown layout field: {}", parts[1])),
            }
        }
        "shader" => {
            if parts.len() != 2 {
                return Err("shader access requires field name (e.g., shader.type)".to_string());
            }
            let shader = ctx.shader.as_ref().ok_or("No shader in context")?;
            get_json_field(shader, parts[1])
        }
        "mask" => {
            if parts.len() != 2 {
                return Err("mask access requires field name (e.g., mask.type)".to_string());
            }
            let mask = ctx.mask.as_ref().ok_or("No mask in context")?;
            get_json_field(mask, parts[1])
        }
        "sampler" => {
            if parts.len() != 2 {
                return Err("sampler access requires field name".to_string());
            }
            let sampler = ctx.sampler.as_ref().ok_or("No sampler in context")?;
            get_json_field(sampler, parts[1])
        }
        "filter" => {
            if parts.len() != 2 {
                return Err("filter access requires field name".to_string());
            }
            let filter = ctx.filter.as_ref().ok_or("No filter in context")?;
            get_json_field(filter, parts[1])
        }
        "effect" => {
            if parts.len() != 2 {
                return Err("effect access requires field name".to_string());
            }
            let effect = ctx.effect.as_ref().ok_or("No effect in context")?;
            get_json_field(effect, parts[1])
        }
        "motion_path" => {
            if parts.len() != 2 {
                return Err(
                    "motion_path access requires field name (e.g., motion_path.type)".to_string(),
                );
            }
            let mp_info = ctx
                .motion_path
                .as_ref()
                .ok_or("No motion_path in context")?;
            match parts[1] {
                // Duration and phase come from MotionPathInfo directly
                "duration_ms" => Ok(ValueType::Number(mp_info.duration_ms as f64)),
                "phase" => Ok(ValueType::String(mp_info.phase.clone())),
                // Direction fields from MotionPathInfo
                "from_direction" => mp_info
                    .from_direction
                    .as_ref()
                    .map(|d| ValueType::String(d.clone()))
                    .ok_or_else(|| "No from_direction specified".to_string()),
                "to_direction" => mp_info
                    .to_direction
                    .as_ref()
                    .map(|d| ValueType::String(d.clone()))
                    .ok_or_else(|| "No to_direction specified".to_string()),
                // All other fields come from the path JSON object
                field => get_json_field(&mp_info.path, field),
            }
        }
        _ => {
            // Direct field access (no prefix) - check current context
            if let Some(ref shader) = ctx.shader {
                if let Ok(val) = get_json_field(shader, expr) {
                    return Ok(val);
                }
            }
            if let Some(ref mask) = ctx.mask {
                if let Ok(val) = get_json_field(mask, expr) {
                    return Ok(val);
                }
            }
            if let Some(ref sampler) = ctx.sampler {
                if let Ok(val) = get_json_field(sampler, expr) {
                    return Ok(val);
                }
            }
            if let Some(ref filter) = ctx.filter {
                if let Ok(val) = get_json_field(filter, expr) {
                    return Ok(val);
                }
            }
            if let Some(ref effect) = ctx.effect {
                if let Ok(val) = get_json_field(effect, expr) {
                    return Ok(val);
                }
            }
            if let Some(ref mp_info) = ctx.motion_path {
                // Check special fields first
                if expr == "duration_ms" {
                    return Ok(ValueType::Number(mp_info.duration_ms as f64));
                }
                if expr == "phase" {
                    return Ok(ValueType::String(mp_info.phase.clone()));
                }
                if let Ok(val) = get_json_field(&mp_info.path, expr) {
                    return Ok(val);
                }
            }
            Err(format!("Field not found: {}", expr))
        }
    }
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_evaluate_field.rs</FILE> - <DESC>Evaluate field access expression</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
