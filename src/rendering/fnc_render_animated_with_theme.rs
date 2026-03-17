// <FILE>src/rendering/fnc_render_animated_with_theme.rs</FILE> - <DESC>Themed rendering entrypoint</DESC>
// <VERS>VERSION: 3.3.0</VERS>
// <WCTX>Fix sampler time routing during dwell phase</WCTX>
// <CLOG>Use loop_t for CompositionOptions.t so dwell samplers (pendulum, sine_wave, ripple) receive cycling time instead of frozen phase progress</CLOG>

use crate::inspect;
use crate::inspector::InspectorContext;
use crate::rendering::cls_ratatui_buffer_adapter::{RatatuiBufferAdapter, RatatuiBufferSnapshot};
use crate::rendering::types::RenderPlanItem;
use crate::state::AnimationPhase;
use crate::theme::{AppearanceConfig, HasAppearance, Theme, TitlePosition};
use crate::traits::Animated;
use crate::types::{Animation, SlideBorderTrimPolicy};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, Widget};
use tui_vfx_compositor::context::cls_compositor_ctx::CompositorCtx;
use tui_vfx_compositor::pipeline::{CompositionOptions, render_pipeline};
use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_geometry::borders::{BorderSegment, BorderTrimSpec, vanishing_edge_trim_spec};
use tui_vfx_geometry::transitions::resolve_slide_direction;
use tui_vfx_geometry::types::SlideDirection;
use tui_vfx_style::models::StyleEffect;
use tui_vfx_style::models::fade_effect;
use tui_vfx_style::traits::StyleInterpolator;

pub fn render_animated_with_theme<T, W, F>(
    item: &T,
    plan: &RenderPlanItem<'_>,
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    build_inner: F,
) where
    T: Animated + HasAppearance,
    W: Widget,
    F: FnOnce(Style, ratatui::widgets::Padding) -> W,
{
    render_animated_optional_appearance(
        item,
        item.appearance(),
        plan,
        theme,
        frame_area,
        buf,
        ctx,
        build_inner,
    );
}

/// Render an animated item with an explicit appearance override.
///
/// Use this when you need to modify the appearance (e.g., for center_content padding)
/// without using the item's built-in appearance.
#[allow(clippy::too_many_arguments)]
pub fn render_animated_with_appearance<T, W, F>(
    item: &T,
    appearance: Option<&AppearanceConfig>,
    plan: &RenderPlanItem<'_>,
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    build_inner: F,
) where
    T: Animated,
    W: Widget,
    F: FnOnce(Style, ratatui::widgets::Padding) -> W,
{
    render_animated_optional_appearance(
        item,
        appearance,
        plan,
        theme,
        frame_area,
        buf,
        ctx,
        build_inner,
    );
}

/// Render an animated item with explicit appearance and pipeline inspection hooks.
///
/// This is the most flexible variant, supporting:
/// - Explicit appearance override (for center_content padding adjustments)
/// - High-level pipeline inspection via InspectorContext
/// - Low-level compositor inspection via CompositorInspector
#[allow(clippy::too_many_arguments)]
pub fn render_animated_with_appearance_inspected<T, W, F>(
    item: &T,
    appearance: Option<&AppearanceConfig>,
    plan: &RenderPlanItem<'_>,
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    inspector_ctx: &mut InspectorContext,
    compositor_inspector: Option<&mut dyn CompositorInspector>,
    build_inner: F,
) where
    T: Animated,
    W: Widget,
    F: FnOnce(Style, ratatui::widgets::Padding) -> W,
{
    render_animated_with_inspector_impl(
        item,
        appearance,
        plan,
        theme,
        frame_area,
        buf,
        ctx,
        inspector_ctx,
        compositor_inspector,
        build_inner,
    );
}

