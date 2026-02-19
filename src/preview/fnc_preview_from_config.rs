// <FILE>src/preview/fnc_preview_from_config.rs</FILE> - <DESC>Build PreviewItem from V2 config</DESC>
// <VERS>VERSION: 2.2.0</VERS>
// <WCTX>Custom frame content support</WCTX>
// <CLOG>Added conversion of RaFrameContent to FrameContent for direct-rendered borders</CLOG>

use super::cls_preview_item::PreviewItem;
use crate::recipe_schema::{
    RaApplyTo, RaBorderType, RaLayoutMode, RaRecipeConfig, RaStyleEffect, RaTitleAlignment,
    RaTitlePosition,
};
use crate::theme::{
    AppearanceConfig, BorderTypeConfig, BordersConfig, ChromeConfig, CustomBorderSet, FadeConfig,
    FrameContent, PaddingConfig, TextConfig, TitleAlignment, TitleConfig, TitlePosition,
};
use serde_json::Value;
use tui_vfx_compositor::types::{FilterSpec, MaskSpec, SamplerSpec};
use tui_vfx_style::models::{FadeApplyTo, FadeDirection, FadeToBlack, ModifierConfig, StyleConfig};

/// Convert V2 modifier strings to ModifierConfig.
/// V2 uses strings like "bold", "italic", etc.
fn string_to_modifier(s: &str) -> Option<ModifierConfig> {
    match s.to_lowercase().as_str() {
        "bold" => Some(ModifierConfig::Bold),
        "dim" => Some(ModifierConfig::Dim),
        "italic" => Some(ModifierConfig::Italic),
        "underlined" | "underline" => Some(ModifierConfig::Underlined),
        "slow_blink" | "slowblink" => Some(ModifierConfig::SlowBlink),
        "rapid_blink" | "rapidblink" => Some(ModifierConfig::RapidBlink),
        "reversed" | "reverse" => Some(ModifierConfig::Reversed),
        "hidden" => Some(ModifierConfig::Hidden),
        "crossed_out" | "crossedout" | "strikethrough" => Some(ModifierConfig::CrossedOut),
        _ => None,
    }
}

/// Convert V2 modifier string vec to ModifierConfig vec.
fn convert_modifiers(strings: &[String]) -> Vec<ModifierConfig> {
    strings
        .iter()
        .filter_map(|s| string_to_modifier(s))
        .collect()
}

/// Convert RaApplyTo to FadeApplyTo.
fn convert_apply_to(v2: &RaApplyTo) -> FadeApplyTo {
    match v2 {
        RaApplyTo::Both => FadeApplyTo::Both,
        RaApplyTo::Foreground => FadeApplyTo::Foreground,
        RaApplyTo::Background => FadeApplyTo::Background,
    }
}

/// Convert RaStyleEffect to FadeToBlack if it's a fade effect.
fn convert_fade_effect(
    effect: Option<&RaStyleEffect>,
    direction: FadeDirection,
) -> Option<FadeToBlack> {
    match effect {
        Some(RaStyleEffect::FadeIn {
            apply_to, easing, ..
        }) => Some(FadeToBlack {
            direction,
            apply_to: convert_apply_to(apply_to),
            ease: *easing,
        }),
        Some(RaStyleEffect::FadeOut {
            apply_to, easing, ..
        }) => Some(FadeToBlack {
            direction,
            apply_to: convert_apply_to(apply_to),
            ease: *easing,
        }),
        _ => None,
    }
}

