// <FILE>src/types/stacking_policy.rs</FILE> - <DESC>Stacking policy enum for layout modes</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>Cell Padding PRD: Configurable stacking spacing</WCTX>
// <CLOG>BREAKING: Add spacing fields to Vertical/Horizontal variants (was unit variants)

use serde::{Deserialize, Serialize};

/// Defines how multiple items at the same anchor are arranged spatially
///
/// When multiple notifications or animations are anchored to the same position,
/// the StackingPolicy determines their spatial arrangement. This enables use cases
/// like vertical toast stacks, horizontal status bars, grid-based icon docks, or
/// single-item displays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[non_exhaustive]
pub enum StackingPolicy {
    /// Stack items vertically
    ///
    /// Direction determined by anchor position:
    /// - Bottom anchors (BottomLeft, BottomCenter, BottomRight): stack upward
    /// - Top anchors (TopLeft, TopCenter, TopRight): stack downward
    /// - Middle anchors: stack downward by default
    ///
    /// # Fields
    /// - `spacing`: Gap between stacked items in cells (default: 1)
    Vertical {
        /// Gap between vertically stacked items (in cells)
        spacing: u16,
    },

    /// Stack items horizontally
    ///
    /// Direction determined by anchor position:
    /// - Right anchors (TopRight, MiddleRight, BottomRight): stack left-to-right
    /// - Left anchors (TopLeft, MiddleLeft, BottomLeft): stack right-to-left
    /// - Center anchors: stack right-to-left by default
    ///
    /// # Fields
    /// - `spacing`: Gap between stacked items in cells (default: 1)
    Horizontal {
        /// Gap between horizontally stacked items (in cells)
        spacing: u16,
    },

    /// Arrange items in a grid (rows × columns)
    ///
    /// Items are arranged in rows with automatic wrapping after `max_columns`.
    /// Grid origin respects anchor position (TopLeft vs BottomRight affects layout direction).
    ///
    /// # Fields
    /// - `max_columns`: Maximum number of columns before wrapping to new row
    /// - `row_spacing`: Vertical spacing between rows (in cells)
    /// - `column_spacing`: Horizontal spacing between columns (in cells)
    Grid {
        max_columns: u16,
        row_spacing: u16,
        column_spacing: u16,
    },

    /// No stacking - only show the first (oldest) item at each anchor
    ///
    /// Additional items at the same anchor remain in state but are not rendered.
    /// Useful for single critical alerts or exclusive notification zones.
    None,
}

impl Default for StackingPolicy {
    fn default() -> Self {
        Self::Vertical { spacing: 1 }
    }
}

impl StackingPolicy {
    /// Create a vertical stacking policy with default spacing (1 cell)
    ///
    /// This is the standard notification toast stack arrangement.
    pub fn vertical() -> Self {
        Self::Vertical { spacing: 1 }
    }

    /// Create a vertical stacking policy with custom spacing
    ///
    /// # Arguments
    /// * `spacing` - Gap between vertically stacked items (in cells)
    pub fn vertical_with_spacing(spacing: u16) -> Self {
        Self::Vertical { spacing }
    }

    /// Create a horizontal stacking policy with default spacing (1 cell)
    ///
    /// Useful for status bars and side-by-side displays.
    pub fn horizontal() -> Self {
        Self::Horizontal { spacing: 1 }
    }

    /// Create a horizontal stacking policy with custom spacing
    ///
    /// # Arguments
    /// * `spacing` - Gap between horizontally stacked items (in cells)
    pub fn horizontal_with_spacing(spacing: u16) -> Self {
        Self::Horizontal { spacing }
    }

    /// Common grid preset: 2 columns with standard spacing
    ///
    /// Creates a 2-column grid with 1px row spacing and 2px column spacing.
    /// Suitable for side-by-side item displays.
    pub fn grid_2x2() -> Self {
        Self::Grid {
            max_columns: 2,
            row_spacing: 1,
            column_spacing: 2,
        }
    }

    /// Common grid preset: 3 columns with standard spacing
    ///
    /// Creates a 3-column grid with 1px row spacing and 2px column spacing.
    /// Suitable for icon docks and multi-column layouts.
    pub fn grid_3x3() -> Self {
        Self::Grid {
            max_columns: 3,
            row_spacing: 1,
            column_spacing: 2,
        }
    }

    /// Common pattern for horizontal status icons
    ///
    /// Alias for `StackingPolicy::horizontal()`.
    /// Semantic convenience method for status bar use cases.
    pub fn status_bar() -> Self {
        Self::horizontal()
    }
}

// <FILE>src/types/stacking_policy.rs</FILE> - <DESC>Stacking policy enum for layout modes</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
