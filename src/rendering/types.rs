// <FILE>src/rendering/types.rs</FILE> - <DESC>Render plan item type shared by rendering + planner</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Clippy cleanup for render plan construction</WCTX>
// <CLOG>Allow multi-argument constructor for RenderPlanItem</CLOG>

use crate::state::AnimationPhase;
use crate::types::Animation;
use ratatui::layout::Rect;
use std::borrow::Cow;
use tui_vfx_compositor::types::{FilterSpec, MaskCombineMode, MaskSpec, SamplerSpec};
use tui_vfx_geometry::types::SignedRect;

/// A single item in the render plan with all pipeline configuration.
///
/// This struct carries everything needed to render an animated item:
/// - Identity and positioning (id, anchor, area, dwell_rect)
/// - Animation state (phase, animation type, progress t)
/// - Pipeline specs (mask, sampler, filter)
///
/// The pipeline specs support multiple effects per stage (masks, filters).
#[derive(Debug, Clone)]
pub struct RenderPlanItem<'a> {
    /// Unique identifier for this item
    pub id: u64,
    /// Anchor position for layout
    pub anchor: tui_vfx_geometry::types::Anchor,
    /// Horizontal offset as percentage of frame width
    pub offset_h_percent: f32,
    /// Vertical offset as percentage of frame height
    pub offset_v_percent: f32,
    /// Horizontal offset in cells
    pub offset_h_cells: i16,
    /// Vertical offset in cells
    pub offset_v_cells: i16,
    /// Horizontal offset in pixels
    pub offset_h_pixels: i32,
    /// Vertical offset in pixels
    pub offset_v_pixels: i32,
    /// Current animation phase (Entering, Dwelling, Exiting, etc.)
    pub phase: AnimationPhase,
    /// Animation type preset
    pub animation: Animation,
    /// The dwell (resting) rectangle for this item
    pub dwell_rect: Rect,
    /// Current animated area (may differ from dwell during transitions)
    pub area: Rect,
    /// Unclamped animated area (can have negative coordinates for off-screen positions)
    /// Used for sixel rendering which needs true position before viewport clamping
    pub signed_area: SignedRect,
    /// Animation progress (0.0 to 1.0)
    pub t: f64,

    // Pipeline configuration
    /// Mask specifications (multiple masks combined via mask_combine_mode)
    pub masks: Cow<'a, [MaskSpec]>,
    /// How to combine multiple masks (All=AND, Any=OR)
    pub mask_combine_mode: MaskCombineMode,
    /// Filter specifications (applied in order)
    pub filters: Cow<'a, [FilterSpec]>,
    /// Sampler specification for distortion effects
    pub sampler_spec: Option<SamplerSpec>,
    /// Loop progress for time-loop effects (0.0 to 1.0, repeating)
    pub loop_t: Option<f64>,
}

impl<'a> RenderPlanItem<'a> {
    /// Create a new render plan item with minimal required fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u64,
        anchor: tui_vfx_geometry::types::Anchor,
        offset_h_percent: f32,
        offset_v_percent: f32,
        offset_h_cells: i16,
        offset_v_cells: i16,
        offset_h_pixels: i32,
        offset_v_pixels: i32,
        phase: AnimationPhase,
        animation: Animation,
        dwell_rect: Rect,
        area: Rect,
        signed_area: SignedRect,
        t: f64,
    ) -> Self {
        Self {
            id,
            anchor,
            offset_h_percent,
            offset_v_percent,
            offset_h_cells,
            offset_v_cells,
            offset_h_pixels,
            offset_v_pixels,
            phase,
            animation,
            dwell_rect,
            area,
            signed_area,
            t,
            masks: Cow::Borrowed(&[]),
            mask_combine_mode: MaskCombineMode::default(),
            filters: Cow::Borrowed(&[]),
            sampler_spec: None,
            loop_t: None,
        }
    }

    /// Set the sampler specification.
    pub fn with_sampler(mut self, sampler: SamplerSpec) -> Self {
        self.sampler_spec = Some(sampler);
        self
    }

    /// Set the loop progress for time-loop effects.
    pub fn with_loop_t(mut self, loop_t: f64) -> Self {
        self.loop_t = Some(loop_t);
        self
    }

    /// Set multiple mask specifications.
    pub fn with_masks(mut self, masks: impl Into<Cow<'a, [MaskSpec]>>) -> Self {
        self.masks = masks.into();
        self
    }

    /// Set the mask combine mode (All=AND, Any=OR).
    pub fn with_mask_combine_mode(mut self, mode: MaskCombineMode) -> Self {
        self.mask_combine_mode = mode;
        self
    }

    /// Set multiple filter specifications.
    pub fn with_filters(mut self, filters: impl Into<Cow<'a, [FilterSpec]>>) -> Self {
        self.filters = filters.into();
        self
    }
}

// <FILE>src/rendering/types.rs</FILE> - <DESC>Render plan item type shared by rendering + planner</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