/// Render an animated item with pipeline inspection hooks.
///
/// Use this variant when you need to trace the render pipeline for debugging.
/// Calls inspector methods at key points:
/// - `on_phase_entered` when processing the current phase
/// - `on_effect_extracted` for each phase's effect
/// - `on_render_plan_created` after extracting the render plan
///
/// Optionally accepts a compositor inspector for low-level cell-by-cell tracing.
#[allow(clippy::too_many_arguments)]
pub fn render_animated_with_inspector<T, W, F>(
    item: &T,
    plan: &RenderPlanItem<'_>,
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    inspector_ctx: &mut InspectorContext,
    compositor_inspector: Option<&mut dyn CompositorInspector>,
    build_inner: F,
) where
    T: Animated + HasAppearance,
    W: Widget,
    F: FnOnce(Style, ratatui::widgets::Padding) -> W,
{
    render_animated_with_inspector_impl(
        item,
        item.appearance(),
        plan,
        theme,
        frame_area,
        buf,
        ctx,
        inspector_ctx,
        compositor_inspector,
        build_inner,
    );
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_animated_optional_appearance<T, W, F>(
    item: &T,
    appearance: Option<&AppearanceConfig>,
    plan: &RenderPlanItem<'_>,
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    build_inner: F,
) where
    T: Animated,
    W: Widget,
    F: FnOnce(Style, ratatui::widgets::Padding) -> W,
{
    // Delegate to the inspector-enabled version with no inspectors
    let mut inspector_ctx = InspectorContext::none();
    render_animated_with_inspector_impl(
        item,
        appearance,
        plan,
        theme,
        frame_area,
        buf,
        ctx,
        &mut inspector_ctx,
        None, // No compositor inspector in production path
        build_inner,
    );
}

#[allow(clippy::too_many_arguments)]
fn render_animated_with_inspector_impl<T, W, F>(
    item: &T,
    appearance: Option<&AppearanceConfig>,
    plan: &RenderPlanItem<'_>,
    theme: &Theme,
    frame_area: Rect,
    buf: &mut Buffer,
    ctx: &mut CompositorCtx,
    inspector_ctx: &mut InspectorContext,
    compositor_inspector: Option<&mut dyn CompositorInspector>,
    build_inner: F,
) where
    T: Animated,
    W: Widget,
    F: FnOnce(Style, ratatui::widgets::Padding) -> W,
{
    // Notify inspector of phase entry
    inspect!(inspector_ctx, on_phase_entered, plan.phase);

    let profile = item.profile();
    let resolved = crate::theme::fnc_resolve_effective_appearance::resolve_effective_appearance_opt(
        theme, appearance, plan.phase,
    );

    let fade = resolved.fade_for_phase();

    // ==========================================================================
    // ARCHITECTURAL NOTE: Phase-Specific Data Extraction
    // ==========================================================================
    // When working with phase-specific data (enter/dwell/exit), ALL extraction
    // points must respect the current phase. This code has multiple extraction
    // points that consume phase_effect:
    //
    //   1. shader extraction (line below)
    //   2. resolve_style() calls for frame/border/text
    //
    // Previously, shader was extracted only from enter_style regardless of phase,
    // causing spatial shaders in dwell_style/exit_style to be ignored. This was
    // a subtle bug because resolve_style() correctly used phase_effect.
    //
    // LESSON: When adding new extraction points for phase-specific data, always
    // extract from phase_effect (or equivalent phase-aware source), never from
    // a single phase like enter_style directly.
    // ==========================================================================

    // Apply phase-specific style effects: enter_style during Entering, dwell_style during Dwelling, exit_style during Exiting
    let phase_effect = match plan.phase {
        AnimationPhase::Entering => profile.enter_style.as_ref(),
        AnimationPhase::Dwelling => profile.dwell_style.as_ref(),
        AnimationPhase::Exiting => profile.exit_style.as_ref(),
        AnimationPhase::Finished => None,
    };

    // Notify inspector of effect extraction
    inspect!(inspector_ctx, on_effect_extracted, plan.phase, phase_effect);

    // Build shader_layers from effective_style_layers for multi-region support
    let style_layers = profile.effective_style_layers();
    let shader_entries: Vec<_> = style_layers
        .iter()
        .filter_map(|layer| {
            // Get the phase-appropriate effect from this layer
            let (effect, region_override) = match plan.phase {
                AnimationPhase::Entering => {
                    (layer.enter_effect.as_ref(), layer.enter_region.as_ref())
                }
                AnimationPhase::Dwelling => {
                    (layer.dwell_effect.as_ref(), layer.dwell_region.as_ref())
                }
                AnimationPhase::Exiting => (layer.exit_effect.as_ref(), layer.exit_region.as_ref()),
                AnimationPhase::Finished => (None, None),
            };
            // Extract shader from the effect
            let shader = effect.and_then(|e| e.shader())?;
            // Use region override if present, otherwise use layer's base region
            let region = region_override
                .cloned()
                .unwrap_or_else(|| layer.region.clone());
            Some((shader, region))
        })
        .collect();

    let frame_style = resolve_style(resolved.frame_style, phase_effect, fade, plan.t);
    let border_style = resolve_style(resolved.border_style, phase_effect, fade, plan.t);
    let text_style = resolve_style(resolved.text_style, phase_effect, fade, plan.t);

    // Notify inspector of style interpolation for non-spatial effects (FadeIn, Pulse, etc.)
    // This captures style changes that don't go through the compositor shader pipeline
    if let Some(effect) = phase_effect {
        // Only fire if an effect actually was applied (not just Spatial which uses shader path)
        if effect.shader().is_none() {
            let effect_name = effect.effect_type_name();
            inspect!(
                inspector_ctx,
                on_style_interpolated,
                plan.phase,
                plan.t,
                resolved.frame_style,
                frame_style,
                effect_name,
                "frame"
            );
            inspect!(
                inspector_ctx,
                on_style_interpolated,
                plan.phase,
                plan.t,
                resolved.border_style,
                border_style,
                effect_name,
                "border"
            );
            inspect!(
                inspector_ctx,
                on_style_interpolated,
                plan.phase,
                plan.t,
                resolved.text_style,
                text_style,
                effect_name,
                "text"
            );
        }
    }
    // Also fire for fade effects when applied standalone
    if phase_effect.is_none() && fade.is_some() {
        inspect!(
            inspector_ctx,
            on_style_interpolated,
            plan.phase,
            plan.t,
            resolved.frame_style,
            frame_style,
            "FadeToBlack",
            "frame"
        );
        inspect!(
            inspector_ctx,
            on_style_interpolated,
            plan.phase,
            plan.t,
            resolved.border_style,
            border_style,
            "FadeToBlack",
            "border"
        );
        inspect!(
            inspector_ctx,
            on_style_interpolated,
            plan.phase,
            plan.t,
            resolved.text_style,
            text_style,
            "FadeToBlack",
            "text"
        );
    }

    // Add extra inner padding for vertical titles so content isn't cramped against the border
    let padding = adjust_padding_for_vertical_title(resolved.padding, &resolved.title);

    let mut block = Block::default()
        .borders(resolved.borders)
        .border_type(resolved.border_type)
        .border_style(border_style)
        .style(frame_style)
        .padding(padding)
        .border_set(resolved.border_set);

    // Apply title if configured (horizontal titles only - vertical handled after render)
    if let Some(ref title_cfg) = resolved.title {
        match title_cfg.position {
            TitlePosition::Top | TitlePosition::Bottom => {
                let alignment: Alignment = title_cfg.alignment.into();
                let title_text: String = title_cfg.text.clone();
                let title_line = Line::from(title_text).alignment(alignment);
                block = match title_cfg.position {
                    TitlePosition::Top => block.title_top(title_line),
                    TitlePosition::Bottom => block.title_bottom(title_line),
                    _ => block,
                };
            }
            TitlePosition::Left | TitlePosition::Right => {
                // Vertical titles rendered after main widget
            }
        }
    }

    let trim_args = BorderTrimArgs {
        policy: item.slide_border_trim(),
        animation: plan.animation,
        frame_area,
        dwell_rect: plan.dwell_rect,
        visible_area: plan.area,
        anchor: item.anchor(),
        slide_direction: item.slide_direction(),
        border_set: resolved.border_set,
    };
    block = apply_slide_border_trim(block, trim_args);

    struct ComposedWidget<W> {
        block: Block<'static>,
        inner: W,
    }
    impl<W: Widget> Widget for ComposedWidget<W> {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let inner_area = self.block.inner(area);
            self.block.render(area, buf);
            self.inner.render(inner_area, buf);
        }
    }

    let inner = build_inner(text_style, padding);
    let composed = ComposedWidget { block, inner };

    // Build CompositionOptions with shader_layers
    let mut options = CompositionOptions {
        masks: plan.masks.clone(),
        mask_combine_mode: plan.mask_combine_mode,
        filters: plan.filters.clone(),
        sampler_spec: plan.sampler_spec.clone(),
        t: plan.loop_t.unwrap_or(plan.t),
        loop_t: plan.loop_t,
        phase: Some(crate::compat::animation_phase_to_mixed(plan.phase)),
        ..Default::default()
    };

    // Add shader layers with their region constraints
    for (shader, region) in shader_entries {
        options = options.with_shader_layer(shader, region);
    }

    // Notify inspector of render plan creation
    inspect!(inspector_ctx, on_render_plan_created, plan);

    // Render widget content to the buffer
    composed.render(plan.area, buf);

    // Check if compositor effects are needed
    let has_effects = !options.masks.is_empty()
        || !options.filters.is_empty()
        || !options.shader_layers.is_empty()
        || options.sampler_spec.as_ref().is_some_and(|s| {
            !matches!(
                s,
                tui_vfx_compositor::types::cls_sampler_spec::SamplerSpec::None
            )
        });

    // Apply compositor pipeline effects if any are configured
    if has_effects {
        // Create a snapshot of the rendered content as the source Grid
        let source = RatatuiBufferSnapshot::from_region(
            buf,
            plan.area.x,
            plan.area.y,
            plan.area.width,
            plan.area.height,
        );

        // Clear the destination area before compositing so masked pixels
        // show through to the background (empty cells) rather than retaining
        // the pre-rendered widget content
        for y in plan.area.y..plan.area.y.saturating_add(plan.area.height) {
            for x in plan.area.x..plan.area.x.saturating_add(plan.area.width) {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.reset();
                }
            }
        }

        // Create an adapter for writing back to the buffer
        let mut dest = RatatuiBufferAdapter::with_offset(buf, plan.area.x, plan.area.y);

        // Run the compositor pipeline to apply effects
        render_pipeline(
            &source,
            &mut dest,
            plan.area.width as usize,
            plan.area.height as usize,
            0, // offset handled by adapter
            0,
            options,
            compositor_inspector,
        );
    }

    // Mark compositor context as used (for future expansion)
    let _ = ctx;

    // Render vertical titles (Left/Right positions) after main content
    if let Some(ref title_cfg) = resolved.title {
        if matches!(
            title_cfg.position,
            TitlePosition::Left | TitlePosition::Right
        ) {
            render_vertical_title(plan.area, buf, title_cfg, border_style);
        }
    }
}

