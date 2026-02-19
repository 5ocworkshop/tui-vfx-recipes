// <FILE>src/preview/cls_preview_manager.rs</FILE> - <DESC>Preview manager for animation playback</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Pipeline stage inspection implementation</WCTX>
// <CLOG>Update render_inspected for new InspectorContext parameter

use super::cls_preview_item::PreviewItem;
use super::fnc_render_preview_item::{render_preview_item, render_preview_item_inspected};
use crate::inspector::InspectorContext;
use crate::manager::AnimationManager;
use crate::rendering::RenderPlanItem;
use crate::state::LifecycleState;
use crate::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::time::Instant;
use tui_vfx_compositor::context::cls_compositor_ctx::CompositorCtx;
use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;

/// Manager for preview items.
///
/// Wraps AnimationManager<PreviewItem> with rendering capabilities.
/// This provides an accurate playback pipeline matching production use.
#[derive(Debug)]
pub struct PreviewManager {
    manager: AnimationManager<PreviewItem>,
    ctx: CompositorCtx,
    theme: Theme,
}

impl Default for PreviewManager {
    fn default() -> Self {
        Self {
            manager: AnimationManager::default(),
            ctx: CompositorCtx::new(),
            theme: Theme::default(),
        }
    }
}

impl PreviewManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn add(&mut self, item: PreviewItem, now: Instant) -> Option<u64> {
        self.manager.add(item, now)
    }

    pub fn clear(&mut self) {
        self.manager.clear();
    }

    pub fn tick(&mut self, now: Instant) {
        self.manager.tick(now);
    }

    pub fn states(&self) -> impl Iterator<Item = &LifecycleState<PreviewItem>> {
        self.manager.states()
    }

    pub fn render_plan(&self, frame_area: Rect, now: Instant) -> Vec<RenderPlanItem<'_>> {
        self.manager.render_plan(frame_area, now)
    }

    pub fn render(&mut self, frame_area: Rect, buf: &mut Buffer, now: Instant) {
        let plan = self.manager.render_plan(frame_area, now);
        for item in plan {
            let Some(state) = self.manager.get_state(item.id) else {
                continue;
            };
            render_preview_item(
                &state.item,
                &item,
                &self.theme,
                frame_area,
                buf,
                &mut self.ctx,
            );
        }
    }

    /// Render with compositor inspection.
    ///
    /// Same as [`render`], but passes an inspector through the pipeline for
    /// cell-by-cell tracing of mask, shader, and filter operations.
    ///
    /// Note: This method only provides compositor-level inspection. For full
    /// pipeline inspection including style interpolation, use
    /// `render_preview_item_inspected` directly with an InspectorContext.
    pub fn render_inspected(
        &mut self,
        frame_area: Rect,
        buf: &mut Buffer,
        now: Instant,
        inspector: &mut dyn CompositorInspector,
    ) {
        let plan = self.manager.render_plan(frame_area, now);
        for item in plan {
            let Some(state) = self.manager.get_state(item.id) else {
                continue;
            };
            let mut inspector_ctx = InspectorContext::none();
            render_preview_item_inspected(
                &state.item,
                &item,
                &self.theme,
                frame_area,
                buf,
                &mut self.ctx,
                &mut inspector_ctx,
                inspector,
            );
        }
    }

    /// Get mutable access to the underlying animation manager
    pub fn manager_mut(&mut self) -> &mut AnimationManager<PreviewItem> {
        &mut self.manager
    }
}

// <FILE>src/preview/cls_preview_manager.rs</FILE> - <DESC>Preview manager for animation playback</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
