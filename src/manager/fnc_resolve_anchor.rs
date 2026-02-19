// <FILE>src/manager/fnc_resolve_anchor.rs</FILE> - <DESC>Resolve anchor and offsets to concrete coordinates</DESC>
// <VERS>VERSION: 0.2.2 - 2026-01-22</VERS>
// <WCTX>Clippy cleanup for anchor resolution helpers</WCTX>
// <CLOG>Allow multi-argument anchor resolution signature</CLOG>

use crate::compat::ratatui_rect_to_vfx;
use ratatui::layout::Rect;
use tui_vfx_geometry::anchors::anchored_rect;
use tui_vfx_geometry::types::{Anchor, SignedRect};

/// Resolve an anchor and its multiple additive offsets into a concrete SignedRect.
#[allow(clippy::too_many_arguments)]
pub fn resolve_anchor(
    anchor: Anchor,
    offset_h_percent: f32,
    offset_v_percent: f32,
    offset_h_cells: i16,
    offset_v_cells: i16,
    _offset_h_pixels: i32, // Reserved for future Sixel/Graphics scale factor
    _offset_v_pixels: i32, // Reserved for future Sixel/Graphics scale factor
    frame_area: Rect,
    width: u16,
    height: u16,
) -> SignedRect {
    match anchor {
        Anchor::Absolute(x, y) => {
            // Absolute position uses fixed base + cell offset
            SignedRect::new(
                x as i32 + offset_h_cells as i32,
                y as i32 + offset_v_cells as i32,
                width,
                height,
            )
        }
        _ => {
            let vfx_frame = ratatui_rect_to_vfx(frame_area);
            let mut rect = anchored_rect(anchor, vfx_frame, width, height);

            // 1. Apply percentage offsets (relative to frame dimensions)
            let off_h_pct = (offset_h_percent / 100.0 * frame_area.width as f32).round() as i32;
            let off_v_pct = (offset_v_percent / 100.0 * frame_area.height as f32).round() as i32;

            // 2. Add cell offsets directly
            let total_off_h = off_h_pct + offset_h_cells as i32;
            let total_off_v = off_v_pct + offset_v_cells as i32;

            // Direction of offset depends on anchor (moving "inward" from the edge)
            rect.x += match anchor {
                Anchor::TopLeft | Anchor::MiddleLeft | Anchor::BottomLeft => total_off_h,
                Anchor::TopRight | Anchor::MiddleRight | Anchor::BottomRight => -total_off_h,
                _ => total_off_h, // Center/TopCenter/BottomCenter treat positive as right
            };

            rect.y += match anchor {
                Anchor::TopLeft | Anchor::TopCenter | Anchor::TopRight => total_off_v,
                Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight => -total_off_v,
                _ => total_off_v, // Middle/Center treat positive as down
            };

            rect
        }
    }
}

// <FILE>src/manager/fnc_resolve_anchor.rs</FILE> - <DESC>Resolve anchor and offsets to concrete coordinates</DESC>
// <VERS>END OF VERSION: 0.2.2 - 2026-01-22</VERS>
