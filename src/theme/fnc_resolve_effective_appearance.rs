// <FILE>src/theme/fnc_resolve_effective_appearance.rs</FILE> - <DESC>Resolve theme defaults + item overrides</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Custom border character support</WCTX>
// <CLOG>Added custom_border_set resolution - overrides border_set when present</CLOG>

use crate::compat::{ratatui_style_to_config, style_config_to_ratatui};
use crate::state::AnimationPhase;
use ratatui::style::Style;
use tui_vfx_style::models::{FadeDirection, FadeToBlack, StyleConfig};

use super::types::{
    AppearanceConfig, BorderSetConfig, BorderTypeConfig, BordersConfig, CustomBorderSet,
    FadeConfig, HasAppearance, PaddingConfig, ResolvedAppearance, Theme, TitleConfig,
};
use ratatui::symbols::border;

pub fn resolve_effective_appearance(
    theme: &Theme,
    item: &impl HasAppearance,
    phase: AnimationPhase,
) -> ResolvedAppearance {
    resolve_effective_appearance_opt(theme, item.appearance(), phase)
}

pub(crate) fn resolve_effective_appearance_opt(
    theme: &Theme,
    appearance: Option<&AppearanceConfig>,
    phase: AnimationPhase,
) -> ResolvedAppearance {
    let theme_app = &theme.defaults;

    // Check if appearance explicitly sets chrome to None (meaning "no chrome wanted")
    // vs appearance being None (meaning "use theme defaults")
    let chrome_explicitly_disabled = appearance.map(|a| a.chrome.is_none()).unwrap_or(false);

    let (
        borders,
        border_type,
        border_set,
        custom_border_set,
        padding,
        frame_style_cfg,
        border_style_cfg,
    ) = if chrome_explicitly_disabled {
        // User explicitly set chrome: None - use no borders, default everything else
        (
            BordersConfig {
                top: false,
                right: false,
                bottom: false,
                left: false,
            },
            BorderTypeConfig::Plain,
            BorderSetConfig::Plain,
            None,
            PaddingConfig::zero(),
            ratatui_style_to_config(Style::default()),
            ratatui_style_to_config(Style::default()),
        )
    } else {
        // Normal resolution: overlay appearance chrome over theme chrome
        let borders = appearance
            .and_then(|a| a.chrome.as_ref())
            .and_then(|c| c.borders)
            .or_else(|| theme_app.chrome.as_ref().and_then(|c| c.borders))
            .unwrap_or_else(BordersConfig::all);

        let border_type = pick_field_copy(
            theme_app.chrome.as_ref(),
            appearance.and_then(|a| a.chrome.as_ref()),
            |c| c.border_type,
        )
        .unwrap_or(BorderTypeConfig::Plain);

        let border_set = pick_field_copy(
            theme_app.chrome.as_ref(),
            appearance.and_then(|a| a.chrome.as_ref()),
            |c| c.border_set,
        )
        .unwrap_or(BorderSetConfig::Plain);

        // Custom border set: overlay wins, then theme
        let custom_border_set: Option<CustomBorderSet> = appearance
            .and_then(|a| a.chrome.as_ref())
            .and_then(|c| c.custom_border_set.clone())
            .or_else(|| {
                theme_app
                    .chrome
                    .as_ref()
                    .and_then(|c| c.custom_border_set.clone())
            });

        let padding = pick_field_copy(
            theme_app.chrome.as_ref(),
            appearance.and_then(|a| a.chrome.as_ref()),
            |c| c.padding,
        )
        .unwrap_or_else(PaddingConfig::zero);

        let frame_style_cfg = pick_style(
            theme_app.chrome.as_ref(),
            appearance.and_then(|a| a.chrome.as_ref()),
            |c| c.frame_style.as_ref(),
        );

        let border_style_cfg = pick_style(
            theme_app.chrome.as_ref(),
            appearance.and_then(|a| a.chrome.as_ref()),
            |c| c.border_style.as_ref(),
        );

        (
            borders,
            border_type,
            border_set,
            custom_border_set,
            padding,
            frame_style_cfg,
            border_style_cfg,
        )
    };

    let text_style_cfg = pick_style(
        theme_app.text.as_ref(),
        appearance.and_then(|a| a.text.as_ref()),
        |t| t.style.as_ref(),
    );

    let fade = pick(
        theme_app.fade.as_ref(),
        appearance.and_then(|a| a.fade.as_ref()),
    );
    let (fade_enter, fade_exit) = resolve_fade(fade, phase);

    // Resolve title: overlay wins, or fall back to theme default
    let title: Option<TitleConfig> = appearance
        .and_then(|a| a.chrome.as_ref())
        .and_then(|c| c.title.clone())
        .or_else(|| theme_app.chrome.as_ref().and_then(|c| c.title.clone()));

    // Use custom border set if present, otherwise use the preset border_set
    let resolved_border_set: border::Set<'static> = custom_border_set
        .map(|cbs| cbs.to_border_set())
        .unwrap_or_else(|| border_set.into());

    ResolvedAppearance {
        frame_style: style_config_to_ratatui(&frame_style_cfg),
        border_style: style_config_to_ratatui(&border_style_cfg),
        text_style: style_config_to_ratatui(&text_style_cfg),
        borders: borders.into(),
        border_type: border_type.into(),
        border_set: resolved_border_set,
        padding: padding.into(),
        fade_enter,
        fade_exit,
        title,
    }
    .with_forced_fade_direction()
}

fn pick<'a, T>(base: Option<&'a T>, overlay: Option<&'a T>) -> Option<&'a T> {
    overlay.or(base)
}

fn pick_field_copy<Root, Field: Copy>(
    base: Option<&Root>,
    overlay: Option<&Root>,
    getter: impl Fn(&Root) -> Option<Field>,
) -> Option<Field> {
    overlay.and_then(&getter).or_else(|| base.and_then(getter))
}

fn pick_style<Root>(
    base: Option<&Root>,
    overlay: Option<&Root>,
    getter: impl Fn(&Root) -> Option<&StyleConfig>,
) -> StyleConfig {
    overlay
        .and_then(|r| getter(r).cloned())
        .or_else(|| base.and_then(|r| getter(r).cloned()))
        .unwrap_or_else(|| ratatui_style_to_config(Style::default()))
}

fn resolve_fade(
    fade: Option<&FadeConfig>,
    phase: AnimationPhase,
) -> (Option<FadeToBlack>, Option<FadeToBlack>) {
    let enter = fade.and_then(|f| f.enter);
    let exit = fade.and_then(|f| f.exit);

    match phase {
        AnimationPhase::Entering => (enter.map(force_in), None),
        AnimationPhase::Exiting => (None, exit.map(force_out)),
        _ => (None, None),
    }
}

fn force_in(mut fade: FadeToBlack) -> FadeToBlack {
    fade.direction = FadeDirection::In;
    fade
}

fn force_out(mut fade: FadeToBlack) -> FadeToBlack {
    fade.direction = FadeDirection::Out;
    fade
}

// <FILE>src/theme/fnc_resolve_effective_appearance.rs</FILE> - <DESC>Resolve theme defaults + item overrides</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
