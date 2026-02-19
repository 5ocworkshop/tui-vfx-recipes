// <FILE>src/inspector/cls_inspector_context.rs</FILE> - <DESC>InspectorContext for carrying inspector through pipeline</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools implementation</WCTX>
// <CLOG>Initial creation of InspectorContext and inspect! macro</CLOG>

use super::traits::PipelineInspector;

/// Context that carries an optional inspector through the render pipeline.
///
/// In production mode, use `InspectorContext::none()` for zero overhead.
/// In debug mode, use `InspectorContext::with_inspector()` to enable tracing.
pub struct InspectorContext {
    inspector: Option<Box<dyn PipelineInspector>>,
}

impl InspectorContext {
    /// Create a context without an inspector (production mode).
    pub fn none() -> Self {
        Self { inspector: None }
    }

    /// Create a context with an inspector (debug mode).
    pub fn with_inspector(inspector: Box<dyn PipelineInspector>) -> Self {
        Self {
            inspector: Some(inspector),
        }
    }

    /// Check if an inspector is active.
    pub fn is_active(&self) -> bool {
        self.inspector.is_some()
    }

    /// Get mutable access to the inspector, if present.
    pub fn inspector_mut(&mut self) -> Option<&mut Box<dyn PipelineInspector>> {
        self.inspector.as_mut()
    }

    /// Take the inspector out of the context for result extraction.
    /// This consumes the context and returns the inspector.
    pub fn take_inspector(self) -> Option<Box<dyn PipelineInspector>> {
        self.inspector
    }
}

impl Default for InspectorContext {
    fn default() -> Self {
        Self::none()
    }
}

/// Macro for calling inspector methods when an inspector is present.
///
/// Usage:
/// ```ignore
/// inspect!(ctx, on_config_parsed, &config);
/// inspect!(ctx, on_phase_entered, AnimationPhase::Entering);
/// ```
///
/// Expands to:
/// ```ignore
/// if let Some(inspector) = ctx.inspector_mut() {
///     inspector.on_config_parsed(&config);
/// }
/// ```
#[macro_export]
macro_rules! inspect {
    ($ctx:expr, $method:ident $(, $arg:expr)*) => {
        if let Some(inspector) = $ctx.inspector_mut() {
            inspector.$method($($arg),*);
        }
    };
}

// <FILE>src/inspector/cls_inspector_context.rs</FILE> - <DESC>InspectorContext for carrying inspector through pipeline</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