/// Build a PreviewItem from a RaRecipeConfig.
///
/// This converts the V2 configuration format into a PreviewItem
/// that can be rendered using the standard playback pipeline.
pub fn preview_from_recipe_config(config: &RaRecipeConfig) -> PreviewItem {
    let profile = config.to_animation_profile();
    let animation = config.animation_type();
    let auto_dismiss = config.auto_dismiss();

    let mut item = PreviewItem::new(config.message.clone())
        .anchor(config.layout.anchor.position())
        .offset_h_percent(config.layout.anchor.offset_horizontal_percent())
        .offset_v_percent(config.layout.anchor.offset_vertical_percent())
        .offset_h_cells(config.layout.anchor.offset_horizontal_cells())
        .offset_v_cells(config.layout.anchor.offset_vertical_cells())
        .offset_h_pixels(config.layout.anchor.offset_horizontal_pixels())
        .offset_v_pixels(config.layout.anchor.offset_vertical_pixels())
        .size(config.layout.width, config.layout.height)
        .fullscreen(matches!(config.layout.mode, RaLayoutMode::Fullscreen))
        .center_content(config.border.center_content)
        .wrap(config.layout.wrap)
        .animation(animation)
        .auto_dismiss(auto_dismiss)
        .profile(profile);

    // Apply content effect if present
    if let Some(ref content) = config.content {
        if let Some(ref effect) = content.effect {
            item = item.content_effect(effect.clone());
        }
    }

    // Apply border trim
    item = item.slide_border_trim(config.border_trim_policy());

    // Build and apply AppearanceConfig from V2 style settings
    let appearance = build_appearance_from_v2(config);
    item = item.appearance(appearance);

    // Apply pipeline specs (enter, dwell, exit phases)
    // Apply masks (take first from Vec for preview - full multi-mask support in renderer)
    if let Some(mask) = config.pipeline.mask.enter.first() {
        if !matches!(mask, MaskSpec::None) {
            item = item.enter_mask(mask.clone());
        }
    }
    if let Some(mask) = config.pipeline.mask.dwell.first() {
        if !matches!(mask, MaskSpec::None) {
            item = item.dwell_mask(mask.clone());
        }
    }
    if let Some(mask) = config.pipeline.mask.exit.first() {
        if !matches!(mask, MaskSpec::None) {
            item = item.exit_mask(mask.clone());
        }
    }

    // Apply mask combine mode
    item = item.mask_combine_mode(config.pipeline.mask.combine_mode);

    if !matches!(config.pipeline.sampler.enter, SamplerSpec::None) {
        item = item.enter_sampler(config.pipeline.sampler.enter.clone());
    }
    if !matches!(config.pipeline.sampler.dwell, SamplerSpec::None) {
        item = item.dwell_sampler(config.pipeline.sampler.dwell.clone());
    }
    if !matches!(config.pipeline.sampler.exit, SamplerSpec::None) {
        item = item.exit_sampler(config.pipeline.sampler.exit.clone());
    }

    // Apply filters (take first from Vec for preview - full multi-filter support in renderer)
    if let Some(filter) = config.pipeline.filter.enter.first() {
        if !matches!(filter, FilterSpec::None) {
            item = item.enter_filter(filter.clone());
        }
    }
    if let Some(filter) = config.pipeline.filter.dwell.first() {
        if !matches!(filter, FilterSpec::None) {
            item = item.dwell_filter(filter.clone());
        }
    }
    if let Some(filter) = config.pipeline.filter.exit.first() {
        if !matches!(filter, FilterSpec::None) {
            item = item.exit_filter(filter.clone());
        }
    }

    // Apply phase-specific style effects from first style layer
    // (For multi-layer support, the render pipeline handles all layers via AnimationProfile)
    if let Some(first_style) = config.pipeline.styles.first() {
        if let Some(effect) = &first_style.enter_effect {
            item = item.enter_style(effect.to_style_effect());
        }
        if let Some(effect) = &first_style.dwell_effect {
            item = item.dwell_style(effect.to_style_effect());
        }
        if let Some(effect) = &first_style.exit_effect {
            item = item.exit_style(effect.to_style_effect());
        }
    }

    // Apply custom frame content if present
    // Frame content is drawn directly to buffer (bypasses Block widget)
    // enabling multi-char patterns and full effect support on borders
    if let Some(ref v2_frame) = config.border.frame {
        let frame = FrameContent {
            top_left: v2_frame.top_left.clone(),
            top_right: v2_frame.top_right.clone(),
            bottom_left: v2_frame.bottom_left.clone(),
            bottom_right: v2_frame.bottom_right.clone(),
            top: v2_frame.top.clone(),
            bottom: v2_frame.bottom.clone(),
            left: v2_frame.left.clone(),
            right: v2_frame.right.clone(),
        };
        item = item.frame(frame);
    }

    item
}

