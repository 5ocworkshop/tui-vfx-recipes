// <FILE>src/compat.rs</FILE> - <DESC>Compatibility conversions between ratatui and tui-vfx types</DESC>
// <VERS>VERSION: 0.4.3</VERS>
// <WCTX>Support modifier alpha override in cell conversions</WCTX>
// <CLOG>Add mod_alpha default for ratatui cell conversions</CLOG>

//! Compatibility module for converting between ratatui types and tui-vfx types.
//!
//! This module provides the glue between:
//! - `ratatui::style::Style` ↔ `tui_vfx_types::Style`
//! - `ratatui::layout::Rect` ↔ `tui_vfx_types::Rect`
//! - `ratatui::style::Color` ↔ `tui_vfx_types::Color`
//!
//! Note: tui-vfx uses RGBA structs while ratatui uses enums. Some information
//! loss may occur (e.g., Indexed colors become RGB approximations).

use ratatui::buffer::Cell as RatatuiCell;
use ratatui::layout::Rect as RatatuiRect;
use ratatui::style::{Color as RatatuiColor, Modifier, Style as RatatuiStyle};
use tui_vfx_style::models::{ColorConfig, ModifierConfig, StyleConfig};
use tui_vfx_types::{
    Cell as VfxCell, Color as VfxColor, Modifiers, Rect as VfxRect, Style as VfxStyle,
};

// =============================================================================
// StyleConfig ↔ ratatui::Style conversions
// =============================================================================

/// Convert StyleConfig to ratatui Style
pub fn style_config_to_ratatui(config: &StyleConfig) -> RatatuiStyle {
    let mut style = RatatuiStyle::default();

    if let Some(ref fg) = config.fg {
        style = style.fg(color_config_to_ratatui(fg));
    }
    if let Some(ref bg) = config.bg {
        style = style.bg(color_config_to_ratatui(bg));
    }

    let mut modifiers = Modifier::empty();
    for m in &config.add_modifier {
        modifiers |= modifier_config_to_ratatui(m);
    }
    if !modifiers.is_empty() {
        style = style.add_modifier(modifiers);
    }

    style
}

/// Convert ColorConfig to ratatui Color
pub fn color_config_to_ratatui(config: &ColorConfig) -> RatatuiColor {
    match config {
        ColorConfig::Rgb { r, g, b } => RatatuiColor::Rgb(*r, *g, *b),
        ColorConfig::Indexed { value } => RatatuiColor::Indexed(*value),
        ColorConfig::Reset => RatatuiColor::Reset,
        ColorConfig::Black => RatatuiColor::Black,
        ColorConfig::Red => RatatuiColor::Red,
        ColorConfig::Green => RatatuiColor::Green,
        ColorConfig::Yellow => RatatuiColor::Yellow,
        ColorConfig::Blue => RatatuiColor::Blue,
        ColorConfig::Magenta => RatatuiColor::Magenta,
        ColorConfig::Cyan => RatatuiColor::Cyan,
        ColorConfig::Gray | ColorConfig::LightGray => RatatuiColor::Gray,
        ColorConfig::DarkGray => RatatuiColor::DarkGray,
        ColorConfig::LightRed => RatatuiColor::LightRed,
        ColorConfig::LightGreen => RatatuiColor::LightGreen,
        ColorConfig::LightYellow => RatatuiColor::LightYellow,
        ColorConfig::LightBlue => RatatuiColor::LightBlue,
        ColorConfig::LightMagenta => RatatuiColor::LightMagenta,
        ColorConfig::LightCyan => RatatuiColor::LightCyan,
        ColorConfig::White => RatatuiColor::White,
    }
}

/// Convert ModifierConfig to ratatui Modifier
pub fn modifier_config_to_ratatui(config: &ModifierConfig) -> Modifier {
    match config {
        ModifierConfig::Bold => Modifier::BOLD,
        ModifierConfig::Dim => Modifier::DIM,
        ModifierConfig::Italic => Modifier::ITALIC,
        ModifierConfig::Underlined => Modifier::UNDERLINED,
        ModifierConfig::SlowBlink => Modifier::SLOW_BLINK,
        ModifierConfig::RapidBlink => Modifier::RAPID_BLINK,
        ModifierConfig::Reversed => Modifier::REVERSED,
        ModifierConfig::Hidden => Modifier::HIDDEN,
        ModifierConfig::CrossedOut => Modifier::CROSSED_OUT,
    }
}

