// <FILE>tools/pipeline-validator/src/types/cls_eval_context.rs</FILE> - <DESC>Evaluation context for rule conditions</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>Pipeline debugging tools - physics/anchor compatibility validation</WCTX>
// <CLOG>Add anchor to LayoutInfo; add from_direction/to_direction to MotionPathInfo for position compatibility rules</CLOG>

use serde_json::Value;

/// Context for evaluating rules - contains the recipe data
#[derive(Debug, Clone)]
pub struct EvalContext {
    /// The full recipe JSON
    #[allow(dead_code)]
    pub recipe: Value,
    /// Current shader being evaluated (if any)
    pub shader: Option<Value>,
    /// Current mask being evaluated (if any)
    pub mask: Option<Value>,
    /// Current sampler being evaluated (if any)
    pub sampler: Option<Value>,
    /// Current filter being evaluated (if any)
    pub filter: Option<Value>,
    /// Current effect being evaluated (if any)
    pub effect: Option<Value>,
    /// Current motion_path being evaluated (if any)
    pub motion_path: Option<MotionPathInfo>,
    /// Layout dimensions
    pub layout: LayoutInfo,
    /// Style base colors (fg, bg)
    pub style_base: Option<StyleBase>,
    /// Time configuration
    #[allow(dead_code)]
    pub time: Option<TimeConfig>,
}

/// Motion path info for physics timing validation
#[derive(Debug, Clone)]
pub struct MotionPathInfo {
    /// The motion_path JSON object
    pub path: Value,
    /// Duration of the transition in milliseconds
    pub duration_ms: u64,
    /// Phase (enter or exit)
    pub phase: String,
    /// The from direction (e.g., "from_top", "from_bottom") if offscreen
    pub from_direction: Option<String>,
    /// The to direction (e.g., "from_top", "from_bottom") if offscreen
    pub to_direction: Option<String>,
}

/// Layout dimensions for rule evaluation
#[derive(Debug, Clone)]
pub struct LayoutInfo {
    pub width: u16,
    pub height: u16,
    /// Anchor position (e.g., "top_left", "center", "bottom_right")
    pub anchor: Option<String>,
}

/// Style base colors for contrast checking
#[derive(Debug, Clone)]
pub struct StyleBase {
    pub fg: Option<Value>,
    pub bg: Option<Value>,
}

/// Time configuration for loop-related rules
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TimeConfig {
    pub loop_enabled: bool,
    pub loop_period_ms: Option<u64>,
}

impl Default for EvalContext {
    fn default() -> Self {
        Self {
            recipe: Value::Null,
            shader: None,
            mask: None,
            sampler: None,
            filter: None,
            effect: None,
            motion_path: None,
            layout: LayoutInfo {
                width: 80,
                height: 24,
                anchor: None,
            },
            style_base: None,
            time: None,
        }
    }
}

// <FILE>tools/pipeline-validator/src/types/cls_eval_context.rs</FILE> - <DESC>Evaluation context for rule conditions</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