fn resolve_style(
    base: Style,
    effect: Option<&StyleEffect>,
    fade: Option<tui_vfx_style::models::FadeToBlack>,
    t: f64,
) -> Style {
    use crate::compat::{ratatui_style_to_vfx, vfx_style_to_ratatui};

    let t = t.clamp(0.0, 1.0);
    let vfx_base = ratatui_style_to_vfx(base);
    let result = match (effect, fade) {
        (Some(inner), Some(fade)) => fade_effect(inner.clone(), fade).calculate(t, vfx_base),
        (Some(inner), None) => inner.calculate(t, vfx_base),
        (None, Some(fade)) => fade.calculate(t, vfx_base),
        (None, None) => vfx_base,
    };
    if result == vfx_base {
        return base;
    }
    vfx_style_to_ratatui(result)
}

/// Adjust padding to add extra space on the side with a vertical title.
/// This prevents content from being cramped against the border where the title is drawn.
fn adjust_padding_for_vertical_title(
    base: ratatui::widgets::Padding,
    title: &Option<crate::theme::TitleConfig>,
) -> ratatui::widgets::Padding {
    use crate::theme::TitlePosition;

    let Some(title_cfg) = title else {
        return base;
    };

    match title_cfg.position {
        TitlePosition::Left => ratatui::widgets::Padding {
            left: base.left.saturating_add(1),
            ..base
        },
        TitlePosition::Right => ratatui::widgets::Padding {
            right: base.right.saturating_add(1),
            ..base
        },
        _ => base,
    }
}

