// <FILE>src/preview/fnc_render_preview_item.rs</FILE> - <DESC>Render preview item</DESC>
// <VERS>VERSION: 1.6.2</VERS>
// <WCTX>Clippy cleanup for preview rendering</WCTX>
// <CLOG>Remove redundant closures and allow multi-arg inspected render helper</CLOG>

use super::cls_preview_item::PreviewItem;
use super::fnc_append_cursor_if_visible::append_cursor_if_visible;
use crate::compat::{animation_phase_to_mixed, style_config_to_ratatui};
use crate::inspector::InspectorContext;
use crate::rendering::{
    RenderPlanItem, render_animated_with_appearance, render_animated_with_appearance_inspected,
};
use crate::theme::{AppearanceConfig, ChromeConfig, PaddingConfig, Theme};
use mixed_signals::prelude::SignalContext;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Paragraph, Wrap};
use std::time::Duration;
use tui_vfx_compositor::context::cls_compositor_ctx::CompositorCtx;
use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_content::transformers::fnc_get_transformer::get_transformer;
use tui_vfx_content::types::ContentEffect;

fn build_signal_context(item: &PreviewItem, plan: &RenderPlanItem<'_>) -> SignalContext {
    let loop_period_ms = item
        .profile
        .loop_period
        .filter(|period| *period > Duration::ZERO)
        .map(|period| period.as_secs_f64() * 1000.0);

    let absolute_t = match (plan.loop_t, loop_period_ms) {
        (Some(loop_t), Some(period_ms)) => Some(loop_t * period_ms),
        _ => Some(plan.t * 1000.0),
    };

    SignalContext {
        frame: 0,
        seed: 0,
        width: item.width,
        height: item.height,
        phase: Some(animation_phase_to_mixed(plan.phase)),
        phase_t: Some(plan.t),
        loop_t: plan.loop_t,
        absolute_t,
        char_index: None,
    }
}

fn resolve_message(item: &PreviewItem, t: f64, signal_ctx: &SignalContext) -> String {
    if let Some(effect) = &item.content_effect {
        let transformer = get_transformer(effect);

        let base_text = transformer
            .transform(&item.message, t, signal_ctx)
            .to_string();

        // Add cursor if applicable (render layer concern)
        if let ContentEffect::Typewriter {
            cursor: Some(cursor),
            ..
        } = effect
        {
            append_cursor_if_visible(&base_text, cursor, t, signal_ctx)
        } else {
            base_text
        }
    } else {
        item.message.clone()
    }
}

/// Calculate padding to center content within the available area.
fn calculate_centering_padding(
    msg: &str,
    area: Rect,
    base_padding: &PaddingConfig,
) -> PaddingConfig {
    // Calculate content dimensions
    let lines: Vec<&str> = msg.lines().collect();
    let content_height = lines.len() as u16;
    let content_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as u16;

    // Calculate available inner area after base padding and borders (2 cells for borders)
    let inner_width = area
        .width
        .saturating_sub(base_padding.left + base_padding.right + 2);
    let inner_height = area
        .height
        .saturating_sub(base_padding.top + base_padding.bottom + 2);

    // Calculate centering offsets
    let extra_horizontal = inner_width.saturating_sub(content_width) / 2;
    let extra_vertical = inner_height.saturating_sub(content_height) / 2;

    PaddingConfig {
        left: base_padding.left + extra_horizontal,
        right: base_padding.right + extra_horizontal,
        top: base_padding.top + extra_vertical,
        bottom: base_padding.bottom + extra_vertical,
    }
}