/// Convert ratatui Style to StyleConfig
pub fn ratatui_style_to_config(style: RatatuiStyle) -> StyleConfig {
    let fg = style.fg.map(ratatui_color_to_config);
    let bg = style.bg.map(ratatui_color_to_config);

    let mut add_modifier = Vec::new();
    if style.add_modifier.contains(Modifier::BOLD) {
        add_modifier.push(ModifierConfig::Bold);
    }
    if style.add_modifier.contains(Modifier::DIM) {
        add_modifier.push(ModifierConfig::Dim);
    }
    if style.add_modifier.contains(Modifier::ITALIC) {
        add_modifier.push(ModifierConfig::Italic);
    }
    if style.add_modifier.contains(Modifier::UNDERLINED) {
        add_modifier.push(ModifierConfig::Underlined);
    }
    if style.add_modifier.contains(Modifier::SLOW_BLINK) {
        add_modifier.push(ModifierConfig::SlowBlink);
    }
    if style.add_modifier.contains(Modifier::RAPID_BLINK) {
        add_modifier.push(ModifierConfig::RapidBlink);
    }
    if style.add_modifier.contains(Modifier::REVERSED) {
        add_modifier.push(ModifierConfig::Reversed);
    }
    if style.add_modifier.contains(Modifier::HIDDEN) {
        add_modifier.push(ModifierConfig::Hidden);
    }
    if style.add_modifier.contains(Modifier::CROSSED_OUT) {
        add_modifier.push(ModifierConfig::CrossedOut);
    }

    StyleConfig {
        fg,
        bg,
        add_modifier,
        sub_modifier: Vec::new(),
    }
}

/// Convert ratatui Color to ColorConfig
pub fn ratatui_color_to_config(color: RatatuiColor) -> ColorConfig {
    match color {
        RatatuiColor::Rgb(r, g, b) => ColorConfig::Rgb { r, g, b },
        RatatuiColor::Indexed(value) => ColorConfig::Indexed { value },
        RatatuiColor::Reset => ColorConfig::Reset,
        RatatuiColor::Black => ColorConfig::Black,
        RatatuiColor::Red => ColorConfig::Red,
        RatatuiColor::Green => ColorConfig::Green,
        RatatuiColor::Yellow => ColorConfig::Yellow,
        RatatuiColor::Blue => ColorConfig::Blue,
        RatatuiColor::Magenta => ColorConfig::Magenta,
        RatatuiColor::Cyan => ColorConfig::Cyan,
        RatatuiColor::Gray => ColorConfig::Gray,
        RatatuiColor::DarkGray => ColorConfig::DarkGray,
        RatatuiColor::LightRed => ColorConfig::LightRed,
        RatatuiColor::LightGreen => ColorConfig::LightGreen,
        RatatuiColor::LightYellow => ColorConfig::LightYellow,
        RatatuiColor::LightBlue => ColorConfig::LightBlue,
        RatatuiColor::LightMagenta => ColorConfig::LightMagenta,
        RatatuiColor::LightCyan => ColorConfig::LightCyan,
        RatatuiColor::White => ColorConfig::White,
    }
}

// =============================================================================
// tui-vfx-types ↔ ratatui conversions
// =============================================================================

/// Convert ratatui Style to tui-vfx Style
pub fn ratatui_style_to_vfx(style: RatatuiStyle) -> VfxStyle {
    VfxStyle {
        fg: style
            .fg
            .map(ratatui_color_to_vfx)
            .unwrap_or(VfxColor::TRANSPARENT),
        bg: style
            .bg
            .map(ratatui_color_to_vfx)
            .unwrap_or(VfxColor::TRANSPARENT),
        mods: ratatui_modifiers_to_vfx(style.add_modifier),
    }
}

/// Convert tui-vfx Style to ratatui Style
pub fn vfx_style_to_ratatui(style: VfxStyle) -> RatatuiStyle {
    let mut rs = RatatuiStyle::default();
    // Only set fg/bg if not transparent
    if style.fg.a > 0 {
        rs = rs.fg(vfx_color_to_ratatui(style.fg));
    }
    if style.bg.a > 0 {
        rs = rs.bg(vfx_color_to_ratatui(style.bg));
    }
    rs = rs.add_modifier(vfx_modifiers_to_ratatui(style.mods));
    rs
}