/// Render a vertical title on the left or right border.
/// Characters are drawn top-to-bottom. Alignment maps to vertical position:
/// - Left alignment → Top of border
/// - Center alignment → Centered vertically
/// - Right alignment → Bottom of border
fn render_vertical_title(
    area: Rect,
    buf: &mut Buffer,
    title_cfg: &crate::theme::TitleConfig,
    style: Style,
) {
    use crate::theme::{TitleAlignment, TitlePosition};

    if area.height < 3 || area.width < 2 {
        return; // Not enough space for border + title
    }

    let title_len = title_cfg.text.chars().count();

    // Available height for title (exclude top and bottom border cells)
    let available_height = (area.height - 2) as usize;
    if title_len == 0 || available_height == 0 {
        return;
    }

    // Determine x position (left or right border column)
    let x = match title_cfg.position {
        TitlePosition::Left => area.x,
        TitlePosition::Right => area.x + area.width.saturating_sub(1),
        _ => return, // Should not happen
    };

    // Calculate starting y based on alignment (mapped to vertical position)
    let chars_to_draw = title_len.min(available_height);
    let start_y = match title_cfg.alignment {
        TitleAlignment::Left => area.y + 1, // Top (after top border)
        TitleAlignment::Center => {
            let offset = (available_height.saturating_sub(chars_to_draw)) / 2;
            area.y + 1 + offset as u16
        }
        TitleAlignment::Right => {
            let offset = available_height.saturating_sub(chars_to_draw);
            area.y + 1 + offset as u16
        }
    };

    // Draw each character vertically
    for (i, ch) in title_cfg.text.chars().take(chars_to_draw).enumerate() {
        let y = start_y + i as u16;
        if y < area.y + area.height - 1 {
            // Within bounds (not on bottom border)
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char(ch);
                cell.set_style(style);
            }
        }
    }
}

