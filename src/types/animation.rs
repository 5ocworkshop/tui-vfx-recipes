// <FILE>src/types/animation.rs</FILE> - <DESC>Animation enum with Motion variant</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-18T22:15:00Z</VERS>
// <WCTX>Connecting MotionSpec to render planning phase</WCTX>
// <CLOG>Added Motion variant for arbitrary waypoint animations via MotionSpec</CLOG>

use serde::{Deserialize, Serialize};
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum Animation {
    /// Slide in/out (legacy default).
    #[default]
    Slide,
    /// Expand/collapse in/out.
    ExpandCollapse,
    /// Fade in/out (rect unchanged).
    Fade,
    /// Custom motion using MotionSpec with from/via/to waypoints.
    /// Requires a MotionSpec to be set in the AnimationProfile.
    Motion,
    /// No animation.
    None,
}

// <FILE>src/types/animation.rs</FILE> - <DESC>Animation enum with Motion variant</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-18T22:15:00Z</VERS>
