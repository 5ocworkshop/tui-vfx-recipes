// <FILE>src/traits/mod.rs</FILE> - <DESC>Animated trait with slide_path</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Multiple filters/shaders/masks per stage</WCTX>
// <CLOG>Added multi-effect trait methods (enter_masks, enter_filters, mask_combine_mode, etc.)</CLOG>

use crate::types::{
    Animation, AnimationProfile, AutoDismiss, SlideBorderTrimPolicy, SlideExitDirection,
};
use ratatui::layout::Rect;
use std::borrow::Cow;
use tui_vfx_compositor::types::{FilterSpec, MaskCombineMode, MaskSpec, SamplerSpec};
use tui_vfx_geometry::transitions::SlidePath;
use tui_vfx_geometry::types::{Anchor, SlideDirection};

pub use crate::theme::HasAppearance;
/// Trait for objects that can be managed by the AnimationManager.
pub trait Animated {
    fn anchor(&self) -> Anchor;
    fn offset_h_percent(&self) -> f32 {
        0.0
    }
    fn offset_v_percent(&self) -> f32 {
        0.0
    }
    fn offset_h_cells(&self) -> i16 {
        0.0 as i16
    }
    fn offset_v_cells(&self) -> i16 {
        0.0 as i16
    }
    fn offset_h_pixels(&self) -> i32 {
        0
    }
    fn offset_v_pixels(&self) -> i32 {
        0
    }
    fn profile(&self) -> &AnimationProfile;
    fn animation(&self) -> Animation;
    fn exit_animation(&self) -> Option<Animation>;
    fn auto_dismiss(&self) -> AutoDismiss;
    // Geometry required for layout planning
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn exterior_margin(&self) -> u16;
    // Slide specifics
    fn slide_direction(&self) -> SlideDirection;
    fn slide_exit_direction(&self) -> SlideExitDirection;
    fn slide_border_trim(&self) -> SlideBorderTrimPolicy;
    /// Calculate the slide path for this item given the frame and its stacked dwell position.
    fn slide_path(&self, frame: Rect, dwell: Rect) -> SlidePath;
    // V2 Pipeline specs (optional, with default implementations)
    /// Get the mask spec for enter transition (V2 pipeline).
    fn enter_mask(&self) -> Option<&MaskSpec> {
        None
    }
    /// Get the mask spec for exit transition (V2 pipeline).
    fn exit_mask(&self) -> Option<&MaskSpec> {
        None
    }
    /// Get the sampler spec for enter transition (V2 pipeline).
    fn enter_sampler(&self) -> Option<&SamplerSpec> {
        None
    }
    /// Get the sampler spec for exit transition (V2 pipeline).
    fn exit_sampler(&self) -> Option<&SamplerSpec> {
        None
    }
    /// Get the filter spec for enter transition (V2 pipeline).
    fn enter_filter(&self) -> Option<&FilterSpec> {
        None
    }
    /// Get the filter spec for exit transition (V2 pipeline).
    fn exit_filter(&self) -> Option<&FilterSpec> {
        None
    }
    /// Get the mask spec for dwell phase (V2 pipeline).
    fn dwell_mask(&self) -> Option<&MaskSpec> {
        None
    }
    /// Get the sampler spec for dwell phase (V2 pipeline).
    fn dwell_sampler(&self) -> Option<&SamplerSpec> {
        None
    }
    /// Get the filter spec for dwell phase (V2 pipeline).
    fn dwell_filter(&self) -> Option<&FilterSpec> {
        None
    }
    /// Whether this item should fill the entire frame (fullscreen mode).
    /// When true, the item's width/height are ignored and it fills the terminal.
    fn is_fullscreen(&self) -> bool {
        false
    }
    /// Whether content should be auto-centered within the item area.
    /// When true, padding is calculated to center the message content.
    fn is_center_content(&self) -> bool {
        false
    }

    // Multi-effect trait methods (new)

    /// Get all mask specs for enter transition.
    /// Default implementation wraps the single enter_mask() for backward compatibility.
    fn enter_masks(&self) -> Cow<'_, [MaskSpec]> {
        self.enter_mask()
            .map(|mask| Cow::Borrowed(std::slice::from_ref(mask)))
            .unwrap_or(Cow::Borrowed(&[]))
    }

    /// Get all mask specs for dwell phase.
    fn dwell_masks(&self) -> Cow<'_, [MaskSpec]> {
        self.dwell_mask()
            .map(|mask| Cow::Borrowed(std::slice::from_ref(mask)))
            .unwrap_or(Cow::Borrowed(&[]))
    }

    /// Get all mask specs for exit transition.
    fn exit_masks(&self) -> Cow<'_, [MaskSpec]> {
        self.exit_mask()
            .map(|mask| Cow::Borrowed(std::slice::from_ref(mask)))
            .unwrap_or(Cow::Borrowed(&[]))
    }

    /// Get all filter specs for enter transition.
    fn enter_filters(&self) -> Cow<'_, [FilterSpec]> {
        self.enter_filter()
            .map(|filter| Cow::Borrowed(std::slice::from_ref(filter)))
            .unwrap_or(Cow::Borrowed(&[]))
    }

    /// Get all filter specs for dwell phase.
    fn dwell_filters(&self) -> Cow<'_, [FilterSpec]> {
        self.dwell_filter()
            .map(|filter| Cow::Borrowed(std::slice::from_ref(filter)))
            .unwrap_or(Cow::Borrowed(&[]))
    }

    /// Get all filter specs for exit transition.
    fn exit_filters(&self) -> Cow<'_, [FilterSpec]> {
        self.exit_filter()
            .map(|filter| Cow::Borrowed(std::slice::from_ref(filter)))
            .unwrap_or(Cow::Borrowed(&[]))
    }

    /// Get the mask combine mode (All=AND, Any=OR).
    fn mask_combine_mode(&self) -> MaskCombineMode {
        MaskCombineMode::default()
    }
}

// <FILE>src/traits/mod.rs</FILE> - <DESC>Animated trait with slide_path</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
