// <FILE>src/manager/fnc_calculate_motion_rect.rs</FILE> - <DESC>Motion animation rect calculation</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>TUI VFX recipes extraction - fix types for tui-vfx compatibility</WCTX>
// <CLOG>Update to use tui_vfx_types::Rect with compat conversions

use crate::compat::ratatui_rect_to_vfx;
use crate::state::AnimationPhase;
use ratatui::layout::Rect as RatatuiRect;
use tui_vfx_geometry::transitions::interpolate_position;
use tui_vfx_geometry::types::{MotionSpec, PathType, Position, SnappingStrategy};

/// Calculate the animated rect for a Motion animation using MotionSpec.
///
/// For entering: interpolates from `from` position to `to` position (or dwell).
/// For exiting: interpolates from current position to `to` position (or offscreen).
///
/// # Arguments
/// * `motion_spec` - The motion specification defining waypoints and path
/// * `phase` - Current animation phase (Entering/Exiting/Dwelling/Finished)
/// * `frame_area` - The visible frame area for boundary calculations
/// * `dwell_rect` - The target resting position
/// * `eased_t` - Eased time parameter (0.0-1.0) with easing already applied
///
/// # Returns
/// The animated rectangle at the current eased time, clamped to frame bounds.
pub fn calculate_motion_rect(
    motion_spec: &MotionSpec,
    phase: AnimationPhase,
    frame_area: RatatuiRect,
    dwell_rect: RatatuiRect,
    eased_t: f64,
) -> RatatuiRect {
    // Convert to vfx types for geometry calculations
    let vfx_frame = ratatui_rect_to_vfx(frame_area);
    let vfx_dwell = ratatui_rect_to_vfx(dwell_rect);

    // Determine start and end positions based on phase
    let (start_pos, end_pos) = match phase {
        AnimationPhase::Entering => {
            // Start from `from` position (or offscreen if not specified)
            let start = motion_spec
                .resolve_from(vfx_frame, vfx_dwell)
                .unwrap_or_else(|| Position {
                    x: vfx_dwell.x as i32,
                    y: vfx_frame.bottom() as i32, // Default: from below
                });
            // End at `to` position (or dwell position if not specified)
            let end = motion_spec
                .resolve_to(vfx_frame, vfx_dwell)
                .unwrap_or(Position {
                    x: vfx_dwell.x as i32,
                    y: vfx_dwell.y as i32,
                });
            (start, end)
        }
        AnimationPhase::Exiting => {
            // Start from dwell position
            let start = Position {
                x: vfx_dwell.x as i32,
                y: vfx_dwell.y as i32,
            };
            // End at `to` position (or offscreen if not specified)
            let end = motion_spec
                .resolve_to(vfx_frame, vfx_dwell)
                .unwrap_or_else(|| Position {
                    x: vfx_dwell.x as i32,
                    y: vfx_frame.bottom() as i32, // Default: to below
                });
            (start, end)
        }
        _ => {
            // Dwelling or Finished - just return dwell rect
            return dwell_rect;
        }
    };

    // Calculate interpolated position based on path type
    // Handle Bezier specially if a via point is specified (overrides control point)
    let effective_path = match &motion_spec.path {
        PathType::Bezier {
            control_x,
            control_y,
        } => {
            // Use via position if specified, otherwise use the explicit control point
            if let Some(via_pos) = motion_spec.resolve_via(vfx_frame, vfx_dwell) {
                PathType::Bezier {
                    control_x: via_pos.x as f32,
                    control_y: via_pos.y as f32,
                }
            } else {
                PathType::Bezier {
                    control_x: *control_x,
                    control_y: *control_y,
                }
            }
        }
        other => other.clone(),
    };

    // Use the centralized interpolate_position for all path types (physics included)
    let (x, y) = interpolate_position(start_pos, end_pos, eased_t, &effective_path);

    // Apply snapping (Stochastic falls back to Round)
    let (snapped_x, snapped_y) = match motion_spec.snap {
        SnappingStrategy::Round | SnappingStrategy::Stochastic { .. } => {
            (x.round().max(0.0) as u16, y.round().max(0.0) as u16)
        }
        SnappingStrategy::Floor => (x.floor().max(0.0) as u16, y.floor().max(0.0) as u16),
    };

    // Construct the animated rect and clamp to frame
    let result = RatatuiRect {
        x: snapped_x.max(frame_area.x),
        y: snapped_y.max(frame_area.y),
        width: dwell_rect.width,
        height: dwell_rect.height,
    };
    result.intersection(frame_area)
}

// <FILE>src/manager/fnc_calculate_motion_rect.rs</FILE> - <DESC>Motion animation rect calculation</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
