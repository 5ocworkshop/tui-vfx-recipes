// <FILE>src/interactions.rs</FILE> - <DESC>Interaction state types for recipe system</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>TUI VFX recipes extraction - interaction state stubs</WCTX>
// <CLOG>Initial creation - stub types for interaction state management</CLOG>

//! Interaction state types for the recipe system.
//!
//! These types define how interactive elements respond to state changes
//! (hover, focus, active, etc.) and how those states compose.

use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;

/// Geometry overrides for interaction states.
///
/// Allows modifying visual geometry (bar widths, outlines) per state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GeometryOverrides {
    /// Width of accent bar (e.g., left edge highlight)
    pub accent_bar_width: Option<u16>,

    /// Thickness of outline/border
    pub outline_thickness: Option<u16>,

    /// Color of outline/border
    pub outline_color: Option<Color>,

    /// Character override (e.g., "▌" → "█" for bar width animation)
    pub character: Option<char>,
}

/// Style configuration for a specific interaction state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StateStyleConfig {
    /// Background color for this state
    pub background: Option<Color>,

    /// Foreground color for this state
    pub foreground: Option<Color>,

    /// Text modifiers for this state
    pub modifiers: Option<tui_vfx_types::Modifiers>,

    /// Opacity for this state (0.0 = transparent, 1.0 = opaque)
    pub opacity: Option<f32>,

    /// Geometry overrides for this state
    #[serde(default)]
    pub geometry: GeometryOverrides,
}

/// How multiple interaction states compose together.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum StateCompositionMode {
    /// States layer on top of each other (hover + focus both visible)
    #[default]
    Layered,

    /// Only the highest-priority state is visible
    Exclusive,

    /// States blend together based on their priorities
    Blended,
}

// <FILE>src/interactions.rs</FILE> - <DESC>Interaction state types for recipe system</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
