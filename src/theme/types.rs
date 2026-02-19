// <FILE>src/theme/types.rs</FILE> - <DESC>Serde + schema friendly theme/appearance types</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Custom frame content support</WCTX>
// <CLOG>Added FrameContent struct with draw_to_buffer for direct-rendered borders</CLOG>

use crate::compat::ratatui_style_to_config;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::widgets::{BorderType, Borders, Padding};
use serde::{Deserialize, Serialize};
use tui_vfx_style::models::{FadeDirection, FadeToBlack, StyleConfig};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct Theme {
    pub defaults: AppearanceConfig,
}

impl Default for Theme {
    fn default() -> Self {
        let default_style = ratatui_style_to_config(Style::default().fg(Color::White));

        Self {
            defaults: AppearanceConfig {
                chrome: Some(ChromeConfig {
                    borders: Some(BordersConfig::all()),
                    border_type: Some(BorderTypeConfig::Rounded),
                    border_set: Some(BorderSetConfig::Rounded),
                    custom_border_set: None,
                    padding: Some(PaddingConfig::zero()),
                    frame_style: Some(default_style.clone()),
                    border_style: Some(default_style.clone()),
                    title: None,
                }),
                text: Some(TextConfig {
                    style: Some(default_style),
                }),
                fade: None,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct AppearanceConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrome: Option<ChromeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fade: Option<FadeConfig>,
}

/// Title position on the border.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum TitlePosition {
    #[default]
    Top,
    Bottom,
    /// Vertical title on left border (rendered top-to-bottom)
    Left,
    /// Vertical title on right border (rendered top-to-bottom)
    Right,
}

/// Title horizontal alignment within the border.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum TitleAlignment {
    #[default]
    Left,
    Center,
    Right,
}

impl From<TitleAlignment> for Alignment {
    fn from(value: TitleAlignment) -> Self {
        match value {
            TitleAlignment::Left => Alignment::Left,
            TitleAlignment::Center => Alignment::Center,
            TitleAlignment::Right => Alignment::Right,
        }
    }
}

/// Configuration for a border title.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct TitleConfig {
    /// The title text.
    pub text: String,
    /// Vertical position (Top or Bottom).
    #[serde(default)]
    pub position: TitlePosition,
    /// Horizontal alignment (Left, Center, Right).
    #[serde(default)]
    pub alignment: TitleAlignment,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct ChromeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borders: Option<BordersConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_type: Option<BorderTypeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_set: Option<BorderSetConfig>,
    /// Custom border characters (braille, blocks, etc.) - overrides border_set when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_border_set: Option<CustomBorderSet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<PaddingConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_style: Option<StyleConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_style: Option<StyleConfig>,
    /// Optional title displayed on the border.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<TitleConfig>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct TextConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleConfig>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
pub struct FadeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter: Option<FadeToBlack>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<FadeToBlack>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct BordersConfig {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

impl BordersConfig {
    pub const fn all() -> Self {
        Self {
            top: true,
            right: true,
            bottom: true,
            left: true,
        }
    }
}

impl Default for BordersConfig {
    fn default() -> Self {
        Self::all()
    }
}

impl From<BordersConfig> for Borders {
    fn from(value: BordersConfig) -> Self {
        let mut out = Borders::NONE;
        if value.top {
            out |= Borders::TOP;
        }
        if value.right {
            out |= Borders::RIGHT;
        }
        if value.bottom {
            out |= Borders::BOTTOM;
        }
        if value.left {
            out |= Borders::LEFT;
        }
        out
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum BorderTypeConfig {
    #[default]
    Plain,
    Rounded,
    Double,
    Thick,
    QuadrantInside,
    QuadrantOutside,
}

impl From<BorderTypeConfig> for BorderType {
    fn from(value: BorderTypeConfig) -> Self {
        match value {
            BorderTypeConfig::Plain => BorderType::Plain,
            BorderTypeConfig::Rounded => BorderType::Rounded,
            BorderTypeConfig::Double => BorderType::Double,
            BorderTypeConfig::Thick => BorderType::Thick,
            BorderTypeConfig::QuadrantInside => BorderType::QuadrantInside,
            BorderTypeConfig::QuadrantOutside => BorderType::QuadrantOutside,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum BorderSetConfig {
    #[default]
    Plain,
    Rounded,
    Double,
    Thick,
    QuadrantOutside,
    QuadrantInside,
    OneEighthWide,
    OneEighthTall,
    ProportionalWide,
    ProportionalTall,
    Full,
    Empty,
}

impl From<BorderSetConfig> for border::Set<'static> {
    fn from(value: BorderSetConfig) -> Self {
        match value {
            BorderSetConfig::Plain => border::PLAIN,
            BorderSetConfig::Rounded => border::ROUNDED,
            BorderSetConfig::Double => border::DOUBLE,
            BorderSetConfig::Thick => border::THICK,
            BorderSetConfig::QuadrantOutside => border::QUADRANT_OUTSIDE,
            BorderSetConfig::QuadrantInside => border::QUADRANT_INSIDE,
            BorderSetConfig::OneEighthWide => border::ONE_EIGHTH_WIDE,
            BorderSetConfig::OneEighthTall => border::ONE_EIGHTH_TALL,
            BorderSetConfig::ProportionalWide => border::PROPORTIONAL_WIDE,
            BorderSetConfig::ProportionalTall => border::PROPORTIONAL_TALL,
            BorderSetConfig::Full => border::FULL,
            BorderSetConfig::Empty => border::EMPTY,
        }
    }
}

/// Custom border characters for artistic borders (braille, blocks, etc.)
/// Note: Converting to border::Set leaks strings to get 'static lifetime,
/// which is acceptable for configuration that lives for the program's duration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct CustomBorderSet {
    pub top_left: String,
    pub top_right: String,
    pub bottom_left: String,
    pub bottom_right: String,
    pub horizontal_top: String,
    pub horizontal_bottom: String,
    pub vertical_left: String,
    pub vertical_right: String,
}

impl CustomBorderSet {
    /// Convert to ratatui's border::Set by leaking strings to static lifetime.
    /// This is acceptable for configuration that lives for the program's duration.
    pub fn to_border_set(&self) -> border::Set<'static> {
        border::Set {
            top_left: Box::leak(self.top_left.clone().into_boxed_str()),
            top_right: Box::leak(self.top_right.clone().into_boxed_str()),
            bottom_left: Box::leak(self.bottom_left.clone().into_boxed_str()),
            bottom_right: Box::leak(self.bottom_right.clone().into_boxed_str()),
            horizontal_top: Box::leak(self.horizontal_top.clone().into_boxed_str()),
            horizontal_bottom: Box::leak(self.horizontal_bottom.clone().into_boxed_str()),
            vertical_left: Box::leak(self.vertical_left.clone().into_boxed_str()),
            vertical_right: Box::leak(self.vertical_right.clone().into_boxed_str()),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
pub struct PaddingConfig {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl PaddingConfig {
    pub const fn zero() -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl From<PaddingConfig> for Padding {
    fn from(value: PaddingConfig) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}

/// Frame content for direct rendering (bypasses Block widget).
/// Unlike CustomBorderSet (which uses ratatui's border system), frame content
/// is drawn directly to the buffer as renderable content. This allows:
/// - Multi-character edge patterns (repeated to fill)
/// - Full effect/shader support via StyleRegion::BorderOnly
/// - Animated borders through content effects
/// - Per-cell styling of frame elements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct FrameContent {
    /// Top-left corner (can be multi-char)
    pub top_left: String,
    /// Top-right corner (can be multi-char)
    pub top_right: String,
    /// Bottom-left corner (can be multi-char)
    pub bottom_left: String,
    /// Bottom-right corner (can be multi-char)
    pub bottom_right: String,
    /// Top edge pattern (repeated to fill)
    pub top: String,
    /// Bottom edge pattern (repeated to fill)
    pub bottom: String,
    /// Left edge pattern (repeated to fill)
    pub left: String,
    /// Right edge pattern (repeated to fill)
    pub right: String,
}

impl FrameContent {
    /// Draw the frame to a buffer at the given area.
    /// The frame is drawn in the border cells (outermost row/column).
    pub fn draw_to_buffer(
        &self,
        buffer: &mut ratatui::buffer::Buffer,
        area: ratatui::layout::Rect,
        style: Style,
    ) {
        if area.width < 2 || area.height < 2 {
            return; // Too small for a frame
        }

        let top_row = area.y;
        let bottom_row = area.y + area.height.saturating_sub(1);
        let left_col = area.x;
        let right_col = area.x + area.width.saturating_sub(1);

        // Draw corners (assuming single-char corners for simplicity)
        self.draw_string(buffer, left_col, top_row, &self.top_left, style);
        self.draw_string(buffer, right_col, top_row, &self.top_right, style);
        self.draw_string(buffer, left_col, bottom_row, &self.bottom_left, style);
        self.draw_string(buffer, right_col, bottom_row, &self.bottom_right, style);

        // Draw top edge (between corners)
        let edge_start = left_col + 1;
        let edge_end = right_col;
        self.draw_repeated_horizontal(buffer, edge_start, edge_end, top_row, &self.top, style);

        // Draw bottom edge
        self.draw_repeated_horizontal(
            buffer,
            edge_start,
            edge_end,
            bottom_row,
            &self.bottom,
            style,
        );

        // Draw left edge (between corners)
        for row in (top_row + 1)..bottom_row {
            self.draw_string(buffer, left_col, row, &self.left, style);
        }

        // Draw right edge
        for row in (top_row + 1)..bottom_row {
            self.draw_string(buffer, right_col, row, &self.right, style);
        }
    }

    fn draw_string(
        &self,
        buffer: &mut ratatui::buffer::Buffer,
        x: u16,
        y: u16,
        s: &str,
        style: Style,
    ) {
        let mut col = x;
        for c in s.chars() {
            if col < buffer.area.x + buffer.area.width {
                if let Some(cell) = buffer.cell_mut((col, y)) {
                    cell.set_char(c);
                    cell.set_style(style);
                }
                col += 1; // Assume single-width chars for frame elements
            }
        }
    }

    fn draw_repeated_horizontal(
        &self,
        buffer: &mut ratatui::buffer::Buffer,
        start: u16,
        end: u16,
        y: u16,
        pattern: &str,
        style: Style,
    ) {
        if pattern.is_empty() || start >= end {
            return;
        }
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let mut col = start;
        let mut char_idx = 0;
        while col < end {
            let c = pattern_chars[char_idx % pattern_chars.len()];
            if let Some(cell) = buffer.cell_mut((col, y)) {
                cell.set_char(c);
                cell.set_style(style);
            }
            col += 1; // Assume single-width chars for frame elements
            char_idx += 1;
        }
    }
}

/// Minimal contract for a domain object to provide an appearance override.
pub trait HasAppearance {
    fn appearance(&self) -> Option<&AppearanceConfig> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedAppearance {
    pub frame_style: Style,
    pub border_style: Style,
    pub text_style: Style,
    pub borders: Borders,
    pub border_type: BorderType,
    pub border_set: border::Set<'static>,
    pub padding: Padding,
    pub fade_enter: Option<FadeToBlack>,
    pub fade_exit: Option<FadeToBlack>,
    /// Optional title for the border.
    pub title: Option<TitleConfig>,
}

impl ResolvedAppearance {
    pub fn fade_for_phase(&self) -> Option<FadeToBlack> {
        self.fade_enter.or(self.fade_exit)
    }

    pub fn with_forced_fade_direction(mut self) -> Self {
        if let Some(mut fade) = self.fade_enter {
            fade.direction = FadeDirection::In;
            self.fade_enter = Some(fade);
        }
        if let Some(mut fade) = self.fade_exit {
            fade.direction = FadeDirection::Out;
            self.fade_exit = Some(fade);
        }
        self
    }
}

// <FILE>src/theme/types.rs</FILE> - <DESC>Serde + schema friendly theme/appearance types</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
