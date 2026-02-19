// <FILE>tools/pipeline-validator/src/stages/rules.rs</FILE> - <DESC>Rules stage: visibility rules validation</DESC>
// <VERS>VERSION: 0.8.0</VERS>
// <WCTX>Pipeline debugging tools - physics/anchor compatibility validation</WCTX>
// <CLOG>Add anchor to layout context; add from/to direction extraction for physics position rules</CLOG>

use serde_json::{Value, json};
use tui_vfx_recipes::recipe_schema::RaRecipeConfig;

use crate::cli::Args;
use crate::rules::{evaluate_condition, interpolate_message};
use crate::stages::StageResult;
use crate::types::{
    EvalContext, LayoutInfo, MotionPathInfo, RuleSet, RuleViolation, Severity, StyleBase,
    TimeConfig,
};

/// Validate the rules stage: check recipe against visibility rules
pub fn validate(config: &RaRecipeConfig, rules: &RuleSet, args: &Args) -> StageResult {
    let mut result = StageResult::pass("rules");
    let mut violations: Vec<RuleViolation> = Vec::new();

    // Build base context with layout info
    let base_ctx = build_base_context(config);

    // Evaluate layout-level rules (no shader/mask context needed)
    for rule in &rules.rules {
        if rule.category.starts_with("layout") || rule.category.starts_with("timing") {
            if let Ok(true) = evaluate_condition(&rule.condition, &base_ctx) {
                violations.push(RuleViolation {
                    rule_id: rule.id.clone(),
                    severity: rule.severity.clone(),
                    message: interpolate_message(&rule.message, &base_ctx),
                    fix_hint: rule.fix_hint.clone(),
                    location: "layout".to_string(),
                });
            }
        }
    }

    // Extract and evaluate shaders from the pipeline
    let shaders = extract_shaders(config);
    for (location, shader_json) in shaders {
        let ctx = EvalContext {
            shader: Some(shader_json),
            ..base_ctx.clone()
        };

        for rule in &rules.rules {
            // Evaluate spatial, shader, and color.contrast rules in shader context
            if rule.category.starts_with("spatial")
                || rule.category.starts_with("shader")
                || rule.category.starts_with("color.contrast")
            {
                if let Ok(true) = evaluate_condition(&rule.condition, &ctx) {
                    violations.push(RuleViolation {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        message: interpolate_message(&rule.message, &ctx),
                        fix_hint: rule.fix_hint.clone(),
                        location: location.clone(),
                    });
                }
            }
        }
    }

    // Extract and evaluate masks
    let masks = extract_masks(config);
    for (location, mask_json) in masks {
        let ctx = EvalContext {
            mask: Some(mask_json),
            ..base_ctx.clone()
        };

        for rule in &rules.rules {
            if rule.category.starts_with("mask") {
                if let Ok(true) = evaluate_condition(&rule.condition, &ctx) {
                    violations.push(RuleViolation {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        message: interpolate_message(&rule.message, &ctx),
                        fix_hint: rule.fix_hint.clone(),
                        location: location.clone(),
                    });
                }
            }
        }
    }

    // Extract and evaluate samplers
    let samplers = extract_samplers(config);
    for (location, sampler_json) in samplers {
        let ctx = EvalContext {
            sampler: Some(sampler_json),
            ..base_ctx.clone()
        };

        for rule in &rules.rules {
            if rule.category.starts_with("sampler") {
                if let Ok(true) = evaluate_condition(&rule.condition, &ctx) {
                    violations.push(RuleViolation {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        message: interpolate_message(&rule.message, &ctx),
                        fix_hint: rule.fix_hint.clone(),
                        location: location.clone(),
                    });
                }
            }
        }
    }

    // Extract and evaluate filters
    let filters = extract_filters(config);
    for (location, filter_json) in filters {
        let ctx = EvalContext {
            filter: Some(filter_json),
            ..base_ctx.clone()
        };

        for rule in &rules.rules {
            if rule.category.starts_with("filter") {
                if let Ok(true) = evaluate_condition(&rule.condition, &ctx) {
                    violations.push(RuleViolation {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        message: interpolate_message(&rule.message, &ctx),
                        fix_hint: rule.fix_hint.clone(),
                        location: location.clone(),
                    });
                }
            }
        }
    }

    // Extract and evaluate motion paths
    let motion_paths = extract_motion_paths(config);
    for (location, motion_path_info) in motion_paths {
        let ctx = EvalContext {
            motion_path: Some(motion_path_info),
            ..base_ctx.clone()
        };

        for rule in &rules.rules {
            if rule.category.starts_with("motion_path") {
                if let Ok(true) = evaluate_condition(&rule.condition, &ctx) {
                    violations.push(RuleViolation {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        message: interpolate_message(&rule.message, &ctx),
                        fix_hint: rule.fix_hint.clone(),
                        location: location.clone(),
                    });
                }
            }
        }
    }

    // Filter based on CLI flags
    let errors: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.severity, Severity::Error))
        .collect();
    let warnings: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.severity, Severity::Warning))
        .collect();

    // Build result
    if !errors.is_empty() || (args.strict && !warnings.is_empty()) {
        result = StageResult::fail("rules", format_violations(&violations, args));

        for violation in &violations {
            if args.errors_only && matches!(violation.severity, Severity::Warning) {
                continue;
            }
            result = result.with_message(format_violation(violation));
        }
    } else {
        result = result.with_message(format!("{} rules checked", rules.rules.len()));

        if !warnings.is_empty() && !args.errors_only {
            result = result.with_message(format!("{} warning(s) found", warnings.len()));
            for w in &warnings {
                result = result.with_detail(format!("WARN [{}]: {}", w.rule_id, w.message));
                if let Some(hint) = &w.fix_hint {
                    result = result.with_detail(format!("  hint: {}", hint));
                }
            }
        }
    }

    result
}

