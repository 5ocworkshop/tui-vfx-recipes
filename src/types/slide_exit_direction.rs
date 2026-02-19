// <FILE>src/types/slide_exit_direction.rs</FILE> - <DESC>Moved from ratatui-notifications</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-18T14:30:00Z - 2025-12-18T14:26:28Z</VERS>
// <WCTX>Refactor: Framework Extraction</WCTX>
// <CLOG>Moved from domain crate</CLOG>

use tui_vfx_geometry::types::SlideDirection;
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[non_exhaustive]
pub enum SlideExitDirection {
    /// Use the same direction as the enter slide.
    #[default]
    SameAsEnter,
    /// Explicitly override the exit direction.
    Direction(SlideDirection),
}
impl SlideExitDirection {
    pub fn resolve(self, enter: SlideDirection) -> SlideDirection {
        match self {
            SlideExitDirection::SameAsEnter => enter,
            SlideExitDirection::Direction(d) => d,
        }
    }
}

// <FILE>src/types/slide_exit_direction.rs</FILE> - <DESC>Moved from ratatui-notifications</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-18T14:30:00Z - 2025-12-18T14:26:28Z</VERS>
