// <FILE>tools/pipeline-validator/src/rules/functions/fnc_evaluate_function.rs</FILE> - <DESC>Evaluate function call expressions</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Pipeline debugging tools - faultline visibility validation</WCTX>
// <CLOG>Add faultline functions: fault_line_displacement, half_width, max_safe_fault_intensity</CLOG>

use super::fnc_color_utils;
use super::fnc_evaluate_field::evaluate_field;
use crate::types::{EvalContext, ValueType};

/// Evaluate a function call
pub fn evaluate_function(expr: &str, ctx: &EvalContext) -> Result<ValueType, String> {
    let paren_pos = expr.find('(').ok_or("Invalid function call")?;
    let func_name = expr[..paren_pos].trim();
    let args_str = &expr[paren_pos + 1..expr.len() - 1];

    match func_name {
        "max_projection" => {
            // Parse arguments: layout, angle_deg
            let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err(format!(
                    "max_projection expects 2 arguments, got {}",
                    args.len()
                ));
            }

            // First arg should be "layout"
            if args[0] != "layout" {
                return Err("max_projection first argument must be 'layout'".to_string());
            }

            // Second arg is angle field
            let angle = match evaluate_field(args[1], ctx)? {
                ValueType::Number(n) => n,
                _ => return Err("angle_deg must be a number".to_string()),
            };

            // Calculate max projection: width * |cos(angle)| + height * |sin(angle)|
            let angle_rad = angle.to_radians();
            let projection = (ctx.layout.width as f64) * angle_rad.cos().abs()
                + (ctx.layout.height as f64) * angle_rad.sin().abs();

            Ok(ValueType::Number(projection))
        }

        "color_distance" => {
            // Parse arguments: color1, color2 (e.g., shader.head, style.base.fg)
            let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err(format!(
                    "color_distance expects 2 arguments, got {}",
                    args.len()
                ));
            }

            let color1 = resolve_color_arg(args[0], ctx)?;
            let color2 = resolve_color_arg(args[1], ctx)?;

            let distance = fnc_color_utils::color_distance(&color1, &color2)?;
            Ok(ValueType::Number(distance))
        }

        "is_dark" => {
            // Parse single color argument
            let arg = args_str.trim();
            let color = resolve_color_arg(arg, ctx)?;
            let is_dark = fnc_color_utils::is_dark(&color)?;
            Ok(ValueType::Bool(is_dark))
        }

        "max_luminance" => {
            // Parse arguments: color1, color2
            let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err(format!(
                    "max_luminance expects 2 arguments, got {}",
                    args.len()
                ));
            }

            let color1 = resolve_color_arg(args[0], ctx)?;
            let color2 = resolve_color_arg(args[1], ctx)?;

            let max_lum = fnc_color_utils::max_luminance(&color1, &color2)?;
            Ok(ValueType::Number(max_lum))
        }

        "luminance" => {
            // Parse single color argument
            let arg = args_str.trim();
            let color = resolve_color_arg(arg, ctx)?;
            let lum = fnc_color_utils::luminance(&color)?;
            Ok(ValueType::Number(lum))
        }

        "fault_line_displacement" => {
            // Calculate max faultline displacement at t=0: intensity * 20
            let arg = args_str.trim();
            let intensity = match evaluate_field(arg, ctx)? {
                ValueType::Number(n) => n,
                _ => return Err("intensity must be a number".to_string()),
            };
            Ok(ValueType::Number(intensity * 20.0))
        }

        "half_width" => {
            // Returns layout.width / 2
            Ok(ValueType::Number(ctx.layout.width as f64 / 2.0))
        }

        "max_safe_fault_intensity" => {
            // Returns max safe intensity: layout.width / 40
            // (so that intensity * 20 < width / 2)
            Ok(ValueType::Number(ctx.layout.width as f64 / 40.0))
        }

        _ => Err(format!("Unknown function: {}", func_name)),
    }
}

/// Resolve a color argument from a field path
fn resolve_color_arg(arg: &str, ctx: &EvalContext) -> Result<serde_json::Value, String> {
    let parts: Vec<&str> = arg.split('.').collect();

    match parts.as_slice() {
        // shader.head, shader.tail
        ["shader", field] => {
            let shader = ctx.shader.as_ref().ok_or("No shader in context")?;
            shader
                .get(*field)
                .cloned()
                .ok_or_else(|| format!("Shader field '{}' not found", field))
        }

        // style.base.fg, style.base.bg
        ["style", "base", field] => {
            let style_base = ctx.style_base.as_ref().ok_or("No style_base in context")?;
            match *field {
                "fg" => style_base
                    .fg
                    .clone()
                    .ok_or_else(|| "style.base.fg not set".to_string()),
                "bg" => style_base
                    .bg
                    .clone()
                    .ok_or_else(|| "style.base.bg not set".to_string()),
                _ => Err(format!("Unknown style.base field: {}", field)),
            }
        }

        // Direct field on current context (e.g., "head" when shader is set)
        [field] => {
            if let Some(ref shader) = ctx.shader {
                if let Some(val) = shader.get(*field) {
                    return Ok(val.clone());
                }
            }
            Err(format!("Color field '{}' not found", field))
        }

        _ => Err(format!("Invalid color path: {}", arg)),
    }
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_evaluate_function.rs</FILE> - <DESC>Evaluate function call expressions</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
