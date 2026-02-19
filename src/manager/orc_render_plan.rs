// <FILE>src/manager/orc_render_plan.rs</FILE> - <DESC>Orchestrate render plan construction</DESC>
// <VERS>VERSION: 1.1.2</VERS>
// <WCTX>Clippy cleanup for render plan orchestration</WCTX>
// <CLOG>Use is_none_or to simplify empty grid layout check</CLOG>

use crate::compat::{ratatui_rect_to_vfx, vfx_rect_to_ratatui};
use crate::manager::fnc_calculate_motion_rect::calculate_motion_rect;
use crate::manager::fnc_compute_dwell_rect::compute_dwell_rect;
use crate::manager::fnc_populate_effects::populate_effects;
use crate::rendering::RenderPlanItem;
use crate::state::{AnimationPhase, LifecycleState};
use crate::traits::Animated;
use crate::types::{Animation, StackingPolicy};
use ratatui::layout::Rect;
use std::collections::HashMap;
use std::time::Instant;
use tui_vfx_geometry::transitions::{
    ExpandPhase, SlidePhase, expand_collapse_calculate_rect,
    slide_calculate_rect_path_with_path_type, slide_calculate_signed_rect,
};
use tui_vfx_geometry::types::{Anchor, SignedRect};
struct VisibleItem<'a, T: Animated> {
    id: u64,
    created_at: Instant,
    state: &'a LifecycleState<T>,
    height: u16,
    width: u16,
    margin: u16,
}

#[derive(Debug, Clone, Copy)]
struct GridPlacement {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct GridLayout {
    placements: Vec<Option<GridPlacement>>,
    column_offsets: Vec<u16>,
    row_offsets: Vec<u16>,
}

#[derive(Clone)]
enum OrderedIndices {
    Forward(std::ops::Range<usize>),
    Reverse(std::iter::Rev<std::ops::Range<usize>>),
}

impl Iterator for OrderedIndices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Forward(iter) => iter.next(),
            Self::Reverse(iter) => iter.next(),
        }
    }
}

