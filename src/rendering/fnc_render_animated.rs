// <FILE>src/rendering/fnc_render_animated.rs</FILE> - <DESC>Compatibility render entrypoint (default theme)</DESC>
// <VERS>VERSION: 0.3.0 - 2025-12-18</VERS>
// <WCTX>Theme refactor: preserve old signature while routing through themed renderer</WCTX>
// <CLOG>Delegated render_animated to themed implementation using Theme::default()</CLOG>

use crate::rendering::fnc_render_animated_with_theme::render_animated_optional_appearance;
use crate::rendering::types::RenderPlanItem;
use crate::theme::Theme;
use crate::traits::Animated;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use tui_vfx_compositor::context::cls_compositor_ctx::CompositorCtx;

pub fn render_animated<W: Widget>(
    item: &impl Animated,
    plan: &RenderPlanItem<'_>,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    inner_widget: W,
) {
    render_animated_optional_appearance(
        item,
        None,
        plan,
        &Theme::default(),
        frame_area,
        buf,
        ctx,
        |_resolved_text_style, _resolved_padding| inner_widget,
    );
}

// <FILE>src/rendering/fnc_render_animated.rs</FILE> - <DESC>Compatibility render entrypoint (default theme)</DESC>
// <VERS>END OF VERSION: 0.3.0 - 2025-12-18</VERS>
