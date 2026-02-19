// <FILE>src/recipe/embedded/fnc_load_config_hybrid.rs</FILE> - <DESC>Hybrid generic config loading: filesystem with embedded fallback</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Embedded recipe library extraction</WCTX>
// <CLOG>Initial implementation of load_config_hybrid function</CLOG>

//! Load generic JSON configs with hybrid strategy: try filesystem, fall back to embedded.

use serde::de::DeserializeOwned;
use std::path::Path;

/// Error type for generic config loading.
#[derive(Debug, Clone)]
pub enum ConfigLoadError {
    /// I/O error reading runtime file
    IoError(String),
    /// JSON parse error
    ParseError(String),
}

impl std::fmt::Display for ConfigLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigLoadError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ConfigLoadError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigLoadError {}

/// Load a generic JSON config with hybrid strategy.
///
/// Strategy:
/// 1. If `runtime_path` is Some and the file exists, load from filesystem
/// 2. Otherwise, parse from `embedded_json`
///
/// # Type Parameters
/// * `T` - The config type to deserialize into (must implement `DeserializeOwned`)
///
/// # Arguments
/// * `runtime_path` - Optional path to check for runtime override file
/// * `embedded_json` - The embedded JSON string to use as fallback
/// * `name` - Name for error messages (e.g., "popup_animation")
///
/// # Returns
/// The deserialized config
///
/// # Errors
/// Returns `ConfigLoadError` if:
/// - Runtime file exists but fails to parse, AND embedded also fails
/// - Embedded JSON fails to parse
///
/// # Example
/// ```ignore
/// use tui_vfx_recipes::recipe::load_config_hybrid;
/// use serde::Deserialize;
/// use std::path::Path;
///
/// #[derive(Deserialize)]
/// struct AnimationConfig {
///     duration_ms: u32,
///     easing: String,
/// }
///
/// const EMBEDDED: &str = include_str!("../config/animation.json");
///
/// let config: AnimationConfig = load_config_hybrid(
///     Some(Path::new("config/animation.json")),
///     EMBEDDED,
///     "animation",
/// )?;
/// ```
pub fn load_config_hybrid<T: DeserializeOwned>(
    runtime_path: Option<&Path>,
    embedded_json: &str,
    name: &str,
) -> Result<T, ConfigLoadError> {
    // Try runtime file first (if provided and exists)
    if let Some(path) = runtime_path {
        if path.exists() {
            match std::fs::read_to_string(path) {
                Ok(json) => match serde_json::from_str(&json) {
                    Ok(config) => {
                        #[cfg(debug_assertions)]
                        eprintln!("[embedded] Loaded {} from filesystem", name);
                        return Ok(config);
                    }
                    Err(e) => {
                        eprintln!(
                            "[embedded] Warning: Failed to parse {} from {:?}, using embedded: {}",
                            name, path, e
                        );
                    }
                },
                Err(e) => {
                    eprintln!(
                        "[embedded] Warning: Failed to read {} from {:?}, using embedded: {}",
                        name, path, e
                    );
                }
            }
        }
    }

    // Fall back to embedded
    serde_json::from_str(embedded_json).map_err(|e| {
        ConfigLoadError::ParseError(format!("Failed to parse embedded {}: {}", name, e))
    })
}

// <FILE>src/recipe/embedded/fnc_load_config_hybrid.rs</FILE> - <DESC>Hybrid generic config loading: filesystem with embedded fallback</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