/// Render a preview item to a buffer.
///
/// This is the core rendering function that ensures accurate playback
/// matching production behavior.
pub fn render_preview_item(
    item: &PreviewItem,
    plan: &RenderPlanItem<'_>,
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
) {
    let signal_ctx = build_signal_context(item, plan);
    let msg = resolve_message(item, plan.t, &signal_ctx);

    // Build appearance, potentially with center_content padding adjustments
    let appearance = if item.center_content {
        // Calculate centering padding and create modified appearance
        let base_appearance = item.appearance.clone().unwrap_or_default();
        let base_chrome = base_appearance.chrome.clone().unwrap_or_default();
        let base_padding = base_chrome.padding.unwrap_or_default();

        let centered_padding = calculate_centering_padding(&msg, plan.area, &base_padding);

        let modified_chrome = ChromeConfig {
            padding: Some(centered_padding),
            ..base_chrome
        };

        Some(AppearanceConfig {
            chrome: Some(modified_chrome),
            ..base_appearance
        })
    } else {
        item.appearance.clone()
    };

    render_animated_with_appearance(
        item,
        appearance.as_ref(),
        plan,
        theme,
        frame_area,
        buf,
        ctx,
        |resolved_text_style: Style, _resolved_padding| {
            let p = Paragraph::new(msg.clone()).style(resolved_text_style);
            if item.wrap {
                p.wrap(Wrap { trim: false })
            } else {
                p
            }
        },
    );

    // Draw custom frame content if present
    // Frame is drawn AFTER the Block widget, so it can override border characters
    // This enables multi-char patterns, partial blocks for visual effects, etc.
    if let Some(frame) = item.get_frame() {
        // Get the resolved border style for the frame
        let border_style = item
            .appearance
            .as_ref()
            .and_then(|a| a.chrome.as_ref())
            .and_then(|c| c.border_style.as_ref())
            .map(style_config_to_ratatui)
            .unwrap_or_default();

        frame.draw_to_buffer(buf, plan.area, border_style);
    }
}

/// Render a preview item with compositor inspection.
///
/// This variant passes an inspector through the rendering pipeline for
/// cell-by-cell tracing of mask, shader, and filter operations.
///
/// # Arguments
/// * `inspector_ctx` - High-level pipeline inspector for style interpolation hooks, etc.
/// * `compositor_inspector` - Low-level compositor inspector for per-cell operations
#[allow(clippy::too_many_arguments)]
pub fn render_preview_item_inspected(
    item: &PreviewItem,
    plan: &RenderPlanItem<'_>,
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    inspector_ctx: &mut InspectorContext,
    compositor_inspector: &mut dyn CompositorInspector,
) {
    let signal_ctx = build_signal_context(item, plan);
    let msg = resolve_message(item, plan.t, &signal_ctx);

    // Build appearance, potentially with center_content padding adjustments
    let appearance = if item.center_content {
        let base_appearance = item.appearance.clone().unwrap_or_default();
        let base_chrome = base_appearance.chrome.clone().unwrap_or_default();
        let base_padding = base_chrome.padding.unwrap_or_default();

        let centered_padding = calculate_centering_padding(&msg, plan.area, &base_padding);

        let modified_chrome = ChromeConfig {
            padding: Some(centered_padding),
            ..base_chrome
        };

        Some(AppearanceConfig {
            chrome: Some(modified_chrome),
            ..base_appearance
        })
    } else {
        item.appearance.clone()
    };

    render_animated_with_appearance_inspected(
        item,
        appearance.as_ref(),
        plan,
        theme,
        frame_area,
        buf,
        ctx,
        inspector_ctx,
        Some(compositor_inspector),
        |resolved_text_style: Style, _resolved_padding| {
            let p = Paragraph::new(msg.clone()).style(resolved_text_style);
            if item.wrap {
                p.wrap(Wrap { trim: false })
            } else {
                p
            }
        },
    );

    // Draw custom frame content if present (same as non-inspected version)
    if let Some(frame) = item.get_frame() {
        let border_style = item
            .appearance
            .as_ref()
            .and_then(|a| a.chrome.as_ref())
            .and_then(|c| c.border_style.as_ref())
            .map(style_config_to_ratatui)
            .unwrap_or_default();

        frame.draw_to_buffer(buf, plan.area, border_style);
    }
}

// <FILE>src/preview/fnc_render_preview_item.rs</FILE> - <DESC>Render preview item</DESC>
// <VERS>END OF VERSION: 1.6.2</VERS>