/// Build base evaluation context from RaRecipeConfig
fn build_base_context(config: &RaRecipeConfig) -> EvalContext {
    let width = config.layout.width;
    let height = config.layout.height;

    let recipe_json = serde_json::to_value(config).unwrap_or_else(|_| json!({}));

    // Extract anchor from layout
    let anchor = serde_json::to_value(&config.layout.anchor)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    // Extract time config
    let time = config.time.as_ref().map(|t| TimeConfig {
        loop_enabled: t.is_loop,
        loop_period_ms: Some(t.loop_period_ms),
    });

    // Extract style_base from first style layer
    let style_base = config.pipeline.styles.first().map(|style| StyleBase {
        fg: serde_json::to_value(&style.base_style.foreground).ok(),
        bg: serde_json::to_value(&style.base_style.background).ok(),
    });

    EvalContext {
        recipe: recipe_json,
        shader: None,
        mask: None,
        sampler: None,
        filter: None,
        effect: None,
        motion_path: None,
        layout: LayoutInfo {
            width,
            height,
            anchor,
        },
        style_base,
        time,
    }
}

/// Extract all shaders from the pipeline configuration
fn extract_shaders(config: &RaRecipeConfig) -> Vec<(String, Value)> {
    let mut shaders = Vec::new();

    if let Ok(json) = serde_json::to_value(config) {
        if let Some(pipeline) = json.get("pipeline") {
            // RaPipelineConfig serializes as "styles" (array) due to #[serde(rename = "styles")]
            if let Some(styles) = pipeline.get("styles").and_then(|s| s.as_array()) {
                for (i, style) in styles.iter().enumerate() {
                    // Check enter_effect.shader
                    if let Some(enter) = style.get("enter_effect") {
                        if !enter.is_null() {
                            if let Some(shader) = enter.get("shader") {
                                shaders.push((
                                    format!("pipeline.styles[{}].enter_effect.shader", i),
                                    shader.clone(),
                                ));
                            }
                        }
                    }
                    // Check dwell_effect.shader
                    if let Some(dwell) = style.get("dwell_effect") {
                        if !dwell.is_null() {
                            if let Some(shader) = dwell.get("shader") {
                                shaders.push((
                                    format!("pipeline.styles[{}].dwell_effect.shader", i),
                                    shader.clone(),
                                ));
                            }
                        }
                    }
                    // Check exit_effect.shader
                    if let Some(exit) = style.get("exit_effect") {
                        if !exit.is_null() {
                            if let Some(shader) = exit.get("shader") {
                                shaders.push((
                                    format!("pipeline.styles[{}].exit_effect.shader", i),
                                    shader.clone(),
                                ));
                            }
                        }
                    }
                    // Check legacy spatial_shader field (used by many existing recipes)
                    if let Some(spatial) = style.get("spatial_shader") {
                        if !spatial.is_null() {
                            shaders.push((
                                format!("pipeline.styles[{}].spatial_shader", i),
                                spatial.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }

    shaders
}

/// Extract all masks from the pipeline configuration
fn extract_masks(config: &RaRecipeConfig) -> Vec<(String, Value)> {
    let mut masks = Vec::new();

    if let Ok(json) = serde_json::to_value(config) {
        if let Some(pipeline) = json.get("pipeline") {
            if let Some(mask) = pipeline.get("mask") {
                if let Some(enter) = mask.get("enter") {
                    if enter.get("type").and_then(|t| t.as_str()) != Some("None") {
                        masks.push(("pipeline.mask.enter".to_string(), enter.clone()));
                    }
                }
                if let Some(exit) = mask.get("exit") {
                    if exit.get("type").and_then(|t| t.as_str()) != Some("None") {
                        masks.push(("pipeline.mask.exit".to_string(), exit.clone()));
                    }
                }
            }
        }
    }

    masks
}

/// Extract all samplers from the pipeline configuration
fn extract_samplers(config: &RaRecipeConfig) -> Vec<(String, Value)> {
    let mut samplers = Vec::new();

    if let Ok(json) = serde_json::to_value(config) {
        if let Some(pipeline) = json.get("pipeline") {
            if let Some(sampler) = pipeline.get("sampler") {
                if let Some(enter) = sampler.get("enter") {
                    if enter.get("type").and_then(|t| t.as_str()) != Some("None") {
                        samplers.push(("pipeline.sampler.enter".to_string(), enter.clone()));
                    }
                }
                if let Some(exit) = sampler.get("exit") {
                    if exit.get("type").and_then(|t| t.as_str()) != Some("None") {
                        samplers.push(("pipeline.sampler.exit".to_string(), exit.clone()));
                    }
                }
            }
        }
    }

    samplers
}

/// Extract all filters from the pipeline configuration
fn extract_filters(config: &RaRecipeConfig) -> Vec<(String, Value)> {
    let mut filters = Vec::new();

    if let Ok(json) = serde_json::to_value(config) {
        if let Some(pipeline) = json.get("pipeline") {
            if let Some(filter) = pipeline.get("filter") {
                if let Some(enter) = filter.get("enter") {
                    if enter.get("type").and_then(|t| t.as_str()) != Some("None") {
                        filters.push(("pipeline.filter.enter".to_string(), enter.clone()));
                    }
                }
                if let Some(exit) = filter.get("exit") {
                    if exit.get("type").and_then(|t| t.as_str()) != Some("None") {
                        filters.push(("pipeline.filter.exit".to_string(), exit.clone()));
                    }
                }
            }
        }
    }

    filters
}

/// Extract motion paths from enter/exit transitions with timing info
fn extract_motion_paths(config: &RaRecipeConfig) -> Vec<(String, MotionPathInfo)> {
    let mut paths = Vec::new();

    if let Ok(json) = serde_json::to_value(config) {
        if let Some(pipeline) = json.get("pipeline") {
            // Check enter transition
            if let Some(enter) = pipeline.get("enter") {
                if let Some(motion_path) = enter.get("motion_path") {
                    if !motion_path.is_null() {
                        let duration_ms = enter
                            .get("duration_ms")
                            .and_then(|d| d.as_u64())
                            .unwrap_or(500);
                        // Extract from direction (e.g., {"type": "offscreen", "direction": "from_top"})
                        let from_direction = enter
                            .get("from")
                            .and_then(|f| f.get("direction"))
                            .and_then(|d| d.as_str())
                            .map(|s| s.to_string());
                        // Extract to direction if specified
                        let to_direction = enter
                            .get("to")
                            .and_then(|t| t.get("direction"))
                            .and_then(|d| d.as_str())
                            .map(|s| s.to_string());
                        paths.push((
                            "pipeline.enter.motion_path".to_string(),
                            MotionPathInfo {
                                path: motion_path.clone(),
                                duration_ms,
                                phase: "enter".to_string(),
                                from_direction,
                                to_direction,
                            },
                        ));
                    }
                }
            }

            // Check exit transition
            if let Some(exit) = pipeline.get("exit") {
                if let Some(motion_path) = exit.get("motion_path") {
                    if !motion_path.is_null() {
                        let duration_ms = exit
                            .get("duration_ms")
                            .and_then(|d| d.as_u64())
                            .unwrap_or(500);
                        let from_direction = exit
                            .get("from")
                            .and_then(|f| f.get("direction"))
                            .and_then(|d| d.as_str())
                            .map(|s| s.to_string());
                        let to_direction = exit
                            .get("to")
                            .and_then(|t| t.get("direction"))
                            .and_then(|d| d.as_str())
                            .map(|s| s.to_string());
                        paths.push((
                            "pipeline.exit.motion_path".to_string(),
                            MotionPathInfo {
                                path: motion_path.clone(),
                                duration_ms,
                                phase: "exit".to_string(),
                                from_direction,
                                to_direction,
                            },
                        ));
                    }
                }
            }
        }
    }

    paths
}

/// Format a single violation for display
fn format_violation(violation: &RuleViolation) -> String {
    let severity_str = match violation.severity {
        Severity::Error => "ERROR",
        Severity::Warning => "WARN",
    };

    let mut msg = format!(
        "{} [{}]: {}",
        severity_str, violation.rule_id, violation.message
    );

    if let Some(hint) = &violation.fix_hint {
        msg.push_str(&format!(" (hint: {})", hint));
    }

    msg
}

/// Format all violations into a summary message
fn format_violations(violations: &[RuleViolation], args: &Args) -> String {
    let errors: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.severity, Severity::Error))
        .collect();
    let warnings: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.severity, Severity::Warning))
        .collect();

    let mut parts = Vec::new();

    if !errors.is_empty() {
        parts.push(format!("{} error(s)", errors.len()));
    }

    if !warnings.is_empty() && !args.errors_only {
        parts.push(format!("{} warning(s)", warnings.len()));
    }

    format!("Rules validation failed: {}", parts.join(", "))
}

// <FILE>tools/pipeline-validator/src/stages/rules.rs</FILE> - <DESC>Rules stage: visibility rules validation</DESC>
// <VERS>END OF VERSION: 0.8.0</VERS>