const ANCHOR_ORDER: [Anchor; 9] = [
    Anchor::TopLeft,
    Anchor::TopCenter,
    Anchor::TopRight,
    Anchor::MiddleLeft,
    Anchor::Center,
    Anchor::MiddleRight,
    Anchor::BottomLeft,
    Anchor::BottomCenter,
    Anchor::BottomRight,
];
pub fn render_plan<'a, T: Animated>(
    states: &'a HashMap<u64, LifecycleState<T>>,
    by_anchor: &HashMap<Anchor, Vec<u64>>,
    frame_area: Rect,
    now: Instant,
    override_t: Option<f64>,
    policy: StackingPolicy,
) -> Vec<RenderPlanItem<'a>> {
    let mut result = Vec::new();
    for anchor in ANCHOR_ORDER {
        if let Some(ids) = by_anchor.get(&anchor) {
            result.extend(plan_for_anchor(
                states, anchor, ids, frame_area, now, override_t, policy,
            ));
        }
    }
    // Anchor is #[non_exhaustive] (tui-geometry): ensure new variants still render.
    let mut remaining_anchors: Vec<Anchor> = by_anchor
        .keys()
        .copied()
        .filter(|anchor| !ANCHOR_ORDER.contains(anchor))
        .collect();
    if !remaining_anchors.is_empty() {
        remaining_anchors.sort_by_cached_key(|anchor| format!("{anchor:?}"));
        for anchor in remaining_anchors {
            if let Some(ids) = by_anchor.get(&anchor) {
                result.extend(plan_for_anchor(
                    states, anchor, ids, frame_area, now, override_t, policy,
                ));
            }
        }
    }
    result
}
fn plan_for_anchor<'a, T: Animated>(
    states: &'a HashMap<u64, LifecycleState<T>>,
    anchor: Anchor,
    ids: &[u64],
    frame_area: Rect,
    now: Instant,
    override_t: Option<f64>,
    policy: StackingPolicy,
) -> Vec<RenderPlanItem<'a>> {
    let mut visible: Vec<VisibleItem<T>> = ids
        .iter()
        .filter_map(|id| {
            let state = states.get(id)?;
            if state.phase == AnimationPhase::Finished {
                return None;
            }
            // Fullscreen items use frame dimensions instead of configured width/height
            let (width, height) = if state.item.is_fullscreen() {
                (frame_area.width, frame_area.height)
            } else {
                (
                    state.item.width().min(frame_area.width),
                    state.item.height().min(frame_area.height),
                )
            };
            Some(VisibleItem {
                id: *id,
                created_at: state.created_at(),
                state,
                height,
                width,
                margin: state.item.exterior_margin(),
            })
        })
        .collect();
    if visible.is_empty() {
        return Vec::new();
    }
    visible.sort_unstable_by_key(|v| v.created_at);

    // Handle StackingPolicy::None - only render first item
    if matches!(policy, StackingPolicy::None) {
        visible.truncate(1);
    }

    // Determine stacking direction based on anchor and policy
    let is_stacking_up = matches!(
        anchor,
        Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight
    );
    let is_stacking_left = matches!(
        anchor,
        Anchor::TopRight | Anchor::MiddleRight | Anchor::BottomRight
    );

    let anchor_pos = tui_vfx_geometry::anchors::calculate_anchor_position(
        anchor,
        ratatui_rect_to_vfx(frame_area),
    );
    let available_height = if is_stacking_up {
        anchor_pos.y.saturating_sub(frame_area.y).saturating_add(1)
    } else {
        frame_area.bottom().saturating_sub(anchor_pos.y)
    };
    let available_width = if is_stacking_left {
        anchor_pos.x.saturating_sub(frame_area.x).saturating_add(1)
    } else {
        frame_area.right().saturating_sub(anchor_pos.x)
    };
    let ordered_indices = if is_stacking_up {
        OrderedIndices::Reverse((0..visible.len()).rev())
    } else {
        OrderedIndices::Forward(0..visible.len())
    };
    let grid_layout = match policy {
        StackingPolicy::Grid {
            max_columns,
            row_spacing,
            column_spacing,
        } => Some(build_grid_layout(
            &visible,
            ordered_indices.clone(),
            available_height,
            max_columns,
            row_spacing,
            column_spacing,
        )),
        _ => None,
    };
    if matches!(policy, StackingPolicy::Grid { .. })
        && grid_layout.as_ref().is_none_or(|layout| {
            layout
                .placements
                .iter()
                .all(|placement| placement.is_none())
        })
        && !visible.iter().any(|item| item.state.item.is_fullscreen())
    {
        return Vec::new();
    }

    let mut accumulated_height: u16 = 0;
    let mut accumulated_width: u16 = 0;
    let mut out = Vec::new();
    for idx in ordered_indices {
        let v = &visible[idx];
        // Calculate stacked position based on policy
        let (stacked_rect, accumulated_size) = match policy {
            StackingPolicy::Vertical {
                spacing: vertical_spacing,
            } => {
                let spacing = if accumulated_height > 0 {
                    vertical_spacing
                } else {
                    0
                };
                let needed = v.height.saturating_add(spacing);
                // Fullscreen items bypass stacking overflow checks - they use the entire frame
                let is_fullscreen = v.state.item.is_fullscreen();
                if !is_fullscreen && accumulated_height.saturating_add(needed) > available_height {
                    break;
                }
                let dwell = compute_dwell_rect(
                    anchor,
                    v.state.item.offset_h_percent(),
                    v.state.item.offset_v_percent(),
                    v.state.item.offset_h_cells(),
                    v.state.item.offset_v_cells(),
                    v.state.item.offset_h_pixels(),
                    v.state.item.offset_v_pixels(),
                    frame_area,
                    v.width,
                    v.height,
                    v.margin,
                );
                let vfx_frame = ratatui_rect_to_vfx(frame_area);
                let base_rect = vfx_rect_to_ratatui(
                    dwell
                        .to_ratatui_clamped()
                        .intersect(&vfx_frame)
                        .unwrap_or_default(),
                );
                // Fullscreen items always start at frame origin
                let stacked_y = if is_fullscreen {
                    frame_area.y
                } else if is_stacking_up {
                    base_rect.y.saturating_sub(accumulated_height)
                } else {
                    base_rect.y.saturating_add(accumulated_height)
                };
                let rect = Rect {
                    x: if is_fullscreen {
                        frame_area.x
                    } else {
                        base_rect.x
                    },
                    y: stacked_y
                        .max(frame_area.y)
                        .min(frame_area.bottom().saturating_sub(v.height)),
                    width: if is_fullscreen {
                        frame_area.width
                    } else {
                        base_rect.width
                    },
                    height: v.height,
                }
                .intersection(frame_area);
                (rect, needed)
            }
            StackingPolicy::Horizontal {
                spacing: horizontal_spacing,
            } => {
                let spacing = if accumulated_width > 0 {
                    horizontal_spacing
                } else {
                    0
                };
                let needed = v.width.saturating_add(spacing);
                // Fullscreen items bypass stacking overflow checks
                let is_fullscreen = v.state.item.is_fullscreen();
                if !is_fullscreen && accumulated_width.saturating_add(needed) > available_width {
                    break;
                }
                let dwell = compute_dwell_rect(
                    anchor,
                    v.state.item.offset_h_percent(),
                    v.state.item.offset_v_percent(),
                    v.state.item.offset_h_cells(),
                    v.state.item.offset_v_cells(),
                    v.state.item.offset_h_pixels(),
                    v.state.item.offset_v_pixels(),
                    frame_area,
                    v.width,
                    v.height,
                    v.margin,
                );
                let vfx_frame = ratatui_rect_to_vfx(frame_area);
                let base_rect = vfx_rect_to_ratatui(
                    dwell
                        .to_ratatui_clamped()
                        .intersect(&vfx_frame)
                        .unwrap_or_default(),
                );
                // Fullscreen items always start at frame origin
                let stacked_x = if is_fullscreen {
                    frame_area.x
                } else if is_stacking_left {
                    base_rect.x.saturating_sub(accumulated_width)
                } else {
                    base_rect.x.saturating_add(accumulated_width)
                };
                let rect = Rect {
                    x: stacked_x
                        .max(frame_area.x)
                        .min(frame_area.right().saturating_sub(v.width)),
                    y: if is_fullscreen {
                        frame_area.y
                    } else {
                        base_rect.y
                    },
                    width: v.width,
                    height: if is_fullscreen {
                        frame_area.height
                    } else {
                        base_rect.height
                    },
                }
                .intersection(frame_area);
                (rect, needed)
            }
            StackingPolicy::Grid { .. } => {
                // Fullscreen items bypass grid layout
                let is_fullscreen = v.state.item.is_fullscreen();
                if is_fullscreen {
                    (frame_area, 0)
                } else {
                    let layout = grid_layout
                        .as_ref()
                        .and_then(|layout| layout.placements.get(idx).copied())
                        .flatten();
                    let Some(placement) = layout else {
                        break;
                    };
                    let dwell = compute_dwell_rect(
                        anchor,
                        v.state.item.offset_h_percent(),
                        v.state.item.offset_v_percent(),
                        v.state.item.offset_h_cells(),
                        v.state.item.offset_v_cells(),
                        v.state.item.offset_h_pixels(),
                        v.state.item.offset_v_pixels(),
                        frame_area,
                        v.width,
                        v.height,
                        v.margin,
                    );
                    let vfx_frame = ratatui_rect_to_vfx(frame_area);
                    let base_rect = vfx_rect_to_ratatui(
                        dwell
                            .to_ratatui_clamped()
                            .intersect(&vfx_frame)
                            .unwrap_or_default(),
                    );
                    let offset_x = grid_layout
                        .as_ref()
                        .and_then(|layout| layout.column_offsets.get(placement.col).copied())
                        .unwrap_or(0);
                    let offset_y = grid_layout
                        .as_ref()
                        .and_then(|layout| layout.row_offsets.get(placement.row).copied())
                        .unwrap_or(0);
                    let stacked_x = if is_stacking_left {
                        base_rect.x.saturating_sub(offset_x)
                    } else {
                        base_rect.x.saturating_add(offset_x)
                    };
                    let stacked_y = if is_stacking_up {
                        base_rect.y.saturating_sub(offset_y)
                    } else {
                        base_rect.y.saturating_add(offset_y)
                    };
                    let rect = Rect {
                        x: stacked_x.max(frame_area.x),
                        y: stacked_y.max(frame_area.y),
                        width: v.width,
                        height: v.height,
                    }
                    .intersection(frame_area);
                    (rect, 0) // Grid doesn't use linear accumulation
                }
            }
            StackingPolicy::None => {
                // Fullscreen items use entire frame
                let is_fullscreen = v.state.item.is_fullscreen();
                if is_fullscreen {
                    (frame_area, 0)
                } else {
                    let dwell = compute_dwell_rect(
                        anchor,
                        v.state.item.offset_h_percent(),
                        v.state.item.offset_v_percent(),
                        v.state.item.offset_h_cells(),
                        v.state.item.offset_v_cells(),
                        v.state.item.offset_h_pixels(),
                        v.state.item.offset_v_pixels(),
                        frame_area,
                        v.width,
                        v.height,
                        v.margin,
                    );
                    let vfx_frame = ratatui_rect_to_vfx(frame_area);
                    let rect = vfx_rect_to_ratatui(
                        dwell
                            .to_ratatui_clamped()
                            .intersect(&vfx_frame)
                            .unwrap_or_default(),
                    );
                    (rect, 0)
                }
            }
        };

        // Skip items with zero dimensions
        if stacked_rect.width == 0 || stacked_rect.height == 0 {
            if matches!(policy, StackingPolicy::Vertical { .. }) {
                accumulated_height = accumulated_height.saturating_add(accumulated_size);
            } else if matches!(policy, StackingPolicy::Horizontal { .. }) {
                accumulated_width = accumulated_width.saturating_add(accumulated_size);
            }
            continue;
        }
        let active_anim = v.state.active_animation();
        let phase = v.state.phase;
        // Use override_t if provided (for scrub/preview mode), otherwise calculate from time
        let t_raw = if let Some(t) = override_t {
            t.clamp(0.0, 1.0)
        } else {
            match phase {
                AnimationPhase::Entering | AnimationPhase::Exiting | AnimationPhase::Dwelling => {
                    v.state.phase_progress(now)
                }
                AnimationPhase::Finished => 1.0,
            }
        };
        let profile = v.state.item.profile();
        let eased = match phase {
            AnimationPhase::Entering => {
                let e = profile.enter.ease.ease(t_raw) as f64;
                profile.enter.quantize(e) // Apply stepped animation if configured
            }
            AnimationPhase::Exiting => {
                let e = profile.exit.ease.ease(t_raw) as f64;
                profile.exit.quantize(e) // Apply stepped animation if configured
            }
            _ => 1.0,
        };
        // Calculate both clamped and unclamped positions
        let (animated_rect, signed_area) = match active_anim {
            Animation::Slide => {
                let path = v.state.item.slide_path(frame_area, stacked_rect);
                let slide_phase = match phase {
                    AnimationPhase::Entering => SlidePhase::SlidingIn,
                    AnimationPhase::Exiting => SlidePhase::SlidingOut,
                    _ => SlidePhase::SlidingIn,
                };
                let path_type = match phase {
                    AnimationPhase::Entering => &profile.enter.path,
                    AnimationPhase::Exiting => &profile.exit.path,
                    _ => &profile.enter.path,
                };
                let clamped = vfx_rect_to_ratatui(slide_calculate_rect_path_with_path_type(
                    path,
                    ratatui_rect_to_vfx(frame_area),
                    eased,
                    slide_phase,
                    path_type,
                ));
                let unclamped = slide_calculate_signed_rect(path, eased, slide_phase, path_type);
                (clamped, unclamped)
            }
            Animation::ExpandCollapse => {
                let vfx_stacked = ratatui_rect_to_vfx(stacked_rect);
                let full = SignedRect::from(vfx_stacked);
                let expand_phase = match phase {
                    AnimationPhase::Entering => ExpandPhase::Expanding,
                    AnimationPhase::Exiting => ExpandPhase::Collapsing,
                    _ => ExpandPhase::Expanding,
                };
                let rect = expand_collapse_calculate_rect(full, expand_phase, eased);
                let vfx_frame = ratatui_rect_to_vfx(frame_area);
                (
                    vfx_rect_to_ratatui(rect.intersect(&vfx_frame).unwrap_or_default()),
                    SignedRect::from(rect),
                )
            }
            Animation::Motion => {
                // Get the appropriate MotionSpec based on phase
                let motion_spec = match phase {
                    AnimationPhase::Entering => profile.enter_motion.as_ref(),
                    AnimationPhase::Exiting => profile
                        .exit_motion
                        .as_ref()
                        .or(profile.enter_motion.as_ref()),
                    _ => None,
                };

                if let Some(spec) = motion_spec {
                    // Apply easing from the MotionSpec
                    let motion_eased = spec.ease.ease(t_raw) as f64;
                    let rect =
                        calculate_motion_rect(spec, phase, frame_area, stacked_rect, motion_eased);
                    (rect, SignedRect::from(ratatui_rect_to_vfx(rect)))
                } else {
                    // No MotionSpec provided, fall back to stacked_rect
                    (
                        stacked_rect,
                        SignedRect::from(ratatui_rect_to_vfx(stacked_rect)),
                    )
                }
            }
            _ => (
                stacked_rect,
                SignedRect::from(ratatui_rect_to_vfx(stacked_rect)),
            ),
        };
        /*
           Invisible items (width/height 0 due to clamping) are preserved in the plan
           to support overshoot testing and Sixel rendering which uses signed_area.
        */
        let mut plan_item = RenderPlanItem::new(
            v.id,
            anchor,
            v.state.item.offset_h_percent(),
            v.state.item.offset_v_percent(),
            v.state.item.offset_h_cells(),
            v.state.item.offset_v_cells(),
            v.state.item.offset_h_pixels(),
            v.state.item.offset_v_pixels(),
            phase,
            active_anim,
            stacked_rect,
            animated_rect,
            signed_area,
            t_raw,
        );

        // WG8: Populate loop_t for cyclical effects
        plan_item.loop_t = v.state.loop_progress(now);

        // Populate V2 pipeline specs based on animation phase
        populate_effects(
            &mut plan_item,
            &v.state.item,
            phase,
            v.state.item.mask_combine_mode(),
        );

        out.push(plan_item);

        // Update accumulation based on stacking policy
        match policy {
            StackingPolicy::Vertical { .. } => {
                accumulated_height = accumulated_height.saturating_add(accumulated_size);
            }
            StackingPolicy::Horizontal { .. } => {
                accumulated_width = accumulated_width.saturating_add(accumulated_size);
            }
            StackingPolicy::Grid { .. } | StackingPolicy::None => {
                // Grid handles its own accumulation, None doesn't accumulate
            }
        }
    }
    out
}

