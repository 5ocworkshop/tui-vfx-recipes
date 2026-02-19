// <FILE>src/preview/cls_preview_item.rs</FILE> - <DESC>Preview item for animation playback</DESC>
// <VERS>VERSION: 1.4.0</VERS>
// <WCTX>Custom frame content support</WCTX>
// <CLOG>Added frame field for direct-rendered custom borders with effect support</CLOG>

use crate::compat::ratatui_rect_to_vfx;
use crate::theme::{AppearanceConfig, FrameContent, HasAppearance};
use crate::traits::Animated;
use crate::types::{
    Animation, AnimationProfile, AutoDismiss, SlideBorderTrimPolicy, SlideExitDirection,
};
use ratatui::layout::Rect;
use tui_vfx_compositor::types::{FilterSpec, MaskCombineMode, MaskSpec, SamplerSpec};
use tui_vfx_content::types::ContentEffect;
use tui_vfx_geometry::transitions::{SlidePath, slide_path_offscreen_start_end};
use tui_vfx_geometry::types::{Anchor, SignedRect, SlideDirection};

/// A preview item for animation playback.
///
/// This is a standardized item for previewing recipes that implements
/// the Animated trait for use with AnimationManager.
#[derive(Debug, Clone)]
pub struct PreviewItem {
    pub message: String,
    pub anchor: Anchor,
    pub offset_h_percent: f32,
    pub offset_v_percent: f32,
    pub offset_h_cells: i16,
    pub offset_v_cells: i16,
    pub offset_h_pixels: i32,
    pub offset_v_pixels: i32,
    pub width: u16,
    pub height: u16,
    /// When true, scales to fill the entire terminal.
    pub fullscreen: bool,
    /// When true, auto-calculates padding to center the message content.
    pub center_content: bool,
    /// When true, text wraps. If false, it truncates.
    pub wrap: bool,
    pub animation: Animation,
    pub profile: AnimationProfile,
    pub auto_dismiss: AutoDismiss,
    pub slide_direction: SlideDirection,
    pub slide_exit_direction: SlideExitDirection,
    pub slide_border_trim: SlideBorderTrimPolicy,
    pub appearance: Option<AppearanceConfig>,
    pub content_effect: Option<ContentEffect>,
    // Pipeline specs (enter, dwell, exit phases)
    pub enter_mask: Option<MaskSpec>,
    pub dwell_mask: Option<MaskSpec>,
    pub exit_mask: Option<MaskSpec>,
    pub enter_sampler: Option<SamplerSpec>,
    pub dwell_sampler: Option<SamplerSpec>,
    pub exit_sampler: Option<SamplerSpec>,
    pub enter_filter: Option<FilterSpec>,
    pub dwell_filter: Option<FilterSpec>,
    pub exit_filter: Option<FilterSpec>,
    /// How to combine multiple masks (All=AND, Any=OR)
    pub mask_combine_mode: MaskCombineMode,
    /// Custom frame content for direct rendering (bypasses Block widget).
    /// When set, draws frame chars directly to buffer, enabling effects on borders.
    pub frame: Option<FrameContent>,
}

impl Default for PreviewItem {
    fn default() -> Self {
        Self {
            message: "Preview".to_string(),
            anchor: Anchor::BottomRight,
            offset_h_percent: 0.0,
            offset_v_percent: 0.0,
            offset_h_cells: 0,
            offset_v_cells: 0,
            offset_h_pixels: 0,
            offset_v_pixels: 0,
            width: 30,
            height: 3,
            fullscreen: false,
            center_content: false,
            wrap: true,
            animation: Animation::Slide,
            profile: AnimationProfile::default(),
            auto_dismiss: AutoDismiss::default(),
            slide_direction: SlideDirection::Default,
            slide_exit_direction: SlideExitDirection::SameAsEnter,
            slide_border_trim: SlideBorderTrimPolicy::None,
            appearance: None,
            content_effect: None,
            enter_mask: None,
            dwell_mask: None,
            exit_mask: None,
            enter_sampler: None,
            dwell_sampler: None,
            exit_sampler: None,
            enter_filter: None,
            dwell_filter: None,
            exit_filter: None,
            mask_combine_mode: MaskCombineMode::default(),
            frame: None,
        }
    }
}

