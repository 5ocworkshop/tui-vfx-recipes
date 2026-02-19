// <FILE>src/types/auto_dismiss.rs</FILE> - <DESC>AutoDismiss enum (Moved from notifications)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-18T13:15:00Z - 2025-12-18T13:13:25Z</VERS>
// <WCTX>V2 Migration: Framework Extraction</WCTX>
// <CLOG>Ported from ratatui-notifications</CLOG>

use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoDismiss {
    Manual,
    After(Duration),
}
impl Default for AutoDismiss {
    fn default() -> Self {
        Self::After(Duration::from_secs(4))
    }
}

// <FILE>src/types/auto_dismiss.rs</FILE> - <DESC>AutoDismiss enum (Moved from notifications)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-18T13:15:00Z - 2025-12-18T13:13:25Z</VERS>
