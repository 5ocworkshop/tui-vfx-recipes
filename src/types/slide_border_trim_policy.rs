// <FILE>src/types/slide_border_trim_policy.rs</FILE> - <DESC>Moved from ratatui-notifications</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-18T14:30:00Z - 2025-12-18T14:26:28Z</VERS>
// <WCTX>Refactor: Framework Extraction</WCTX>
// <CLOG>Moved from domain crate</CLOG>

/// Controls how borders behave when a sliding item is partially clipped by the viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema)]
#[non_exhaustive]
pub enum SlideBorderTrimPolicy {
    /// Do not alter the border symbols.
    #[default]
    None,
    /// Remove the border on the clipped edge (legacy "vanishing edge" illusion).
    VanishingEdge,
}

// <FILE>src/types/slide_border_trim_policy.rs</FILE> - <DESC>Moved from ratatui-notifications</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-18T14:30:00Z - 2025-12-18T14:26:28Z</VERS>