/// Convert ratatui Color to tui-vfx Color
///
/// Note: Indexed colors are approximated using standard 16-color palette.
/// Reset becomes transparent.
pub fn ratatui_color_to_vfx(color: RatatuiColor) -> VfxColor {
    match color {
        RatatuiColor::Rgb(r, g, b) => VfxColor::rgb(r, g, b),
        RatatuiColor::Reset => VfxColor::TRANSPARENT,
        RatatuiColor::Black => VfxColor::rgb(0, 0, 0),
        RatatuiColor::Red => VfxColor::rgb(205, 0, 0),
        RatatuiColor::Green => VfxColor::rgb(0, 205, 0),
        RatatuiColor::Yellow => VfxColor::rgb(205, 205, 0),
        RatatuiColor::Blue => VfxColor::rgb(0, 0, 238),
        RatatuiColor::Magenta => VfxColor::rgb(205, 0, 205),
        RatatuiColor::Cyan => VfxColor::rgb(0, 205, 205),
        RatatuiColor::Gray => VfxColor::rgb(229, 229, 229),
        RatatuiColor::DarkGray => VfxColor::rgb(127, 127, 127),
        RatatuiColor::LightRed => VfxColor::rgb(255, 0, 0),
        RatatuiColor::LightGreen => VfxColor::rgb(0, 255, 0),
        RatatuiColor::LightYellow => VfxColor::rgb(255, 255, 0),
        RatatuiColor::LightBlue => VfxColor::rgb(92, 92, 255),
        RatatuiColor::LightMagenta => VfxColor::rgb(255, 0, 255),
        RatatuiColor::LightCyan => VfxColor::rgb(0, 255, 255),
        RatatuiColor::White => VfxColor::rgb(255, 255, 255),
        RatatuiColor::Indexed(i) => indexed_color_to_rgb(i),
    }
}

/// Convert tui-vfx Color to ratatui Color
pub fn vfx_color_to_ratatui(color: VfxColor) -> RatatuiColor {
    if color.a == 0 {
        RatatuiColor::Reset
    } else {
        RatatuiColor::Rgb(color.r, color.g, color.b)
    }
}

/// Convert ratatui Modifier to tui-vfx Modifiers
pub fn ratatui_modifiers_to_vfx(mods: Modifier) -> Modifiers {
    Modifiers {
        bold: mods.contains(Modifier::BOLD),
        dim: mods.contains(Modifier::DIM),
        italic: mods.contains(Modifier::ITALIC),
        underline: mods.contains(Modifier::UNDERLINED),
        slow_blink: mods.contains(Modifier::SLOW_BLINK),
        rapid_blink: mods.contains(Modifier::RAPID_BLINK),
        reverse: mods.contains(Modifier::REVERSED),
        hidden: mods.contains(Modifier::HIDDEN),
        strikethrough: mods.contains(Modifier::CROSSED_OUT),
    }
}

/// Convert tui-vfx Modifiers to ratatui Modifier
pub fn vfx_modifiers_to_ratatui(mods: Modifiers) -> Modifier {
    let mut m = Modifier::empty();
    if mods.bold {
        m |= Modifier::BOLD;
    }
    if mods.dim {
        m |= Modifier::DIM;
    }
    if mods.italic {
        m |= Modifier::ITALIC;
    }
    if mods.underline {
        m |= Modifier::UNDERLINED;
    }
    if mods.slow_blink {
        m |= Modifier::SLOW_BLINK;
    }
    if mods.rapid_blink {
        m |= Modifier::RAPID_BLINK;
    }
    if mods.reverse {
        m |= Modifier::REVERSED;
    }
    if mods.hidden {
        m |= Modifier::HIDDEN;
    }
    if mods.strikethrough {
        m |= Modifier::CROSSED_OUT;
    }
    m
}

