// <FILE>tools/pipeline-validator/src/rules/fnc_load_rules.rs</FILE> - <DESC>TOML rule loader for visibility rules</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Pipeline debugging tools - color contrast validation</WCTX>
// <CLOG>Embed default rules in binary for portable execution</CLOG>

use std::fs;
use std::path::Path;

use crate::types::{Rule, RuleSet};

/// Default visibility rules embedded in binary
const EMBEDDED_RULES: &str = include_str!("../../rules/visibility.toml");

/// Load rules from a TOML file.
pub fn load_rules_from_file(path: &Path) -> Result<RuleSet, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read rules file '{}': {}", path.display(), e))?;

    parse_rules_toml(&content)
        .map_err(|e| format!("Failed to parse rules file '{}': {}", path.display(), e))
}

/// Load default visibility rules (embedded in binary).
pub fn load_default_rules() -> Result<RuleSet, String> {
    parse_rules_toml(EMBEDDED_RULES).map_err(|e| format!("Failed to parse embedded rules: {}", e))
}

/// Parse TOML content into a RuleSet.
fn parse_rules_toml(content: &str) -> Result<RuleSet, String> {
    #[derive(serde::Deserialize)]
    struct TomlRuleSet {
        meta: TomlMeta,
        rules: Vec<Rule>,
    }

    #[derive(serde::Deserialize)]
    struct TomlMeta {
        version: String,
        description: String,
    }

    let toml_ruleset: TomlRuleSet =
        toml::from_str(content).map_err(|e| format!("TOML parse error: {}", e))?;

    // Validate that all rules have required fields
    for rule in &toml_ruleset.rules {
        if rule.id.is_empty() {
            return Err("Rule with empty 'id' field found".to_string());
        }
        if rule.condition.is_empty() {
            return Err(format!("Rule '{}' has empty 'condition' field", rule.id));
        }
    }

    Ok(RuleSet {
        version: toml_ruleset.meta.version,
        description: toml_ruleset.meta.description,
        rules: toml_ruleset.rules,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_toml() {
        let toml = r#"
[meta]
version = "1.0.0"
description = "Test rules"

[[rules]]
id = "test-rule"
severity = "error"
category = "test"
description = "A test rule"
condition = "x > 5"
message = "Value too high"
"#;

        let ruleset = parse_rules_toml(toml).expect("Should parse valid TOML");
        assert_eq!(ruleset.version, "1.0.0");
        assert_eq!(ruleset.rules.len(), 1);
        assert_eq!(ruleset.rules[0].id, "test-rule");
    }

    #[test]
    fn test_parse_empty_id_error() {
        let toml = r#"
[meta]
version = "1.0.0"
description = "Test"

[[rules]]
id = ""
severity = "warning"
category = "test"
description = "Bad rule"
condition = "true"
message = "Test"
"#;

        let result = parse_rules_toml(toml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty 'id'"));
    }
}

// <FILE>tools/pipeline-validator/src/rules/fnc_load_rules.rs</FILE> - <DESC>TOML rule loader for visibility rules</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
