// <FILE>src/types/animation_profile.rs</FILE> - <DESC>AnimationProfile with optional MotionSpec</DESC>
// <VERS>VERSION: 3.0.0</VERS>
// <WCTX>Multi-layer style support</WCTX>
// <CLOG>Added style_layers Vec for multi-region styling; deprecated single-effect fields</CLOG>

use serde::{Deserialize, Serialize};
use std::time::Duration;
use tui_vfx_geometry::types::{MotionSpec, TransitionSpec};
use tui_vfx_style::models::{StyleEffect, StyleLayer, StyleRegion};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationProfile {
    /// Enter transition spec (used for Slide, ExpandCollapse, Fade animations).
    pub enter: TransitionSpec,
    /// Exit transition spec (used for Slide, ExpandCollapse, Fade animations).
    pub exit: TransitionSpec,
    /// Optional MotionSpec for enter animation (used when Animation::Motion).
    /// Provides from/via/to waypoints for arbitrary motion paths.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter_motion: Option<MotionSpec>,
    /// Optional MotionSpec for exit animation (used when Animation::Motion).
    /// If None, enter_motion is used in reverse.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_motion: Option<MotionSpec>,

    // =========================================================================
    // Multi-layer style support (new)
    // =========================================================================
    /// Style layers - each layer can target different regions with different effects.
    /// Layers are processed in order; later layers override earlier ones for overlapping cells.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style_layers: Vec<StyleLayer>,

    // =========================================================================
    // Legacy single-effect fields (deprecated but preserved for compatibility)
    // =========================================================================
    /// Optional style effect applied during Entering phase
    /// DEPRECATED: Use style_layers instead for multi-region support
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter_style: Option<StyleEffect>,
    /// Optional style effect applied during Dwelling phase
    /// DEPRECATED: Use style_layers instead for multi-region support
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dwell_style: Option<StyleEffect>,
    /// Optional style effect applied during Exiting phase
    /// DEPRECATED: Use style_layers instead for multi-region support
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_style: Option<StyleEffect>,
    /// Region constraint for shader effects (TextOnly, BorderOnly, Rows, etc.)
    /// DEPRECATED: Use style_layers instead for multi-region support
    #[serde(default)]
    pub shader_region: StyleRegion,

    // =========================================================================
    // Timing
    // =========================================================================
    /// Period for loop_t cycling (for continuous pulsing/oscillating effects).
    /// Default is 2 seconds. Set to None or Duration::ZERO to disable looping.
    #[serde(
        default = "default_loop_period",
        with = "duration_millis",
        skip_serializing_if = "Option::is_none"
    )]
    pub loop_period: Option<Duration>,
}

impl AnimationProfile {
    /// Get effective style layers, converting legacy fields if style_layers is empty.
    /// This provides backwards compatibility with code using the old single-effect fields.
    pub fn effective_style_layers(&self) -> Vec<StyleLayer> {
        if !self.style_layers.is_empty() {
            return self.style_layers.clone();
        }

        // Convert legacy fields to a single layer
        if self.enter_style.is_some() || self.dwell_style.is_some() || self.exit_style.is_some() {
            vec![StyleLayer {
                region: self.shader_region.clone(),
                enter_effect: self.enter_style.clone(),
                enter_region: None,
                dwell_effect: self.dwell_style.clone(),
                dwell_region: None,
                exit_effect: self.exit_style.clone(),
                exit_region: None,
            }]
        } else {
            Vec::new()
        }
    }
}

fn default_loop_period() -> Option<Duration> {
    Some(Duration::from_secs(2))
}

mod duration_millis {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match duration {
            Some(d) => serializer.serialize_some(&d.as_millis()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis: Option<u64> = Option::deserialize(deserializer)?;
        Ok(millis.map(Duration::from_millis))
    }
}

impl Default for AnimationProfile {
    fn default() -> Self {
        Self {
            enter: TransitionSpec::default(),
            exit: TransitionSpec {
                duration_ms: 750,
                ..TransitionSpec::default()
            },
            enter_motion: None,
            exit_motion: None,
            style_layers: Vec::new(),
            enter_style: None,
            dwell_style: None,
            exit_style: None,
            shader_region: StyleRegion::All,
            loop_period: default_loop_period(),
        }
    }
}

// <FILE>src/types/animation_profile.rs</FILE> - <DESC>AnimationProfile with optional MotionSpec</DESC>
// <VERS>END OF VERSION: 3.0.0</VERS>