/// Build AppearanceConfig from RaRecipeConfig style settings.
fn build_appearance_from_v2(config: &RaRecipeConfig) -> AppearanceConfig {
    // Use first style layer for base appearance, with sensible defaults
    let default_base = crate::recipe_schema::config::RaBaseStyle::default();
    let base_style = config
        .pipeline
        .styles
        .first()
        .map(|s| &s.base_style)
        .unwrap_or(&default_base);

    // Convert V2 modifier strings to ModifierConfig
    let add_mods = convert_modifiers(&base_style.added_modifiers);
    let sub_mods = convert_modifiers(&base_style.removed_modifiers);

    // Build StyleConfig from RaBaseStyle for text/content
    let text_style_config = StyleConfig {
        fg: base_style.foreground,
        bg: base_style.background,
        add_modifier: add_mods.clone(),
        sub_modifier: sub_mods.clone(),
    };

    // Frame style uses the same fg for border, bg for background
    let frame_style_config = StyleConfig {
        fg: None, // Frame fg not typically used
        bg: base_style.background,
        add_modifier: vec![],
        sub_modifier: vec![],
    };

    // Border style uses fg color for the border lines
    let border_style_config = StyleConfig {
        fg: base_style.foreground,
        bg: None,               // Border bg not typically used
        add_modifier: add_mods, // Apply modifiers to border too
        sub_modifier: sub_mods,
    };

    // Build title config if present in V2 border config
    let title = config.border.title.as_ref().map(|text| TitleConfig {
        text: text.clone(),
        position: match config.border.title_position {
            RaTitlePosition::Top => TitlePosition::Top,
            RaTitlePosition::Bottom => TitlePosition::Bottom,
            RaTitlePosition::Left => TitlePosition::Left,
            RaTitlePosition::Right => TitlePosition::Right,
        },
        alignment: match config.border.title_alignment {
            RaTitleAlignment::Left => TitleAlignment::Left,
            RaTitleAlignment::Center => TitleAlignment::Center,
            RaTitleAlignment::Right => TitleAlignment::Right,
        },
    });

    // Map RaBorderType to borders and border_type
    let (borders, border_type) = match config.border.border_type {
        RaBorderType::None => (
            Some(BordersConfig {
                top: false,
                right: false,
                bottom: false,
                left: false,
            }),
            Some(BorderTypeConfig::Plain), // Type doesn't matter when no borders
        ),
        RaBorderType::Rounded => (None, Some(BorderTypeConfig::Rounded)),
        RaBorderType::Plain => (None, Some(BorderTypeConfig::Plain)),
        RaBorderType::Double => (None, Some(BorderTypeConfig::Double)),
        RaBorderType::Thick => (None, Some(BorderTypeConfig::Thick)),
    };

    // Map V2 padding if non-zero
    let padding = {
        let p = &config.border.padding;
        if p.top == 0 && p.right == 0 && p.bottom == 0 && p.left == 0 {
            None
        } else {
            Some(PaddingConfig {
                top: p.top,
                right: p.right,
                bottom: p.bottom,
                left: p.left,
            })
        }
    };

    // Convert custom border chars if present
    let custom_border_set = config
        .border
        .custom_chars
        .as_ref()
        .map(|chars| CustomBorderSet {
            top_left: chars.top_left.clone(),
            top_right: chars.top_right.clone(),
            bottom_left: chars.bottom_left.clone(),
            bottom_right: chars.bottom_right.clone(),
            horizontal_top: chars.horizontal_top.clone(),
            horizontal_bottom: chars.horizontal_bottom.clone(),
            vertical_left: chars.vertical_left.clone(),
            vertical_right: chars.vertical_right.clone(),
        });

    // Chrome config with frame style, border style, and title
    let chrome = ChromeConfig {
        borders,
        border_type,
        border_set: None,
        custom_border_set,
        padding,
        frame_style: Some(frame_style_config),
        border_style: Some(border_style_config),
        title,
    };

    // Text config with the text style
    let text = TextConfig {
        style: Some(text_style_config),
    };

    // Build fade config from V2 style effects (first layer)
    let first_style = config.pipeline.styles.first();
    let fade_enter = convert_fade_effect(
        first_style.and_then(|s| s.enter_effect.as_ref()),
        FadeDirection::In,
    );
    let fade_exit = convert_fade_effect(
        first_style.and_then(|s| s.exit_effect.as_ref()),
        FadeDirection::Out,
    );
    let fade = if fade_enter.is_some() || fade_exit.is_some() {
        Some(FadeConfig {
            enter: fade_enter,
            exit: fade_exit,
        })
    } else {
        None
    };

    AppearanceConfig {
        chrome: Some(chrome),
        text: Some(text),
        fade,
    }
}

/// Build a PreviewItem from a recipe ID and config JSON.
///
/// Attempts to parse the config as RaRecipeConfig and build a preview.
/// Returns None if parsing fails.
pub fn preview_for_recipe_id(_recipe_id: &str, cfg: &Value) -> Option<PreviewItem> {
    // Try to parse as V2 config
    serde_json::from_value::<RaRecipeConfig>(cfg.clone())
        .ok()
        .map(|v2_config| preview_from_recipe_config(&v2_config))
}

// <FILE>src/preview/fnc_preview_from_config.rs</FILE> - <DESC>Build PreviewItem from V2 config</DESC>
// <VERS>END OF VERSION: 1.10.0</VERS>