impl PreviewItem {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            ..Default::default()
        }
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn offset_h_percent(mut self, offset: f32) -> Self {
        self.offset_h_percent = offset;
        self
    }

    pub fn offset_v_percent(mut self, offset: f32) -> Self {
        self.offset_v_percent = offset;
        self
    }

    pub fn offset_h_cells(mut self, offset: i16) -> Self {
        self.offset_h_cells = offset;
        self
    }

    pub fn offset_v_cells(mut self, offset: i16) -> Self {
        self.offset_v_cells = offset;
        self
    }

    pub fn offset_h_pixels(mut self, offset: i32) -> Self {
        self.offset_h_pixels = offset;
        self
    }

    pub fn offset_v_pixels(mut self, offset: i32) -> Self {
        self.offset_v_pixels = offset;
        self
    }

    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn center_content(mut self, center_content: bool) -> Self {
        self.center_content = center_content;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = animation;
        self
    }

    pub fn profile(mut self, profile: AnimationProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn auto_dismiss(mut self, auto_dismiss: AutoDismiss) -> Self {
        self.auto_dismiss = auto_dismiss;
        self
    }

    pub fn slide_border_trim(mut self, policy: SlideBorderTrimPolicy) -> Self {
        self.slide_border_trim = policy;
        self
    }

    pub fn content_effect(mut self, effect: ContentEffect) -> Self {
        self.content_effect = Some(effect);
        self
    }

    pub fn appearance(mut self, appearance: AppearanceConfig) -> Self {
        self.appearance = Some(appearance);
        self
    }

    pub fn enter_mask(mut self, mask: MaskSpec) -> Self {
        self.enter_mask = Some(mask);
        self
    }

    pub fn exit_mask(mut self, mask: MaskSpec) -> Self {
        self.exit_mask = Some(mask);
        self
    }

    pub fn enter_sampler(mut self, sampler: SamplerSpec) -> Self {
        self.enter_sampler = Some(sampler);
        self
    }

    pub fn exit_sampler(mut self, sampler: SamplerSpec) -> Self {
        self.exit_sampler = Some(sampler);
        self
    }

    pub fn enter_filter(mut self, filter: FilterSpec) -> Self {
        self.enter_filter = Some(filter);
        self
    }

    pub fn exit_filter(mut self, filter: FilterSpec) -> Self {
        self.exit_filter = Some(filter);
        self
    }

    pub fn dwell_mask(mut self, mask: MaskSpec) -> Self {
        self.dwell_mask = Some(mask);
        self
    }

    pub fn dwell_sampler(mut self, sampler: SamplerSpec) -> Self {
        self.dwell_sampler = Some(sampler);
        self
    }

    pub fn dwell_filter(mut self, filter: FilterSpec) -> Self {
        self.dwell_filter = Some(filter);
        self
    }

    pub fn enter_style(mut self, effect: tui_vfx_style::models::StyleEffect) -> Self {
        self.profile.enter_style = Some(effect);
        self
    }

    pub fn dwell_style(mut self, effect: tui_vfx_style::models::StyleEffect) -> Self {
        self.profile.dwell_style = Some(effect);
        self
    }

    pub fn exit_style(mut self, effect: tui_vfx_style::models::StyleEffect) -> Self {
        self.profile.exit_style = Some(effect);
        self
    }

    pub fn mask_combine_mode(mut self, mode: MaskCombineMode) -> Self {
        self.mask_combine_mode = mode;
        self
    }

    pub fn frame(mut self, frame: FrameContent) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Get the frame content if set.
    pub fn get_frame(&self) -> Option<&FrameContent> {
        self.frame.as_ref()
    }
}

impl HasAppearance for PreviewItem {
    fn appearance(&self) -> Option<&AppearanceConfig> {
        self.appearance.as_ref()
    }
}

impl Animated for PreviewItem {
    fn anchor(&self) -> Anchor {
        self.anchor
    }

    fn offset_h_percent(&self) -> f32 {
        self.offset_h_percent
    }

    fn offset_v_percent(&self) -> f32 {
        self.offset_v_percent
    }

    fn offset_h_cells(&self) -> i16 {
        self.offset_h_cells
    }

    fn offset_v_cells(&self) -> i16 {
        self.offset_v_cells
    }

    fn offset_h_pixels(&self) -> i32 {
        self.offset_h_pixels
    }

    fn offset_v_pixels(&self) -> i32 {
        self.offset_v_pixels
    }

    fn profile(&self) -> &AnimationProfile {
        &self.profile
    }

    fn animation(&self) -> Animation {
        self.animation
    }

    fn exit_animation(&self) -> Option<Animation> {
        None
    }

    fn auto_dismiss(&self) -> AutoDismiss {
        self.auto_dismiss
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn exterior_margin(&self) -> u16 {
        0
    }

    fn slide_direction(&self) -> SlideDirection {
        self.slide_direction
    }

    fn slide_exit_direction(&self) -> SlideExitDirection {
        self.slide_exit_direction
    }

    fn slide_border_trim(&self) -> SlideBorderTrimPolicy {
        self.slide_border_trim
    }

    fn slide_path(&self, frame: Rect, dwell: Rect) -> SlidePath {
        let vfx_dwell = ratatui_rect_to_vfx(dwell);
        let dwell_signed = SignedRect::from(vfx_dwell);
        let exit_dir = self.slide_exit_direction.resolve(self.slide_direction);
        slide_path_offscreen_start_end(
            ratatui_rect_to_vfx(frame),
            self.anchor,
            self.slide_direction,
            exit_dir,
            dwell_signed,
        )
    }

    fn enter_mask(&self) -> Option<&MaskSpec> {
        self.enter_mask.as_ref()
    }

    fn exit_mask(&self) -> Option<&MaskSpec> {
        self.exit_mask.as_ref()
    }

    fn enter_sampler(&self) -> Option<&SamplerSpec> {
        self.enter_sampler.as_ref()
    }

    fn exit_sampler(&self) -> Option<&SamplerSpec> {
        self.exit_sampler.as_ref()
    }

    fn enter_filter(&self) -> Option<&FilterSpec> {
        self.enter_filter.as_ref()
    }

    fn exit_filter(&self) -> Option<&FilterSpec> {
        self.exit_filter.as_ref()
    }

    fn dwell_mask(&self) -> Option<&MaskSpec> {
        self.dwell_mask.as_ref()
    }

    fn dwell_sampler(&self) -> Option<&SamplerSpec> {
        self.dwell_sampler.as_ref()
    }

    fn dwell_filter(&self) -> Option<&FilterSpec> {
        self.dwell_filter.as_ref()
    }

    fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    fn is_center_content(&self) -> bool {
        self.center_content
    }

    fn mask_combine_mode(&self) -> MaskCombineMode {
        self.mask_combine_mode
    }
}

// <FILE>src/preview/cls_preview_item.rs</FILE> - <DESC>Preview item for animation playback</DESC>
// <VERS>END OF VERSION: 1.4.0</VERS>