struct BorderTrimArgs {
    policy: SlideBorderTrimPolicy,
    animation: Animation,
    frame_area: Rect,
    dwell_rect: Rect,
    visible_area: Rect,
    anchor: tui_vfx_geometry::types::Anchor,
    slide_direction: SlideDirection,
    border_set: border::Set<'static>,
}

fn apply_slide_border_trim<'a>(block: Block<'a>, args: BorderTrimArgs) -> Block<'a> {
    if args.policy == SlideBorderTrimPolicy::None || args.animation != Animation::Slide {
        return block;
    }
    if args.visible_area.width == 0
        || args.visible_area.height == 0
        || (args.visible_area.width == args.dwell_rect.width
            && args.visible_area.height == args.dwell_rect.height)
    {
        return block;
    }
    let effective_dir = resolve_slide_direction(args.slide_direction, args.anchor);
    let Some(spec) = vanishing_edge_trim_spec(
        effective_dir,
        crate::compat::ratatui_rect_to_vfx(args.frame_area),
        crate::compat::ratatui_rect_to_vfx(args.dwell_rect),
        crate::compat::ratatui_rect_to_vfx(args.visible_area),
    ) else {
        return block;
    };
    block.border_set(apply_trim_spec(args.border_set, spec))
}

fn apply_trim_spec(base: border::Set<'static>, spec: BorderTrimSpec) -> border::Set<'static> {
    fn seg_or_blank(seg: BorderSegment, keep: &'static str) -> &'static str {
        match seg {
            BorderSegment::Blank => " ",
            _ => keep,
        }
    }

    fn corner(
        seg: BorderSegment,
        horiz: &'static str,
        vert: &'static str,
        keep: &'static str,
    ) -> &'static str {
        match seg {
            BorderSegment::Blank => " ",
            BorderSegment::Horizontal => horiz,
            BorderSegment::Vertical => vert,
            _ => keep,
        }
    }

    border::Set {
        horizontal_top: seg_or_blank(spec.top, base.horizontal_top),
        horizontal_bottom: seg_or_blank(spec.bottom, base.horizontal_bottom),
        vertical_left: seg_or_blank(spec.left, base.vertical_left),
        vertical_right: seg_or_blank(spec.right, base.vertical_right),
        top_left: corner(
            spec.top_left,
            base.horizontal_top,
            base.vertical_left,
            base.top_left,
        ),
        top_right: corner(
            spec.top_right,
            base.horizontal_top,
            base.vertical_right,
            base.top_right,
        ),
        bottom_left: corner(
            spec.bottom_left,
            base.horizontal_bottom,
            base.vertical_left,
            base.bottom_left,
        ),
        bottom_right: corner(
            spec.bottom_right,
            base.horizontal_bottom,
            base.vertical_right,
            base.bottom_right,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;
    use tui_vfx_geometry::borders::BorderSegment;
    use tui_vfx_geometry::easing::EasingType;
    use tui_vfx_geometry::types::EasingCurve;
    use tui_vfx_style::models::{FadeApplyTo, FadeDirection, FadeToBlack, StyleEffect};

    #[test]
    fn test_apply_trim_preserves_border_set() {
        let base = border::ROUNDED;
        let spec = BorderTrimSpec {
            left: BorderSegment::Blank,
            top_left: BorderSegment::Blank,
            bottom_left: BorderSegment::Blank,
            ..BorderTrimSpec::none()
        };

        let trimmed = apply_trim_spec(base, spec);
        assert_eq!(trimmed.horizontal_top, base.horizontal_top);
        assert_eq!(trimmed.horizontal_bottom, base.horizontal_bottom);
        assert_eq!(trimmed.vertical_right, base.vertical_right);
        assert_eq!(trimmed.top_right, base.top_right);
        assert_eq!(trimmed.bottom_right, base.bottom_right);
        assert_eq!(trimmed.vertical_left, " ");
        assert_eq!(trimmed.top_left, " ");
        assert_eq!(trimmed.bottom_left, " ");
    }

    // =========================================================================
    // Phase Isolation Tests for resolve_style
    // =========================================================================
    // These tests verify that effects are applied correctly based on inputs.
    // The phase logic (Entering vs Dwelling/Exiting) is tested via integration.

    #[test]
    fn test_resolve_style_no_effect_no_fade_returns_base() {
        let base = Style::default().fg(Color::Red).bg(Color::Blue);
        let result = resolve_style(base, None, None, 0.5);
        assert_eq!(
            result, base,
            "With no effect or fade, base style should be returned unchanged"
        );
    }

    #[test]
    fn test_resolve_style_with_fade_applies_fade() {
        let base = Style::default().fg(Color::White);
        let fade = FadeToBlack {
            direction: FadeDirection::In,
            ..Default::default()
        };

        // At t=0.0 with FadeIn, should be fully black (fading IN from black)
        let result_start = resolve_style(base, None, Some(fade), 0.0);
        // At t=1.0 with FadeIn, should be original color
        let result_end = resolve_style(base, None, Some(fade), 1.0);

        // The fade effect should modify the style at t=0
        assert_ne!(result_start, base, "Fade at t=0 should modify the style");
        // At t=1.0, fade should be complete (style matches base)
        assert_eq!(
            result_end.fg, base.fg,
            "Fade at t=1.0 should return to base fg color"
        );
    }

    #[test]
    fn test_resolve_style_with_effect_applies_effect() {
        let base = Style::default().fg(Color::White);
        // Create a simple fade-in effect
        let effect = StyleEffect::FadeIn {
            apply_to: FadeApplyTo::Foreground,
            ease: EasingCurve::Type(EasingType::Linear),
        };

        let result_start = resolve_style(base, Some(&effect), None, 0.0);
        let result_end = resolve_style(base, Some(&effect), None, 1.0);

        // Effect should be applied - FadeIn produces different styles at different t values
        assert_ne!(
            result_start, result_end,
            "Effect should produce different results at different t values"
        );
    }

    #[test]
    fn test_resolve_style_none_effect_means_no_modification() {
        // This tests that passing None for effect doesn't crash and returns base
        let base = Style::default().fg(Color::Cyan).bg(Color::Magenta);
        let result = resolve_style(base, None, None, 0.5);
        assert_eq!(result, base);
    }
}

// <FILE>src/rendering/fnc_render_animated_with_theme.rs</FILE> - <DESC>Themed rendering entrypoint</DESC>
// <VERS>END OF VERSION: 3.2.3</VERS>
