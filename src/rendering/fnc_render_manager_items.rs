// <FILE>src/rendering/fnc_render_manager_items.rs</FILE> - <DESC>Shared render loop for AnimationManager items</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Unified pipeline architecture - render abstraction</WCTX>
// <CLOG>Initial creation - shared render loop to eliminate duplication</CLOG>

use crate::manager::AnimationManager;
use crate::rendering::RenderPlanItem;
use crate::theme::Theme;
use crate::traits::Animated;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use tui_vfx_compositor::context::cls_compositor_ctx::CompositorCtx;

/// Render all items from an AnimationManager using a custom per-item renderer.
///
/// This function provides the shared render loop pattern used by both
/// `Notifications` and `PreviewManager`. It iterates over the render plan,
/// retrieves each item's state, and delegates to the provided render function.
///
/// # Arguments
///
/// * `manager` - The animation manager containing item states
/// * `plan` - Pre-computed render plan from `manager.render_plan()`
/// * `theme` - Theme for styling
/// * `frame_area` - Full frame area for clipping
/// * `buf` - Buffer to render into
/// * `ctx` - Compositor context for effects
/// * `render_item` - Per-item render function
///
/// # Type Parameters
///
/// * `T` - The animated item type (e.g., `Notification`, `PreviewItem`)
/// * `F` - The render function type
///
/// # Example
///
/// ```ignore
/// use tui_vfx_recipes::rendering::render_manager_items;
///
/// let plan = manager.render_plan(frame_area, now);
/// render_manager_items(
///     &manager,
///     &plan,
///     &theme,
///     frame_area,
///     buf,
///     ctx,
///     |item, plan_item, theme, frame_area, buf, ctx| {
///         render_my_item(item, plan_item, theme, frame_area, buf, ctx);
///     },
/// );
/// ```
pub fn render_manager_items<T, F>(
    manager: &AnimationManager<T>,
    plan: &[RenderPlanItem<'_>],
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    mut render_item: F,
) where
    T: Animated,
    F: FnMut(&T, &RenderPlanItem, &Theme, Rect, &mut Buffer, &mut CompositorCtx),
{
    for item in plan {
        let Some(state) = manager.get_state(item.id) else {
            continue;
        };
        render_item(&state.item, item, theme, frame_area, buf, ctx);
    }
}

// <FILE>src/rendering/fnc_render_manager_items.rs</FILE> - <DESC>Shared render loop for AnimationManager items</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
