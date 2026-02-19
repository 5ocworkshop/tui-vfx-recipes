// <FILE>src/types/overflow_policy.rs</FILE> - <DESC>OverflowPolicy enum (Moved from notifications)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-18T13:15:00Z - 2025-12-18T13:13:25Z</VERS>
// <WCTX>V2 Migration: Framework Extraction</WCTX>
// <CLOG>Ported from ratatui-notifications</CLOG>

use serde::{Deserialize, Serialize};
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum OverflowPolicy {
    /// When the per-anchor cap is exceeded, drop the oldest item at that anchor.
    #[default]
    DiscardOldest,
    /// When the per-anchor cap is exceeded, drop the newest existing item at that anchor
    /// to make room for the incoming item.
    DiscardNewest,
    /// When the per-anchor cap is exceeded, refuse to add the new item.
    Reject,
}

// <FILE>src/types/overflow_policy.rs</FILE> - <DESC>OverflowPolicy enum (Moved from notifications)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-18T13:15:00Z - 2025-12-18T13:13:25Z</VERS>
