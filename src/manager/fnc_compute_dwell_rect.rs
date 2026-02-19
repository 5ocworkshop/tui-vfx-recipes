// <FILE>src/manager/fnc_compute_dwell_rect.rs</FILE> - <DESC>Compute dwell rectangle with exterior margin</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Clippy cleanup for dwell rect computation</WCTX>
// <CLOG>Allow multi-argument dwell rect helper signature</CLOG>

use crate::manager::fnc_resolve_anchor::resolve_anchor;
use ratatui::layout::Rect;
use tui_vfx_geometry::types::{Anchor, SignedRect};

/// Compute the dwell rectangle for an item at the given anchor position.
///
/// This combines anchor-based positioning with exterior margin application.
/// The margin pushes the rect inward from the frame edges based on anchor position.
#[allow(clippy::too_many_arguments)]
pub fn compute_dwell_rect(
    anchor: Anchor,
    offset_h_pct: f32,
    offset_v_pct: f32,
    offset_h_cells: i16,
    offset_v_cells: i16,
    offset_h_pixels: i32,
    offset_v_pixels: i32,
    frame_area: Rect,
    width: u16,
    height: u16,
    margin: u16,
) -> SignedRect {
    let base = resolve_anchor(
        anchor,
        offset_h_pct,
        offset_v_pct,
        offset_h_cells,
        offset_v_cells,
        offset_h_pixels,
        offset_v_pixels,
        frame_area,
        width,
        height,
    );
    apply_exterior_margin(anchor, base, margin)
}

/// Apply exterior margin to a rect based on anchor position.
///
/// The margin direction depends on the anchor:
/// - TopLeft: push right and down (+x, +y)
/// - TopRight: push left and down (-x, +y)
/// - BottomLeft: push right and up (+x, -y)
/// - BottomRight: push left and up (-x, -y)
/// - Center anchors: only push perpendicular to edge
fn apply_exterior_margin(anchor: Anchor, rect: SignedRect, margin: u16) -> SignedRect {
    let m = margin as i32;
    let (dx, dy) = match anchor {
        Anchor::TopLeft => (m, m),
        Anchor::TopCenter => (0, m),
        Anchor::TopRight => (-m, m),
        Anchor::MiddleLeft => (m, 0),
        Anchor::Center => (0, 0),
        Anchor::MiddleRight => (-m, 0),
        Anchor::BottomLeft => (m, -m),
        Anchor::BottomCenter => (0, -m),
        Anchor::BottomRight => (-m, -m),
        _ => (0, 0),
    };
    SignedRect::new(rect.x + dx, rect.y + dy, rect.width, rect.height)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> Rect {
        Rect::new(0, 0, 100, 50)
    }

    /// Test that margin is correctly applied by comparing with zero margin.
    /// This verifies the DELTA from margin application, not absolute positions.
    #[test]
    fn test_top_left_margin_delta() {
        let no_margin =
            compute_dwell_rect(Anchor::TopLeft, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 0);
        let with_margin =
            compute_dwell_rect(Anchor::TopLeft, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 5);
        // TopLeft pushes right (+5) and down (+5)
        assert_eq!(with_margin.x - no_margin.x, 5);
        assert_eq!(with_margin.y - no_margin.y, 5);
        assert_eq!(with_margin.width, no_margin.width);
        assert_eq!(with_margin.height, no_margin.height);
    }

    #[test]
    fn test_bottom_right_margin_delta() {
        let no_margin = compute_dwell_rect(
            Anchor::BottomRight,
            0.0,
            0.0,
            0,
            0,
            0,
            0,
            frame(),
            20,
            10,
            0,
        );
        let with_margin = compute_dwell_rect(
            Anchor::BottomRight,
            0.0,
            0.0,
            0,
            0,
            0,
            0,
            frame(),
            20,
            10,
            5,
        );
        // BottomRight pushes left (-5) and up (-5)
        assert_eq!(with_margin.x - no_margin.x, -5);
        assert_eq!(with_margin.y - no_margin.y, -5);
    }

    #[test]
    fn test_center_anchor_no_margin_effect() {
        let no_margin =
            compute_dwell_rect(Anchor::Center, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 0);
        let with_margin =
            compute_dwell_rect(Anchor::Center, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 5);
        // Center: margin has no effect (0, 0)
        assert_eq!(with_margin.x, no_margin.x);
        assert_eq!(with_margin.y, no_margin.y);
    }

    #[test]
    fn test_zero_margin_preserves_base() {
        let result = compute_dwell_rect(Anchor::TopLeft, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 0);
        // TopLeft with zero margin should be at origin
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
        assert_eq!(result.width, 20);
        assert_eq!(result.height, 10);
    }

    #[test]
    fn test_top_center_only_vertical_margin() {
        let no_margin =
            compute_dwell_rect(Anchor::TopCenter, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 0);
        let with_margin =
            compute_dwell_rect(Anchor::TopCenter, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 5);
        // TopCenter: only vertical margin (+5 y), no horizontal
        assert_eq!(with_margin.x, no_margin.x); // No horizontal offset
        assert_eq!(with_margin.y - no_margin.y, 5);
    }

    #[test]
    fn test_middle_left_only_horizontal_margin() {
        let no_margin =
            compute_dwell_rect(Anchor::MiddleLeft, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 0);
        let with_margin =
            compute_dwell_rect(Anchor::MiddleLeft, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 5);
        // MiddleLeft: only horizontal margin (+5 x), no vertical
        assert_eq!(with_margin.x - no_margin.x, 5);
        assert_eq!(with_margin.y, no_margin.y); // No vertical offset
    }

    #[test]
    fn test_middle_right_margin_delta() {
        let no_margin = compute_dwell_rect(
            Anchor::MiddleRight,
            0.0,
            0.0,
            0,
            0,
            0,
            0,
            frame(),
            20,
            10,
            0,
        );
        let with_margin = compute_dwell_rect(
            Anchor::MiddleRight,
            0.0,
            0.0,
            0,
            0,
            0,
            0,
            frame(),
            20,
            10,
            5,
        );
        // MiddleRight: horizontal margin pushes left (-5), no vertical
        assert_eq!(with_margin.x - no_margin.x, -5);
        assert_eq!(with_margin.y, no_margin.y);
    }

    #[test]
    fn test_bottom_left_margin_delta() {
        let no_margin =
            compute_dwell_rect(Anchor::BottomLeft, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 0);
        let with_margin =
            compute_dwell_rect(Anchor::BottomLeft, 0.0, 0.0, 0, 0, 0, 0, frame(), 20, 10, 5);
        // BottomLeft: push right (+5) and up (-5)
        assert_eq!(with_margin.x - no_margin.x, 5);
        assert_eq!(with_margin.y - no_margin.y, -5);
    }

    #[test]
    fn test_bottom_center_margin_delta() {
        let no_margin = compute_dwell_rect(
            Anchor::BottomCenter,
            0.0,
            0.0,
            0,
            0,
            0,
            0,
            frame(),
            20,
            10,
            0,
        );
        let with_margin = compute_dwell_rect(
            Anchor::BottomCenter,
            0.0,
            0.0,
            0,
            0,
            0,
            0,
            frame(),
            20,
            10,
            5,
        );
        // BottomCenter: only vertical margin (-5 y), no horizontal
        assert_eq!(with_margin.x, no_margin.x);
        assert_eq!(with_margin.y - no_margin.y, -5);
    }
}

// <FILE>src/manager/fnc_compute_dwell_rect.rs</FILE> - <DESC>Compute dwell rectangle with exterior margin</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