fn build_grid_layout<T: Animated, I: IntoIterator<Item = usize>>(
    visible: &[VisibleItem<'_, T>],
    ordered_indices: I,
    available_height: u16,
    max_columns: u16,
    row_spacing: u16,
    column_spacing: u16,
) -> GridLayout {
    let max_columns = max_columns.max(1) as usize;
    let max_columns = max_columns.min(visible.len().max(1));
    if visible.is_empty() {
        return GridLayout {
            placements: Vec::new(),
            column_offsets: vec![0u16; max_columns],
            row_offsets: Vec::new(),
        };
    }
    let mut placements = vec![None; visible.len()];
    let mut column_widths = vec![0u16; max_columns];
    let mut row_heights: Vec<u16> = vec![0u16];
    let mut row: usize = 0;
    let mut col: usize = 0;

    for idx in ordered_indices {
        let v = &visible[idx];
        if v.state.item.is_fullscreen() {
            placements[idx] = None;
            continue;
        }
        if col >= max_columns {
            row += 1;
            col = 0;
            row_heights.push(0);
        }
        let current_row_height = row_heights.get(row).copied().unwrap_or(0);
        let new_row_height = current_row_height.max(v.height);
        let mut used_height: u16 = 0;
        for (r, height) in row_heights.iter().enumerate() {
            let applied_height = if r == row { new_row_height } else { *height };
            used_height = used_height.saturating_add(applied_height);
            if r + 1 < row_heights.len() {
                used_height = used_height.saturating_add(row_spacing);
            }
        }
        if used_height > available_height {
            break;
        }
        row_heights[row] = new_row_height;
        if v.width > column_widths[col] {
            column_widths[col] = v.width;
        }
        placements[idx] = Some(GridPlacement { row, col });
        col += 1;
    }

    let mut column_offsets = vec![0u16; max_columns];
    let mut running_x: u16 = 0;
    for (col_idx, width) in column_widths.iter().enumerate() {
        column_offsets[col_idx] = running_x;
        running_x = running_x.saturating_add(width.saturating_add(column_spacing));
    }

    let mut row_offsets = Vec::with_capacity(row_heights.len());
    let mut running_y: u16 = 0;
    for height in row_heights {
        row_offsets.push(running_y);
        running_y = running_y.saturating_add(height.saturating_add(row_spacing));
    }

    GridLayout {
        placements,
        column_offsets,
        row_offsets,
    }
}

// <FILE>src/manager/orc_render_plan.rs</FILE> - <DESC>Orchestrate render plan construction</DESC>
// <VERS>END OF VERSION: 1.1.2</VERS>