/// Convert ratatui Rect to tui-vfx Rect
pub fn ratatui_rect_to_vfx(rect: RatatuiRect) -> VfxRect {
    VfxRect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

/// Convert tui-vfx Rect to ratatui Rect
pub fn vfx_rect_to_ratatui(rect: VfxRect) -> RatatuiRect {
    RatatuiRect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

// =============================================================================
// Cell conversions (for Grid adapter)
// =============================================================================

/// Convert ratatui Cell to tui-vfx Cell.
///
/// Extracts the first character from the cell's symbol (grapheme).
/// Multi-character graphemes are truncated to the first char.
pub fn ratatui_cell_to_vfx(cell: &RatatuiCell) -> VfxCell {
    let ch = cell.symbol().chars().next().unwrap_or(' ');
    VfxCell {
        ch,
        fg: ratatui_color_to_vfx(cell.fg),
        bg: ratatui_color_to_vfx(cell.bg),
        mods: ratatui_modifiers_to_vfx(cell.modifier),
        mod_alpha: None,
    }
}

/// Convert tui-vfx Cell to ratatui Cell.
///
/// Creates a new ratatui Cell with the character and styling from the vfx Cell.
pub fn vfx_cell_to_ratatui(cell: VfxCell) -> RatatuiCell {
    let mut ratatui_cell = RatatuiCell::default();
    ratatui_cell.set_char(cell.ch);

    // Build style with colors and modifiers
    let mut style = RatatuiStyle::default();
    if cell.fg.a > 0 {
        style = style.fg(vfx_color_to_ratatui(cell.fg));
    }
    if cell.bg.a > 0 {
        style = style.bg(vfx_color_to_ratatui(cell.bg));
    }
    let mods = vfx_modifiers_to_ratatui(cell.mods);
    if !mods.is_empty() {
        style = style.add_modifier(mods);
    }
    ratatui_cell.set_style(style);

    ratatui_cell
}

/// Apply a vfx Cell's styling to a ratatui Cell in-place.
///
/// This is more efficient than converting and copying when modifying an existing buffer.
pub fn apply_vfx_cell_to_ratatui(vfx_cell: VfxCell, ratatui_cell: &mut RatatuiCell) {
    ratatui_cell.set_char(vfx_cell.ch);

    // Build style with colors and modifiers
    let mut style = RatatuiStyle::default();
    if vfx_cell.fg.a > 0 {
        style = style.fg(vfx_color_to_ratatui(vfx_cell.fg));
    }
    if vfx_cell.bg.a > 0 {
        style = style.bg(vfx_color_to_ratatui(vfx_cell.bg));
    }
    let mods = vfx_modifiers_to_ratatui(vfx_cell.mods);
    if !mods.is_empty() {
        style = style.add_modifier(mods);
    }
    ratatui_cell.set_style(style);
}

// =============================================================================
// Helper functions
// =============================================================================

/// Convert 256-color indexed palette to RGB approximation.
///
/// Uses standard xterm-256 palette approximations.
fn indexed_color_to_rgb(index: u8) -> VfxColor {
    match index {
        // Standard colors (0-15)
        0 => VfxColor::rgb(0, 0, 0),
        1 => VfxColor::rgb(128, 0, 0),
        2 => VfxColor::rgb(0, 128, 0),
        3 => VfxColor::rgb(128, 128, 0),
        4 => VfxColor::rgb(0, 0, 128),
        5 => VfxColor::rgb(128, 0, 128),
        6 => VfxColor::rgb(0, 128, 128),
        7 => VfxColor::rgb(192, 192, 192),
        8 => VfxColor::rgb(128, 128, 128),
        9 => VfxColor::rgb(255, 0, 0),
        10 => VfxColor::rgb(0, 255, 0),
        11 => VfxColor::rgb(255, 255, 0),
        12 => VfxColor::rgb(0, 0, 255),
        13 => VfxColor::rgb(255, 0, 255),
        14 => VfxColor::rgb(0, 255, 255),
        15 => VfxColor::rgb(255, 255, 255),
        // 216-color cube (16-231)
        16..=231 => {
            let n = index - 16;
            let r = (n / 36) % 6;
            let g = (n / 6) % 6;
            let b = n % 6;
            let to_rgb = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            VfxColor::rgb(to_rgb(r), to_rgb(g), to_rgb(b))
        }
        // Grayscale (232-255)
        232..=255 => {
            let gray = 8 + (index - 232) * 10;
            VfxColor::rgb(gray, gray, gray)
        }
    }
}

// =============================================================================
// AnimationPhase ↔ mixed_signals::Phase conversions
// =============================================================================

use crate::state::AnimationPhase;
use mixed_signals::traits::Phase as MixedPhase;

/// Convert AnimationPhase to mixed_signals Phase
pub fn animation_phase_to_mixed(phase: AnimationPhase) -> MixedPhase {
    match phase {
        AnimationPhase::Entering => MixedPhase::Start,
        AnimationPhase::Dwelling => MixedPhase::Active,
        AnimationPhase::Exiting => MixedPhase::End,
        AnimationPhase::Finished => MixedPhase::Done,
    }
}

/// Convert mixed_signals Phase to AnimationPhase
pub fn mixed_phase_to_animation(phase: MixedPhase) -> AnimationPhase {
    match phase {
        MixedPhase::Start => AnimationPhase::Entering,
        MixedPhase::Active => AnimationPhase::Dwelling,
        MixedPhase::End => AnimationPhase::Exiting,
        MixedPhase::Done => AnimationPhase::Finished,
        MixedPhase::Custom(_) => AnimationPhase::Dwelling, // Map custom to dwelling as fallback
    }
}

// <FILE>src/compat.rs</FILE> - <DESC>Compatibility conversions between ratatui and tui-vfx types</DESC>
// <VERS>END OF VERSION: 0.4.3</VERS>
