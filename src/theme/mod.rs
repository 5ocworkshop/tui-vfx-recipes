// <FILE>src/theme/mod.rs</FILE> - <DESC>Framework theme + appearance contract</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Custom border character support</WCTX>
// <CLOG>Export CustomBorderSet</CLOG>

pub mod fnc_resolve_effective_appearance;
pub mod types;

pub use fnc_resolve_effective_appearance::resolve_effective_appearance;
pub use types::{
    AppearanceConfig, BorderSetConfig, BorderTypeConfig, BordersConfig, ChromeConfig,
    CustomBorderSet, FadeConfig, FrameContent, HasAppearance, PaddingConfig, ResolvedAppearance,
    TextConfig, Theme, TitleAlignment, TitleConfig, TitlePosition,
};

// <FILE>src/theme/mod.rs</FILE> - <DESC>Framework theme + appearance contract</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
